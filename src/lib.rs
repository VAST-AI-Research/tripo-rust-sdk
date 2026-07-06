//! `tripo3d-sdk` — an unofficial Rust SDK for the [Tripo3D v3 API](https://developers.tripo3d.com/en/docs/introduction).
//!
//! ```no_run
//! use tripo3d_sdk::{TripoClient, ClientOptions, params::TextToModelParams};
//!
//! # async fn run() -> tripo3d_sdk::Result<()> {
//! let client = TripoClient::new(ClientOptions::default())?; // reads TRIPO_API_KEY
//!
//! let task_id = client
//!     .text_to_model(TextToModelParams::new("a cute cat"))
//!     .await?;
//!
//! let task = client.wait_for_task(&task_id, Default::default()).await?;
//! println!("model url: {:?}", task.primary_model_url());
//! # Ok(())
//! # }
//! ```

mod client;
pub mod constants;
mod error;
pub mod models;
pub mod params;

mod http;

pub use client::{ClientOptions, DownloadedModel, TripoClient, WaitOptions};
pub use constants::{
    model_version, AnimOutFormat, Animation, OutputFormat, RigSpec, RigType, TaskStatus,
    TextureFormat, DEFAULT_BASE_URL,
};
pub use error::{Error, Result};
pub use models::{Balance, FileDescriptor, FileInput, ObjectRef, Task, TaskOutput, UploadedFile};
