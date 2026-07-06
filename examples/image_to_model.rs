//! examples/image_to_model.rs
//!
//! Convert a local or remote image into a 3D model.
//!
//!   # local file
//!   $ cargo run --example image_to_model -- ./cat.png
//!   # remote URL
//!   $ cargo run --example image_to_model -- https://example.com/cat.jpg

use std::env;
use std::path::Path;
use tokio::fs;
use tripo3d_sdk::{
    constants::model_version, models::FileInput, params::ImageToModelParams, ClientOptions,
    TripoClient, WaitOptions,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = env::args().nth(1).ok_or_else(|| {
        anyhow::anyhow!("Usage: cargo run --example image_to_model -- <local-file|url>")
    })?;

    let client = TripoClient::new(ClientOptions::default())?;

    let file_input: FileInput = if input.starts_with("http://") || input.starts_with("https://") {
        FileInput::Url(input.clone())
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
        FileInput::FileToken(uploaded.file_token)
    };

    let task_id = client
        .image_to_model(ImageToModelParams {
            model: Some(model_version::H3_1.to_string()),
            texture: Some(true),
            pbr: Some(true),
            texture_alignment: Some("original_image".to_string()),
            ..ImageToModelParams::new(file_input)
        })
        .await?;
    println!("> submitted, task_id={task_id}");

    let task = client
        .wait_for_task_with_progress(&task_id, WaitOptions::default(), |t| {
            print!("\r  {} — {}%   ", t.status, t.progress.unwrap_or(0));
        })
        .await?;
    println!();

    if let Some(downloaded) = client.download_model(&task).await? {
        let filename = format!("tripo-{task_id}.glb");
        fs::write(&filename, &downloaded.data).await?;
        println!("> saved {filename} ({} bytes)", downloaded.data.len());
    }

    Ok(())
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
        "bmp" => "image/bmp",
        _ => "application/octet-stream",
    }
}
