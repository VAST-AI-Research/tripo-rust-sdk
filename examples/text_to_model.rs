//! examples/text_to_model.rs
//!
//! Generate a 3D model from a text prompt, wait for completion, then save
//! the resulting GLB to disk.
//!
//!   $ export TRIPO_API_KEY="tsk_..."
//!   $ cargo run --example text_to_model -- "a cute red panda"

use std::env;
use tokio::fs;
use tripo3d_sdk::{
    constants::model_version, params::TextToModelParams, ClientOptions, TripoClient, WaitOptions,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let prompt = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let prompt = if prompt.is_empty() {
        "a cute red panda holding bamboo".to_string()
    } else {
        prompt
    };

    println!("> prompt: {prompt}");

    let client = TripoClient::new(ClientOptions::default())?;

    let task_id = client
        .text_to_model(TextToModelParams {
            prompt: prompt.clone(),
            model: Some(model_version::H3_1.to_string()),
            texture: Some(true),
            pbr: Some(true),
            texture_quality: Some("detailed".to_string()),
            ..Default::default()
        })
        .await?;

    println!("> submitted, task_id={task_id}");

    let task = client
        .wait_for_task_with_progress(&task_id, WaitOptions::default(), |t| {
            print!("\r  {} — {}%   ", t.status, t.progress.unwrap_or(0));
        })
        .await?;
    println!();

    match client.download_model(&task).await? {
        Some(downloaded) => {
            let filename = downloaded.filename(&format!("tripo-{task_id}"));
            fs::write(&filename, &downloaded.data).await?;
            println!("> saved {filename} ({} bytes)", downloaded.data.len());
        }
        None => println!(
            "Task succeeded but no model URL was returned: {:?}",
            task.output
        ),
    }

    Ok(())
}
