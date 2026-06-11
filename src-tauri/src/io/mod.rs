pub mod importers;
pub mod exporters;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOptions {
    pub scale: f32,
    pub flip_normals: bool,
    pub flip_uv: bool,
    pub center_origin: bool,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            flip_normals: false,
            flip_uv: false,
            center_origin: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub scale: f32,
    pub flip_normals: bool,
    pub flip_uv: bool,
    pub include_normals: bool,
    pub include_uv: bool,
    pub binary: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            flip_normals: false,
            flip_uv: false,
            include_normals: true,
            include_uv: true,
            binary: false,
        }
    }
}