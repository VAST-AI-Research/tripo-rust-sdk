//! examples/image_to_image.rs
//!
//! Apply a style/edit transformation to an existing image and save the
//! result to disk.
//!
//!   # local file
//!   $ cargo run --example image_to_image -- ./cat.png "turn it into a watercolor painting"
//!   # remote URL
//!   $ cargo run --example image_to_image -- https://example.com/cat.jpg "make it look like a pencil sketch"

use std::env;
use std::path::Path;
use tokio::fs;
use tripo3d_sdk::{
    constants::{image_model, TaskStatus},
    models::FileInput,
    params::ImageToImageParams,
    ClientOptions, TripoClient, WaitOptions,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or_else(|| {
        anyhow::anyhow!("Usage: cargo run --example image_to_image -- <local-file|url> [prompt]")
    })?;
    let prompt = args.collect::<Vec<_>>().join(" ");
    let prompt = if prompt.is_empty() {
        "turn it into a watercolor painting".to_string()
    } else {
        prompt
    };

    let client = TripoClient::new(ClientOptions::default())?;

    let file_input: FileInput = if input.starts_with("http://") || input.starts_with("https://") {
        FileInput::from(input.as_str())
    } else {
        let bytes = fs::read(&input).await?;
        let filename = Path::new(&input)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("upload.bin")
            .to_string();
        let content_type = guess_content_type(&input);
        let uploaded = client
            .upload_file(bytes, filename, Some(content_type))
            .await?;
        println!("> uploaded, file_token={}", uploaded.file_token);
        FileInput::from(uploaded.file_token.as_str())
    };

    println!("> prompt: {prompt}");
    let task_id = client
        .image_to_image(ImageToImageParams {
            model: Some(image_model::SEEDREAM_V5.to_string()),
            ..ImageToImageParams::new(file_input, prompt)
        })
        .await?;
    println!("> submitted, task_id={task_id}");

    let task = client
        .wait_for_task_with_progress(&task_id, WaitOptions::default(), |t| {
            print!("\r  {} — {}%   ", t.status, t.progress.unwrap_or(0));
        })
        .await?;
    println!();

    if task.status != TaskStatus::Success {
        anyhow::bail!("task did not succeed: {task:?}");
    }

    let Some(output) = &task.output else {
        println!("Task succeeded but returned no output.");
        return Ok(());
    };
    println!("> raw output: {:?}", output.extra);

    let mut i = 0;
    for (key, value) in &output.extra {
        if let Some(url) = value.as_str() {
            if url.starts_with("http") {
                let ext = guess_ext(url);
                let filename = format!("tripo-{task_id}-{key}-{i}{ext}");
                download(&filename, url).await?;
                i += 1;
            }
        }
    }

    Ok(())
}

async fn download(filename: &str, url: &str) -> anyhow::Result<()> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    fs::write(filename, &bytes).await?;
    println!("> saved {filename} ({} bytes)", bytes.len());
    Ok(())
}

fn guess_ext(url: &str) -> &'static str {
    let path = url.split('?').next().unwrap_or(url);
    if path.ends_with(".png") {
        ".png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        ".jpg"
    } else if path.ends_with(".webp") {
        ".webp"
    } else {
        ".bin"
    }
}

fn guess_content_type(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "application/octet-stream",
    }
}
