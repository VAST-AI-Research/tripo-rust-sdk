# tripo3d-sdk (Rust)

**English** · [简体中文](./README.zh-CN.md)

An unofficial, async **Rust SDK** for the [Tripo3D v3 API](https://developers.tripo3d.com/zh/docs/introduction) — a full AI 3D generation platform covering text-to-3D, image-to-3D, multiview-to-3D, re-texturing, mesh editing, auto-rigging and animation retargeting.

- Built on `tokio` + `reqwest` (rustls, no OpenSSL dependency).
- Strongly-typed request/response models via `serde`.
- Automatic retries on transient network / 5xx errors, honoring `Retry-After`.
- A rich `Error` enum (`Error::Api`, `Error::Task`, `Error::Timeout`, `Error::Request`, …).
- `wait_for_task` / `wait_for_task_with_progress` pollers.
- Sibling SDK to [`tripo3d-sdk-js`](../tripo3d-sdk-js) and [`tripo3d-sdk-go`](../tripo3d-sdk-go) — same API surface, idiomatic per language.

> Base URL: `https://openapi.tripo3d.com/v3` — this SDK targets the **v3** REST endpoints under `developers.tripo3d.com`, not the older `/v2/openapi/task` endpoint.

---

## Installation

This crate is not published to crates.io yet. Add it as a git or path dependency:

```toml
[dependencies]
tripo3d-sdk = { git = "https://github.com/VAST-AI-Research/tripo-rust-sdk.git" }
# or, for local development against a working copy of this repo:
tripo3d-sdk = { path = "../tripo3d-sdk-rust" }

tokio = { version = "1", features = ["full"] }
```

If the repository is private, use the `ssh://` form (or configure Cargo's [`net.git-fetch-with-cli`](https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli) to reuse your existing SSH key):

```toml
tripo3d-sdk = { git = "ssh://git@github.com/VAST-AI-Research/tripo-rust-sdk.git" }
```

Create an API key on the [Tripo console](https://platform.tripo3d.ai/) and export it:

```bash
export TRIPO_API_KEY="tsk_..."
```

## Quick start

```rust
use tripo3d_sdk::{TripoClient, ClientOptions, WaitOptions, params::TextToModelParams, constants::model_version};

#[tokio::main]
async fn main() -> tripo3d_sdk::Result<()> {
    let client = TripoClient::new(ClientOptions::default())?; // reads TRIPO_API_KEY

    let task_id = client
        .text_to_model(TextToModelParams {
            prompt: "a cute red panda holding bamboo".into(),
            model: Some(model_version::H3_1.to_string()),
            texture: Some(true),
            pbr: Some(true),
            texture_quality: Some("detailed".into()),
            ..Default::default()
        })
        .await?;

    let task = client
        .wait_for_task_with_progress(&task_id, WaitOptions::default(), |t| {
            println!("{} — {}%", t.status, t.progress.unwrap_or(0));
        })
        .await?;

    println!("Model URL: {:?}", task.primary_model_url());
    Ok(())
}
```

> ⚠️ Model URLs expire ~5 minutes after task completion — download them right away. See `client.download_model(&task)`.

---

## Client options

```rust
pub struct ClientOptions {
    pub api_key: Option<String>,      // defaults to TRIPO_API_KEY env var
    pub base_url: Option<String>,     // default: https://openapi.tripo3d.com/v3
    pub timeout: Option<Duration>,    // per-request timeout, default 60s
    pub retries: Option<u32>,         // extra attempts on 5xx / network errors, default 2
    pub user_agent: Option<String>,
}
```

---

## API reference

Every generation method returns a `task_id` (`String`). Use `wait_for_task()` / `wait_for_task_with_progress()` to await the terminal result.

### Generation

| Method | Endpoint | Description |
| --- | --- | --- |
| `text_to_model(params)` | `POST /generation/text-to-model` | Text → 3D model |
| `image_to_model(params)` | `POST /generation/image-to-model` | Single image → 3D model |
| `multiview_to_model(params)` | `POST /generation/multiview-to-model` | 4 views `[front, left, back, right]` → 3D model |
| `text_to_image(params)` | `POST /generation/text-to-image` | Concept image from text |
| `image_to_image(params)` | `POST /generation/image-to-image` | Image style / edit |
| `image_to_multiview(params)` | `POST /generation/image-to-multiview` | Image → 4-view sheet |
| `edit_multiview(params)` | `POST /generation/edit-multiview` | Refine multiview output |

### Model post-processing

| Method | Endpoint | Description |
| --- | --- | --- |
| `texture_model(params)` | `POST /models/texture` | Re-texture an existing model |
| `convert_model(params)` | `POST /models/convert` | Convert to GLTF / FBX / OBJ / STL / USDZ / 3MF |
| `segment_mesh(params)` | `POST /mesh/segment` | Semantic segmentation |
| `complete_mesh(params)` | `POST /mesh/complete` | Mesh completion / repair |
| `decimate_mesh(params)` | `POST /mesh/decimate` | Retopology / face-count reduction |

### Animation

| Method | Endpoint | Description |
| --- | --- | --- |
| `rig_check(params)` | `POST /animations/rig-check` | Detect whether a model is riggable |
| `rig_model(params)` | `POST /animations/rig` | Attach a skeleton |
| `retarget_animation(params)` | `POST /animations/retarget` | Apply preset animations |

### Utility

| Method | Endpoint | Description |
| --- | --- | --- |
| `get_task(task_id)` | `GET /tasks/{task_id}` | Fetch a task snapshot |
| `list_tasks(task_ids)` | `POST /tasks/list` | Batch task query |
| `wait_for_task(task_id, opts)` | — | Poll until terminal state |
| `wait_for_task_with_progress(task_id, opts, cb)` | — | Same, with a progress callback |
| `upload_file(bytes, filename, content_type)` | `POST /files` | Upload a raw file and get a `file_token` |
| `get_balance()` | `GET /account/balance` | Account credit balance |
| `download_model(&task)` | — | Download the primary model URL into a `Vec<u8>` |

---

## Passing images / files

Any method that accepts an image (`file`, `image_prompt`, `style_image`, …) takes a [`FileInput`], which is `From<&str>` / `From<String>` / `From<FileDescriptor>`:

```rust
use tripo3d_sdk::{FileInput, FileDescriptor, ObjectRef};

let a: FileInput = "https://example.com/hero.png".into();   // absolute URL
let b: FileInput = "8f2a4c...".into();                       // bare file_token
let c: FileInput = FileDescriptor { url: Some("https://example.com/a.png".into()), ..Default::default() }.into();
let d: FileInput = FileDescriptor {
    object: Some(ObjectRef { bucket: "tripo-data".into(), key: "uploads/abc.png".into() }),
    ..Default::default()
}.into();
```

Upload a local buffer to get a `file_token`:

```rust
let bytes = tokio::fs::read("./hero.png").await?;
let uploaded = client.upload_file(bytes, "hero.png", Some("image/png")).await?;

let task_id = client
    .image_to_model(tripo3d_sdk::params::ImageToModelParams::new(uploaded.file_token))
    .await?;
```

---

## End-to-end pipeline: game-ready character

```rust
use tripo3d_sdk::{
    ClientOptions, TripoClient, WaitOptions,
    constants::model_version,
    params::{ImageToModelParams, RigCheckParams, RigModelParams, RetargetAnimationParams},
};

let client = TripoClient::new(ClientOptions::default())?;

// 1. Image -> 3D (low-poly P1 topology, mobile/game friendly)
let model_id = client
    .image_to_model(ImageToModelParams {
        model: Some(model_version::P1.to_string()),
        face_limit: Some(5000),
        texture: Some(true),
        ..ImageToModelParams::new("https://example.com/hero.png")
    })
    .await?;
client.wait_for_task(&model_id, WaitOptions::default()).await?;

// 2. Verify skeleton compatibility
let check_id = client.rig_check(RigCheckParams::new(model_id.as_str())).await?;
let check = client.wait_for_task(&check_id, WaitOptions::default()).await?;
let output = check.output.clone().unwrap_or_default();
assert!(output.riggable.unwrap_or(false), "model is not riggable");

// 3. Attach skeleton (Mixamo-compatible bones -> Unity/Unreal ready)
let rig_id = client
    .rig_model(RigModelParams {
        rig_type: output.rig_type.clone(),
        spec: Some("mixamo".into()),
        ..RigModelParams::new(model_id.as_str())
    })
    .await?;
client.wait_for_task(&rig_id, WaitOptions::default()).await?;

// 4. Bake preset locomotion animations
let anim_id = client
    .retarget_animation(RetargetAnimationParams {
        animations: Some(vec!["preset:idle".into(), "preset:walk".into(), "preset:run".into()]),
        out_format: Some("glb".into()),
        ..RetargetAnimationParams::new(rig_id.as_str())
    })
    .await?;
let anim = client.wait_for_task(&anim_id, WaitOptions::default()).await?;

println!("Animated GLB URLs: {:?}", anim.output.and_then(|o| o.model_urls));
```

---

## Error handling

```rust
use tripo3d_sdk::Error;

match client.text_to_model(params).await {
    Ok(task_id) => { /* ... */ }
    Err(Error::Api { code, message, suggestion, .. }) => {
        eprintln!("API error {code}: {message:?} — {suggestion:?}");
    }
    Err(Error::Task { task }) => {
        eprintln!("Task {} failed: {:?}", task.task_id, task.error_msg);
    }
    Err(Error::Timeout { task_id, timeout_ms }) => {
        eprintln!("Gave up after {timeout_ms}ms — task {task_id}");
    }
    Err(Error::Request { status, body, .. }) => {
        eprintln!("Transport failure: HTTP {status:?} — {body:?}");
    }
    Err(e) => eprintln!("{e}"),
}
```

---

## Constants

```rust
use tripo3d_sdk::{TaskStatus, Animation, RigType, RigSpec, constants::model_version, OutputFormat};

TaskStatus::Success;
Animation::Walk.as_str();          // "preset:walk"
RigType::Biped;                    // serializes as "biped"
RigSpec::Mixamo;                   // serializes as "mixamo"
model_version::H3_1;               // "v3.1-20260211"
model_version::P1;                 // "P1-20260311"
OutputFormat::Fbx;                 // serializes as "FBX"
```

---

## Running the examples

```bash
export TRIPO_API_KEY="tsk_..."

cargo run --example text_to_model -- "a wooden treasure chest"
cargo run --example image_to_model -- ./hero.png
cargo run --example rig_and_animate -- https://example.com/hero.png
```

## Development

```bash
cargo build
cargo test      # hermetic — uses `wiremock` to stub the HTTP layer, no real API key needed
```

Source tree:

```
src/
  lib.rs         # public exports
  client.rs      # TripoClient — all API methods
  http.rs        # reqwest wrapper with retry + envelope parsing
  error.rs       # Error enum
  constants.rs   # enums (TaskStatus, Animation, RigType, …)
  models.rs      # Task, TaskOutput, Balance, FileDescriptor, …
  params.rs       # per-endpoint request parameter structs
examples/        # runnable end-to-end demos
tests/           # wiremock-backed integration tests
```

---

## Reference

- API reference (English): https://developers.tripo3d.com/en/docs/introduction
- API reference (中文): https://developers.tripo3d.com/zh/docs/introduction
- Endpoint details: https://docs.tripo3d.ai/

## License

MIT — see `LICENSE`. Not affiliated with, or endorsed by, VAST AI / Tripo3D.
