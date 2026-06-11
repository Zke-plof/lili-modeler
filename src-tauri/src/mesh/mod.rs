pub mod primitives;
pub mod bmesh;

use glam::{Vec3, Vec2};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SelectState {
    None,
    Hover,
    Selected,
}

impl Default for SelectState {
    fn default() -> Self { Self::None }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vert {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub color: [f32; 4],
    pub select: SelectState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub vert_ids: [u32; 2],
    pub face_ids: Vec<u32>,
    pub select: SelectState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Face {
    pub vertex_ids: Vec<u32>,
    pub normal: Vec3,
    pub select: SelectState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mesh {
    pub vertices: Vec<Vert>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub name: String,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            name: String::new(),
        }
    }
}

impl Mesh {
    pub fn new() -> Self { Self::default() }

    pub fn add_vertex(&mut self, pos: Vec3) -> u32 {
        let id = self.vertices.len() as u32;
        self.vertices.push(Vert {
            position: pos,
            normal: Vec3::Y,
            uv: Vec2::ZERO,
            color: [0.8, 0.8, 0.8, 1.0],
            select: SelectState::None,
        });
        id
    }

    pub fn add_edge(&mut self, v0: u32, v1: u32) -> u32 {
        let id = self.edges.len() as u32;
        self.edges.push(Edge {
            vert_ids: [v0, v1],
            face_ids: Vec::new(),
            select: SelectState::None,
        });
        id
    }

    pub fn add_face(&mut self, vert_ids: Vec<u32>) -> u32 {
        let id = self.faces.len() as u32;
        let normal = self.calculate_face_normal(&vert_ids);
        self.faces.push(Face {
            vertex_ids: vert_ids,
            normal,
            select: SelectState::None,
        });
        id
    }

    pub fn calculate_face_normal(&self, vert_ids: &[u32]) -> Vec3 {
        if vert_ids.len() < 3 {
            return Vec3::Y;
        }
        let v0 = self.vertices[vert_ids[0] as usize].position;
        let v1 = self.vertices[vert_ids[1] as usize].position;
        let v2 = self.vertices[vert_ids[2] as usize].position;
        (v1 - v0).cross(v2 - v0).normalize()
    }

    pub fn calculate_volume(&self) -> f32 {
        let mut volume = 0.0;
        for face in &self.faces {
            if face.vertex_ids.len() < 3 { continue; }
            for i in 1..face.vertex_ids.len() - 1 {
                let v0 = self.vertices[face.vertex_ids[0] as usize].position;
                let v1 = self.vertices[face.vertex_ids[i] as usize].position;
                let v2 = self.vertices[face.vertex_ids[i + 1] as usize].position;
                volume += v0.dot(v1.cross(v2)) / 6.0;
            }
        }
        volume.abs()
    }

    pub fn calculate_surface_area(&self) -> f32 {
        let mut area = 0.0;
        for face in &self.faces {
            if face.vertex_ids.len() < 3 { continue; }
            for i in 1..face.vertex_ids.len() - 1 {
                let v0 = self.vertices[face.vertex_ids[0] as usize].position;
                let v1 = self.vertices[face.vertex_ids[i] as usize].position;
                let v2 = self.vertices[face.vertex_ids[i + 1] as usize].position;
                area += (v1 - v0).cross(v2 - v0).length() * 0.5;
            }
        }
        area
    }

    pub fn to_vertex_buffer(&self) -> Vec<Vertex> {
        let mut buffer = Vec::with_capacity(self.vertices.len());
        for vert in &self.vertices {
            buffer.push(Vertex {
                position: vert.position.into(),
                normal: vert.normal.into(),
                uv: vert.uv.into(),
                color: vert.color,
            });
        }
        buffer
    }

    pub fn to_index_buffer(&self) -> Vec<u32> {
        let mut indices = Vec::new();
        for face in &self.faces {
            for i in 1..face.vertex_ids.len() - 1 {
                indices.push(face.vertex_ids[0]);
                indices.push(face.vertex_ids[i]);
                indices.push(face.vertex_ids[i + 1]);
            }
        }
        indices
    }

    pub fn recalculate_normals(&mut self) {
        let mut normals = vec![Vec3::ZERO; self.vertices.len()];
        
        for face in &self.faces {
            let normal = self.calculate_face_normal(&face.vertex_ids);
            for &vid in &face.vertex_ids {
                normals[vid as usize] += normal;
            }
        }
        
        for (i, vert) in self.vertices.iter_mut().enumerate() {
            vert.normal = normals[i].normalize_or_zero();
        }
    }

    pub fn extrude_faces(&self, face_ids: &[u32], distance: f32) -> Self {
        let mut new_mesh = self.clone();
        
        for &fid in face_ids {
            let face = &new_mesh.faces[fid as usize];
            let normal = face.normal;
            let offset = normal * distance;
            let old_ids: Vec<u32> = face.vertex_ids.clone();
            
            let mut new_ids = Vec::new();
            for &vid in &old_ids {
                let new_vid = new_mesh.add_vertex(new_mesh.vertices[vid as usize].position + offset);
                new_ids.push(new_vid);
            }
            
            new_mesh.add_face(new_ids);
        }
        
        new_mesh
    }

    pub fn inset_faces(&self, face_ids: &[u32], thickness: f32) -> Self {
        let mut new_mesh = self.clone();
        
        let face_data: Vec<(Vec3, Vec<u32>)> = face_ids.iter().map(|&fid| {
            let face = &new_mesh.faces[fid as usize];
            let center: Vec3 = face.vertex_ids.iter()
                .map(|&vid| new_mesh.vertices[vid as usize].position)
                .sum::<Vec3>() / face.vertex_ids.len() as f32;
            let verts: Vec<u32> = face.vertex_ids.clone();
            (center, verts)
        }).collect();
        
        for (center, verts) in face_data {
            let mut new_ids = Vec::new();
            for vid in verts {
                let pos = new_mesh.vertices[vid as usize].position;
                let new_pos = pos.lerp(center, thickness);
                let new_vid = new_mesh.add_vertex(new_pos);
                new_ids.push(new_vid);
            }
            
            new_mesh.add_face(new_ids);
        }
        
        new_mesh
    }

    pub fn bevel_edges(&self, edge_ids: &[u32], segments: u32) -> Self {
        let mut new_mesh = self.clone();
        
        for &eid in edge_ids {
            let edge = &new_mesh.edges[eid as usize];
            let v0 = new_mesh.vertices[edge.vert_ids[0] as usize].position;
            let v1 = new_mesh.vertices[edge.vert_ids[1] as usize].position;
            let mid = (v0 + v1) * 0.5;
            
            for seg in 1..=segments {
                let t = seg as f32 / (segments + 1) as f32;
                let pos = v0.lerp(v1, t);
                let _ = new_mesh.add_vertex(pos);
            }
        }
        
        new_mesh
    }

    pub fn loop_cut(&self, _edge_loop_id: u32, cuts: u32) -> Self {
        let mut new_mesh = self.clone();
        let _ = cuts;
        new_mesh.recalculate_normals();
        new_mesh
    }

    pub fn boolean_union(&self, _other: &Mesh) -> Self {
        let mut result = self.clone();
        result.recalculate_normals();
        result
    }

    pub fn boolean_difference(&self, _other: &Mesh) -> Self {
        let mut result = self.clone();
        result.recalculate_normals();
        result
    }

    pub fn boolean_intersect(&self, _other: &Mesh) -> Self {
        let mut result = self.clone();
        result.recalculate_normals();
        result
    }
}