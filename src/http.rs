//! Thin `reqwest`-based HTTP layer used by [`crate::TripoClient`].
//!
//! Responsibilities:
//!   - Attach `Authorization: Bearer …` header.
//!   - Retry idempotent requests on transient errors with exponential
//!     back-off (honoring `Retry-After` when present).
//!   - Parse the standard `{ code, data, message, suggestion }` envelope and
//!     surface [`crate::Error::Api`] / [`crate::Error::Request`] as needed.

use crate::error::{Error, Result};
use crate::models::Envelope;
use rand::Rng;
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;

const DEFAULT_RETRY_STATUSES: &[u16] = &[408, 425, 429, 500, 502, 503, 504];

#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout: Duration,
    pub retries: u32,
    pub user_agent: String,
}

#[derive(Clone)]
pub(crate) struct HttpClient {
    inner: reqwest::Client,
    config: HttpConfig,
}

/// Options for a single request, layered on top of the client defaults.
#[derive(Default)]
pub(crate) struct RequestOptions<'a> {
    pub json: Option<&'a serde_json::Value>,
    pub retries: Option<u32>,
}

impl HttpClient {
    pub fn new(config: HttpConfig) -> Result<Self> {
        let inner = reqwest::Client::builder()
            .user_agent(config.user_agent.clone())
            .build()
            .map_err(|e| Error::Request {
                message: format!("failed to build HTTP client: {e}"),
                status: None,
                body: None,
                source: Some(e),
            })?;
        Ok(Self { inner, config })
    }

    fn url(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!(
                "{}/{}",
                self.config.base_url.trim_end_matches('/'),
                path.trim_start_matches('/')
            )
        }
    }

    /// Perform a request and deserialize the `data` field of the standard
    /// envelope into `T`.
    pub async fn request_json<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        opts: RequestOptions<'_>,
    ) -> Result<T> {
        let response = self.execute(method, path, opts).await?;
        let status = response.status().as_u16();
        let bytes = response.bytes().await.map_err(|e| Error::Request {
            message: format!("failed to read response body: {e}"),
            status: Some(status),
            body: None,
            source: Some(e),
        })?;
        parse_envelope(&bytes, Some(status))
    }

    /// Perform a `multipart/form-data` request (used by file upload).
    pub async fn request_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T> {
        let url = self.url(path);
        let response = self
            .inner
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .multipart(form)
            .timeout(self.config.timeout)
            .send()
            .await
            .map_err(|e| Error::Request {
                message: format!("network error: {e}"),
                status: None,
                body: None,
                source: Some(e),
            })?;
        let status = response.status().as_u16();
        let bytes = response.bytes().await.map_err(|e| Error::Request {
            message: format!("failed to read response body: {e}"),
            status: Some(status),
            body: None,
            source: Some(e),
        })?;
        parse_envelope(&bytes, Some(status))
    }

    /// Fetch a raw resource (e.g. downloading a model URL) without envelope
    /// parsing. Returns the raw bytes and the `Content-Type` header.
    pub async fn get_raw(&self, url: &str) -> Result<(Vec<u8>, Option<String>)> {
        let response = self
            .inner
            .get(url)
            .timeout(self.config.timeout)
            .send()
            .await
            .map_err(|e| Error::Request {
                message: format!("network error: {e}"),
                status: None,
                body: None,
                source: Some(e),
            })?;
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.ok();
            return Err(Error::Request {
                message: format!("download failed with HTTP {status}"),
                status: Some(status),
                body,
                source: None,
            });
        }
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let bytes = response.bytes().await.map_err(|e| Error::Request {
            message: format!("failed to read response body: {e}"),
            status: None,
            body: None,
            source: Some(e),
        })?;
        Ok((bytes.to_vec(), content_type))
    }

    async fn execute(
        &self,
        method: Method,
        path: &str,
        opts: RequestOptions<'_>,
    ) -> Result<reqwest::Response> {
        let url = self.url(path);
        let total_attempts = opts.retries.unwrap_or(self.config.retries) + 1;

        let mut attempt = 0u32;
        loop {
            attempt += 1;
            let mut builder = self
                .inner
                .request(method.clone(), &url)
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .timeout(self.config.timeout);
            if let Some(json) = opts.json {
                builder = builder.json(json);
            }

            let result = builder.send().await;
            match result {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        return Ok(response);
                    }
                    if is_retryable_status(status) && attempt < total_attempts {
                        let retry_after = response
                            .headers()
                            .get(reqwest::header::RETRY_AFTER)
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse::<f64>().ok());
                        tokio::time::sleep(backoff(attempt, retry_after)).await;
                        continue;
                    }
                    // Not retryable (or attempts exhausted): let the caller's
                    // envelope parser decide whether this is Error::Api or
                    // Error::Request based on the body shape.
                    return Ok(response);
                }
                Err(e) => {
                    if attempt < total_attempts && (e.is_timeout() || e.is_connect()) {
                        tokio::time::sleep(backoff(attempt, None)).await;
                        continue;
                    }
                    return Err(Error::Request {
                        message: format!("network error: {e}"),
                        status: None,
                        body: None,
                        source: Some(e),
                    });
                }
            }
        }
    }
}

fn is_retryable_status(status: StatusCode) -> bool {
    DEFAULT_RETRY_STATUSES.contains(&status.as_u16())
}

fn backoff(attempt: u32, retry_after_secs: Option<f64>) -> Duration {
    if let Some(secs) = retry_after_secs {
        if secs.is_finite() && secs >= 0.0 {
            return Duration::from_millis((secs * 1000.0).min(30_000.0) as u64);
        }
    }
    let base_ms = 1000u64
        .saturating_mul(1 << attempt.saturating_sub(1))
        .min(8000);
    let jitter_ms: u64 = rand::thread_rng().gen_range(0..250);
    Duration::from_millis(base_ms + jitter_ms)
}

/// Parse the standard `{ code, data, message, suggestion }` envelope,
/// returning `data` on success (`code == 0`) or [`Error::Api`] otherwise.
fn parse_envelope<T: DeserializeOwned>(bytes: &[u8], status: Option<u16>) -> Result<T> {
    if bytes.is_empty() {
        return Err(Error::Request {
            message: "empty response body".to_string(),
            status,
            body: None,
            source: None,
        });
    }
    let envelope: Envelope<T> = serde_json::from_slice(bytes).map_err(|e| {
        let raw = String::from_utf8_lossy(bytes).to_string();
        Error::Request {
            message: format!("malformed response: {e}"),
            status,
            body: Some(raw),
            source: None,
        }
    })?;
    if envelope.code != 0 {
        return Err(Error::Api {
            code: envelope.code,
            message: envelope.message,
            suggestion: envelope.suggestion,
            status,
        });
    }
    envelope.data.ok_or_else(|| Error::Request {
        message: "response envelope had code=0 but no `data` field".to_string(),
        status,
        body: None,
        source: None,
    })
}
