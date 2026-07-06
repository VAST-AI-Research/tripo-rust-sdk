//! Request parameter structs for every `TripoClient` method.
//!
//! Every struct derives `Default` so callers can use struct-update syntax:
//!
//! ```no_run
//! use tripo3d_sdk::params::TextToModelParams;
//!
//! let params = TextToModelParams {
//!     prompt: "a cute cat".into(),
//!     texture: Some(true),
//!     ..Default::default()
//! };
//! ```
//!
//! Unknown/forward-compatible fields can be passed via `extra`, which is
//! flattened into the JSON payload alongside the typed fields.

use crate::models::FileDescriptor;
use serde::Serialize;
use std::collections::HashMap;

/// `POST /v3/generation/text-to-model`
#[derive(Debug, Clone, Default, Serialize)]
pub struct TextToModelParams {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_low_poly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_parts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_uv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl TextToModelParams {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/generation/image-to-model`
#[derive(Debug, Clone, Default, Serialize)]
pub struct ImageToModelParams {
    pub file: FileDescriptor,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_image_autofix: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_low_poly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_parts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_uv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ImageToModelParams {
    pub fn new(file: impl Into<crate::models::FileInput>) -> Self {
        Self {
            file: file.into().into_descriptor(),
            ..Default::default()
        }
    }
}

/// `POST /v3/generation/multiview-to-model`
///
/// `files`, when set, must contain exactly 4 items in
/// `[front, left, back, right]` order. Individual items may be `None`
/// (empty descriptor) except the front view. Mutually exclusive with
/// `original_task_id`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct MultiviewToModelParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<[FileDescriptor; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_low_poly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_parts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl MultiviewToModelParams {
    /// `files` in `[front, left, back, right]` order — use `None` for
    /// omitted non-front views.
    ///
    /// ```
    /// use tripo3d_sdk::{models::FileInput, params::MultiviewToModelParams};
    ///
    /// let params = MultiviewToModelParams::from_files([
    ///     Some(FileInput::from("https://example.com/front.png")),
    ///     None,
    ///     Some(FileInput::from("https://example.com/back.png")),
    ///     None,
    /// ]);
    /// ```
    pub fn from_files(files: [Option<crate::models::FileInput>; 4]) -> Self {
        let files = files.map(|f| {
            f.map(crate::models::FileInput::into_descriptor)
                .unwrap_or_default()
        });
        Self {
            files: Some(files),
            ..Default::default()
        }
    }

    pub fn from_original_task_id(task_id: impl Into<String>) -> Self {
        Self {
            original_task_id: Some(task_id.into()),
            ..Default::default()
        }
    }
}

/// `POST /v3/generation/text-to-image`
#[derive(Debug, Clone, Default, Serialize)]
pub struct TextToImageParams {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl TextToImageParams {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/generation/image-to-image`
#[derive(Debug, Clone, Default, Serialize)]
pub struct ImageToImageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// `POST /v3/generation/image-to-multiview`
#[derive(Debug, Clone, Default, Serialize)]
pub struct ImageToMultiviewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileDescriptor>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// `POST /v3/generation/edit-multiview`
#[derive(Debug, Clone, Default, Serialize)]
pub struct EditMultiviewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_task_id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// `POST /v3/models/texture`
#[derive(Debug, Clone, Default, Serialize)]
pub struct TextureModelParams {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_prompt: Option<FileDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_image: Option<FileDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bake: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_names: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl TextureModelParams {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/models/convert`
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConvertModelParams {
    pub input: String,
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten_bottom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten_bottom_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_to_center_bottom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_animation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_uv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_symmetry: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bake: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_names: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ConvertModelParams {
    pub fn new(input: impl Into<String>, format: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            format: format.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/mesh/segment`
#[derive(Debug, Clone, Default, Serialize)]
pub struct SegmentMeshParams {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl SegmentMeshParams {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/mesh/complete`
#[derive(Debug, Clone, Default, Serialize)]
pub struct CompleteMeshParams {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_names: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl CompleteMeshParams {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/mesh/decimate`
#[derive(Debug, Clone, Default, Serialize)]
pub struct DecimateMeshParams {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bake: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_names: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl DecimateMeshParams {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/animations/rig-check`
#[derive(Debug, Clone, Default, Serialize)]
pub struct RigCheckParams {
    pub input: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl RigCheckParams {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/animations/rig`
#[derive(Debug, Clone, Default, Serialize)]
pub struct RigModelParams {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rig_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_format: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl RigModelParams {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

/// `POST /v3/animations/retarget`
///
/// Provide either `animation` (single preset) or `animations` (up to 5).
#[derive(Debug, Clone, Default, Serialize)]
pub struct RetargetAnimationParams {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bake_animation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_with_geometry: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate_in_place: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl RetargetAnimationParams {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}
