//! examples/text_to_image.rs
//!
//! Generate a concept image from a text prompt, wait for the task to
//! complete, then save every URL found in the task output to disk.
//!
//!   $ export TRIPO_API_KEY="tsk_..."
//!   $ cargo run --example text_to_image -- "a cute red panda holding bamboo"

use std::env;
use tokio::fs;
use tripo3d_sdk::{
    constants::TaskStatus, params::TextToImageParams, ClientOptions, TripoClient, WaitOptions,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let prompt = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let prompt = if prompt.is_empty() {
        "a cute red panda holding bamboo, studio lighting".to_string()
    } else {
        prompt
    };
    println!("> prompt: {prompt}");

    let client = TripoClient::new(ClientOptions::default())?;

    let task_id = client.text_to_image(TextToImageParams::new(prompt)).await?;
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
