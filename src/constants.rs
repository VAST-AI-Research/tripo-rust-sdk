//! Enumerations and public constants for the Tripo3D v3 API.
//!
//! Enums implement `Serialize`/`Deserialize` via their kebab/snake-case wire
//! representation (matching the JSON strings the API expects), and
//! `AsRef<str>` / `Display` for easy interpolation into request payloads.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Default REST endpoint for the Tripo3D v3 openapi service.
pub const DEFAULT_BASE_URL: &str = "https://openapi.tripo3d.com/v3";

/// Task lifecycle statuses returned by `GET /v3/tasks/{task_id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Queued,
    Running,
    Success,
    Failed,
    Cancelled,
    Unknown,
    Banned,
    Expired,
}

impl TaskStatus {
    /// A task will not change once it enters one of these states.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskStatus::Success
                | TaskStatus::Failed
                | TaskStatus::Cancelled
                | TaskStatus::Banned
                | TaskStatus::Expired
        )
    }

    pub fn is_success(&self) -> bool {
        matches!(self, TaskStatus::Success)
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::Success => "success",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
            TaskStatus::Unknown => "unknown",
            TaskStatus::Banned => "banned",
            TaskStatus::Expired => "expired",
        };
        f.write_str(s)
    }
}

/// Preset animation identifiers accepted by `POST /v3/animations/retarget`.
/// Combine at most 5 in a single retarget call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Animation {
    Idle,
    Walk,
    Run,
    Dive,
    Climb,
    Jump,
    Slash,
    Shoot,
    Hurt,
    Fall,
    Turn,
    QuadrupedWalk,
    HexapodWalk,
    OctopodWalk,
    SerpentineMarch,
    AquaticMarch,
}

impl Animation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Animation::Idle => "preset:idle",
            Animation::Walk => "preset:walk",
            Animation::Run => "preset:run",
            Animation::Dive => "preset:dive",
            Animation::Climb => "preset:climb",
            Animation::Jump => "preset:jump",
            Animation::Slash => "preset:slash",
            Animation::Shoot => "preset:shoot",
            Animation::Hurt => "preset:hurt",
            Animation::Fall => "preset:fall",
            Animation::Turn => "preset:turn",
            Animation::QuadrupedWalk => "preset:quadruped:walk",
            Animation::HexapodWalk => "preset:hexapod:walk",
            Animation::OctopodWalk => "preset:octopod:walk",
            Animation::SerpentineMarch => "preset:serpentine:march",
            Animation::AquaticMarch => "preset:aquatic:march",
        }
    }
}

impl fmt::Display for Animation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for Animation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Skeleton topology types for `POST /v3/animations/rig`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RigType {
    Biped,
    Quadruped,
    Hexapod,
    Octopod,
    Avian,
    Serpentine,
    Aquatic,
    Others,
}

impl fmt::Display for RigType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RigType::Biped => "biped",
            RigType::Quadruped => "quadruped",
            RigType::Hexapod => "hexapod",
            RigType::Octopod => "octopod",
            RigType::Avian => "avian",
            RigType::Serpentine => "serpentine",
            RigType::Aquatic => "aquatic",
            RigType::Others => "others",
        };
        f.write_str(s)
    }
}

/// Rig specifications — controls bone naming/hierarchy conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RigSpec {
    Mixamo,
    Tripo,
}

impl fmt::Display for RigSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RigSpec::Mixamo => "mixamo",
            RigSpec::Tripo => "tripo",
        };
        f.write_str(s)
    }
}

/// Output format for rig / retarget tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimOutFormat {
    Glb,
    Fbx,
}

impl fmt::Display for AnimOutFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AnimOutFormat::Glb => "glb",
            AnimOutFormat::Fbx => "fbx",
        };
        f.write_str(s)
    }
}

/// Available `model` values (product lines) exposed by the v3 API.
/// This is intentionally a plain string wrapper (not a closed enum) since
/// Tripo3D regularly ships new model versions.
pub mod model_version {
    pub const H3_1: &str = "v3.1-20260211";
    pub const H3_0: &str = "v3.0-20250812";
    pub const H2_5: &str = "v2.5-20250123";
    pub const H2_0: &str = "v2.0-20240919";
    pub const P1: &str = "P1-20260311";
    pub const TURBO_V1: &str = "Turbo-v1.0-20250506";
}

/// Model conversion output formats accepted by `POST /v3/models/convert`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputFormat {
    #[serde(rename = "GLTF")]
    Gltf,
    #[serde(rename = "GLB")]
    Glb,
    #[serde(rename = "USDZ")]
    Usdz,
    #[serde(rename = "FBX")]
    Fbx,
    #[serde(rename = "OBJ")]
    Obj,
    #[serde(rename = "STL")]
    Stl,
    #[serde(rename = "3MF")]
    ThreeMf,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OutputFormat::Gltf => "GLTF",
            OutputFormat::Glb => "GLB",
            OutputFormat::Usdz => "USDZ",
            OutputFormat::Fbx => "FBX",
            OutputFormat::Obj => "OBJ",
            OutputFormat::Stl => "STL",
            OutputFormat::ThreeMf => "3MF",
        };
        f.write_str(s)
    }
}

/// Texture image formats supported by the conversion API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextureFormat {
    #[serde(rename = "BMP")]
    Bmp,
    #[serde(rename = "DPX")]
    Dpx,
    #[serde(rename = "HDR")]
    Hdr,
    #[serde(rename = "JPEG")]
    Jpeg,
    #[serde(rename = "OPEN_EXR")]
    OpenExr,
    #[serde(rename = "PNG")]
    Png,
    #[serde(rename = "TARGA")]
    Targa,
    #[serde(rename = "TIFF")]
    Tiff,
    #[serde(rename = "WEBP")]
    Webp,
}
