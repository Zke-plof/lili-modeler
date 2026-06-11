use glam::{Vec3, Vec2};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UvMap {
    pub vertices: Vec<Vec2>,
    pub faces: Vec<Vec<u32>>,
}

impl Default for UvMap {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub normal_strength: f32,
    pub emission: [f32; 3],
    pub emission_strength: f32,
    pub alpha: f32,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub texture_slots: Vec<TextureSlot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlendMode {
    Opaque,
    AlphaBlend,
    AlphaHashed,
    AlphaClip,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextureSlot {
    pub slot_type: TextureType,
    pub image_path: Option<String>,
    pub color: [f32; 4],
    pub projection: ProjectionType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TextureType {
    BaseColor,
    Metallic,
    Roughness,
    Normal,
    Emission,
    AO,
    Height,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProjectionType {
    Flat,
    Box,
    Sphere,
    Tube,
    Cardinal,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "Material".to_string(),
            base_color: [0.8, 0.8, 0.8, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            normal_strength: 1.0,
            emission: [0.0, 0.0, 0.0],
            emission_strength: 0.0,
            alpha: 1.0,
            blend_mode: BlendMode::Opaque,
            double_sided: false,
            texture_slots: Vec::new(),
        }
    }
}

impl Material {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn metal(color: [f32; 3]) -> Self {
        Self {
            name: "Metal".to_string(),
            base_color: [color[0], color[1], color[2], 1.0],
            metallic: 0.9,
            roughness: 0.2,
            ..Default::default()
        }
    }

    pub fn plastic(color: [f32; 3]) -> Self {
        Self {
            name: "Plastic".to_string(),
            base_color: [color[0], color[1], color[2], 1.0],
            metallic: 0.0,
            roughness: 0.4,
            ..Default::default()
        }
    }

    pub fn glass() -> Self {
        Self {
            name: "Glass".to_string(),
            base_color: [0.9, 0.95, 1.0, 0.1],
            metallic: 0.0,
            roughness: 0.0,
            alpha: 0.1,
            blend_mode: BlendMode::AlphaBlend,
            ..Default::default()
        }
    }

    pub fn rubber() -> Self {
        Self {
            name: "Rubber".to_string(),
            base_color: [0.1, 0.1, 0.1, 1.0],
            metallic: 0.0,
            roughness: 0.9,
            ..Default::default()
        }
    }
}

pub struct UvUnwrapper;

impl UvUnwrapper {
    pub fn unwrap_plane(mesh: &crate::mesh::Mesh, axis: [bool; 3]) -> UvMap {
        let mut uv_map = UvMap::default();
        let mut min = Vec2::splat(f32::INFINITY);
        let mut max = Vec2::splat(f32::NEG_INFINITY);

        for vert in &mesh.vertices {
            let (u, v) = if axis[0] && axis[1] {
                (vert.position.x, vert.position.y)
            } else if axis[1] && axis[2] {
                (vert.position.y, vert.position.z)
            } else {
                (vert.position.x, vert.position.z)
            };
            min = min.min(Vec2::new(u, v));
            max = max.max(Vec2::new(u, v));
        }

        let size = (max - min).max_element();
        if size < 1e-6 {
            return uv_map;
        }

        for vert in &mesh.vertices {
            let (u, v) = if axis[0] && axis[1] {
                (vert.position.x, vert.position.y)
            } else if axis[1] && axis[2] {
                (vert.position.y, vert.position.z)
            } else {
                (vert.position.x, vert.position.z)
            };
            uv_map.vertices.push(Vec2::new(
                (u - min.x) / size,
                (v - min.y) / size,
            ));
        }

        for face in &mesh.faces {
            uv_map.faces.push(face.vertex_ids.clone());
        }

        uv_map
    }

    pub fn unwrap_box(mesh: &crate::mesh::Mesh) -> UvMap {
        Self::unwrap_plane(mesh, [true, true, false])
    }

    pub fn unwrap_sphere(mesh: &crate::mesh::Mesh) -> UvMap {
        let mut uv_map = UvMap::default();

        for vert in &mesh.vertices {
            let r = vert.position.length();
            if r < 1e-6 {
                uv_map.vertices.push(Vec2::ZERO);
                continue;
            }
            let u = 0.5 + vert.position.z.atan2(vert.position.x) / (2.0 * PI);
            let v = 0.5 - (vert.position.y / r).asin() / PI;
            uv_map.vertices.push(Vec2::new(u, v));
        }

        for face in &mesh.faces {
            uv_map.faces.push(face.vertex_ids.clone());
        }

        uv_map
    }
}