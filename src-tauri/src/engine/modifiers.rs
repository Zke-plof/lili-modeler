use glam::Vec3;
use serde::{Deserialize, Serialize};
use crate::mesh::Mesh;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModifierStack {
    pub modifiers: Vec<Modifier>,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Modifier {
    Array {
        count: u32,
        offset: [f32; 3],
        use_relative: bool,
    },
    Mirror {
        axis: [bool; 3],
        use_clip: bool,
    },
    SubdivisionSurface {
        levels: u32,
        algorithm: SubdivAlgorithm,
    },
    Solidify {
        thickness: f32,
        offset: f32,
    },
    Bevel {
        width: f32,
        segments: u32,
        limit_method: BevelLimitMethod,
    },
    Boolean {
        operation: BooleanOp,
        target_id: String,
    },
    Weld {
        threshold: f32,
    },
    Decimate {
        ratio: f32,
        mode: DecimateMode,
    },
    Smooth {
        factor: f32,
        iterations: u32,
    },
    LaplacianSmooth {
        factor: f32,
        iterations: u32,
    },
    Triangulate {
        quad_method: TriangulateQuadMethod,
    },
    WeightedNormal {
        weight: f32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubdivAlgorithm {
    CatmullClark,
    Simple,
    Linear,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BevelLimitMethod {
    None,
    Angle,
    Weight,
    VertexGroup,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BooleanOp {
    Union,
    Difference,
    Intersect,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DecimateMode {
    Collapse,
    UnSubdivide,
    Planar,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TriangulateQuadMethod {
    Beauty,
    Fixed,
    Alternate,
    FixedAlt,
}

impl Default for ModifierStack {
    fn default() -> Self {
        Self {
            modifiers: Vec::new(),
            enabled: true,
        }
    }
}

impl ModifierStack {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, modifier: Modifier) {
        self.modifiers.push(modifier);
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.modifiers.len() {
            self.modifiers.remove(index);
        }
    }

    pub fn move_up(&mut self, index: usize) {
        if index > 0 && index < self.modifiers.len() {
            self.modifiers.swap(index, index - 1);
        }
    }

    pub fn move_down(&mut self, index: usize) {
        if index < self.modifiers.len().saturating_sub(1) {
            self.modifiers.swap(index, index + 1);
        }
    }

    pub fn apply_all(&self, mesh: &Mesh) -> Mesh {
        let mut result = mesh.clone();
        if !self.enabled { return result; }

        for modifier in &self.modifiers {
            result = match modifier {
                Modifier::Array { count, offset, use_relative } => {
                    self.apply_array(&result, *count, Vec3::from(*offset), *use_relative)
                }
                Modifier::Mirror { axis, use_clip } => {
                    self.apply_mirror(&result, *axis, *use_clip)
                }
                Modifier::SubdivisionSurface { levels, .. } => {
                    self.apply_subdivision(&result, *levels)
                }
                Modifier::Solidify { thickness, offset } => {
                    self.apply_solidify(&result, *thickness, *offset)
                }
                Modifier::Smooth { factor, iterations } => {
                    self.apply_smooth(&result, *factor, *iterations)
                }
                Modifier::Weld { threshold } => {
                    self.apply_weld(&result, *threshold)
                }
                Modifier::Triangulate { .. } => {
                    self.apply_triangulate(&result)
                }
                _ => result,
            };
        }
        result
    }

    fn apply_array(&self, mesh: &Mesh, count: u32, offset: Vec3, _relative: bool) -> Mesh {
        let mut result = mesh.clone();
        let vertex_count = result.vertices.len() as u32;

        for i in 1..count {
            let offset = offset * i as f32;
            for vert in &mesh.vertices {
                result.add_vertex(vert.position + offset);
            }
            for face in &mesh.faces {
                let new_ids: Vec<u32> = face.vertex_ids.iter()
                    .map(|id| id + i * vertex_count)
                    .collect();
                result.add_face(new_ids);
            }
        }
        result.recalculate_normals();
        result
    }

    fn apply_mirror(&self, mesh: &Mesh, axis: [bool; 3], _clip: bool) -> Mesh {
        let mut result = mesh.clone();
        let vertex_count = result.vertices.len() as u32;

        for vert in &mesh.vertices {
            let mut pos = vert.position;
            if axis[0] { pos.x = -pos.x; }
            if axis[1] { pos.y = -pos.y; }
            if axis[2] { pos.z = -pos.z; }
            result.add_vertex(pos);
        }

        for face in &mesh.faces {
            let mut new_ids: Vec<u32> = face.vertex_ids.iter()
                .map(|id| id + vertex_count)
                .collect();
            new_ids.reverse();
            result.add_face(new_ids);
        }

        result.recalculate_normals();
        result
    }

    fn apply_subdivision(&self, mesh: &Mesh, levels: u32) -> Mesh {
        let mut result = mesh.clone();
        for _ in 0..levels {
            let new_mesh = Mesh::new();
            let _ = new_mesh;
            result.recalculate_normals();
        }
        result
    }

    fn apply_solidify(&self, mesh: &Mesh, thickness: f32, _offset: f32) -> Mesh {
        let mut result = mesh.clone();
        let vertex_count = result.vertices.len() as u32;

        for vert in &mesh.vertices {
            result.add_vertex(vert.position + vert.normal * thickness);
        }

        for face in &mesh.faces {
            let back_ids: Vec<u32> = face.vertex_ids.iter()
                .map(|id| id + vertex_count)
                .collect();
            let mut back_reversed = back_ids.clone();
            back_reversed.reverse();
            result.add_face(back_reversed);
        }

        result.recalculate_normals();
        result
    }

    fn apply_smooth(&self, mesh: &Mesh, factor: f32, iterations: u32) -> Mesh {
        let mut result = mesh.clone();
        for _ in 0..iterations {
            let original = result.clone();
            for (i, vert) in result.vertices.iter_mut().enumerate() {
                let neighbors: Vec<Vec3> = original.edges.iter()
                    .filter(|e| e.vert_ids[0] as usize == i || e.vert_ids[1] as usize == i)
                    .map(|e| {
                        let other_id = if e.vert_ids[0] as usize == i { e.vert_ids[1] } else { e.vert_ids[0] };
                        original.vertices[other_id as usize].position
                    })
                    .collect();

                if !neighbors.is_empty() {
                    let avg: Vec3 = neighbors.iter().sum::<Vec3>() / neighbors.len() as f32;
                    vert.position = vert.position.lerp(avg, factor);
                }
            }
        }
        result.recalculate_normals();
        result
    }

    fn apply_weld(&self, mesh: &Mesh, threshold: f32) -> Mesh {
        let mut result = Mesh::new();
        let mut vertex_map: Vec<u32> = vec![0; mesh.vertices.len()];

        for (i, vert) in mesh.vertices.iter().enumerate() {
            let mut merged = false;
            for (j, existing) in result.vertices.iter().enumerate() {
                if vert.position.distance(existing.position) < threshold {
                    vertex_map[i] = j as u32;
                    merged = true;
                    break;
                }
            }
            if !merged {
                vertex_map[i] = result.add_vertex(vert.position);
            }
        }

        for face in &mesh.faces {
            let new_ids: Vec<u32> = face.vertex_ids.iter()
                .map(|id| vertex_map[*id as usize])
                .collect();
            if new_ids.iter().collect::<std::collections::HashSet<_>>().len() == new_ids.len() {
                result.add_face(new_ids);
            }
        }

        result.recalculate_normals();
        result
    }

    fn apply_triangulate(&self, mesh: &Mesh) -> Mesh {
        let mut result = Mesh::new();

        for vert in &mesh.vertices {
            result.add_vertex(vert.position);
        }

        for face in &mesh.faces {
            if face.vertex_ids.len() == 3 {
                result.add_face(face.vertex_ids.clone());
            } else if face.vertex_ids.len() > 3 {
                for i in 1..face.vertex_ids.len() - 1 {
                    result.add_face(vec![
                        face.vertex_ids[0],
                        face.vertex_ids[i],
                        face.vertex_ids[i + 1],
                    ]);
                }
            }
        }

        result.recalculate_normals();
        result
    }
}