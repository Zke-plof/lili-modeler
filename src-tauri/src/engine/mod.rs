pub mod commands;
pub mod modifiers;
pub mod undo;
pub mod materials;
pub mod snapping;
pub mod overlay;
pub mod sculpt;
pub mod animation;
pub mod physics;
pub mod nodes;
pub mod renderer;

use crate::mesh::Mesh;
use crate::scene::{Scene, SceneObject};
use modifiers::ModifierStack;
use undo::UndoSystem;
use snapping::{SnappingConfig, ProportionalEditing};
use overlay::OverlaySystem;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use std::collections::HashMap;

pub struct Engine {
    pub scene: Arc<RwLock<Scene>>,
    pub selection: Selection,
    pub undo_system: UndoSystem,
    pub modifier_stacks: HashMap<Uuid, ModifierStack>,
    pub materials: HashMap<Uuid, materials::Material>,
    pub snapping: SnappingConfig,
    pub proportional: ProportionalEditing,
    pub overlay: OverlaySystem,
    pub clipboard_mesh: Option<Mesh>,
    pub pivot_point: PivotPoint,
    pub orientation: TransformOrientation,
    pub tool_settings: ToolSettings,
}

#[derive(Clone, Debug)]
pub enum PivotPoint {
    Median,
    Individual,
    Center,
    Cursor,
    Active,
    BoundingBox,
}

impl Default for PivotPoint {
    fn default() -> Self { Self::Median }
}

#[derive(Clone, Debug, Default)]
pub enum TransformOrientation {
    #[default]
    Global,
    Local,
    Normal,
    Gimbal,
    Parent,
    Cursor,
    Custom,
}

#[derive(Clone, Debug)]
pub struct ToolSettings {
    pub move_snap: bool,
    pub move_snap_distance: f32,
    pub rotate_snap_angle: f32,
    pub scale_uniform: bool,
    pub extrude_depth: f32,
    pub inset_thickness: f32,
    pub bevel_width: f32,
    pub bevel_segments: u32,
    pub loop_cut_count: u32,
    pub knife_snap_to_grid: bool,
    pub knife_cut_through: bool,
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            move_snap: false,
            move_snap_distance: 0.1,
            rotate_snap_angle: 5.0,
            scale_uniform: false,
            extrude_depth: 0.5,
            inset_thickness: 0.1,
            bevel_width: 0.1,
            bevel_segments: 3,
            loop_cut_count: 1,
            knife_snap_to_grid: false,
            knife_cut_through: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Selection {
    pub mode: SelectMode,
    pub object_ids: Vec<Uuid>,
    pub vertex_ids: Vec<u32>,
    pub edge_ids: Vec<u32>,
    pub face_ids: Vec<u32>,
    pub active_vertex: Option<u32>,
    pub active_edge: Option<u32>,
    pub active_face: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectMode {
    #[default]
    Object,
    Vertex,
    Edge,
    Face,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            scene: Arc::new(RwLock::new(Scene::new())),
            selection: Selection::default(),
            undo_system: UndoSystem::new(64),
            modifier_stacks: HashMap::new(),
            materials: HashMap::new(),
            snapping: SnappingConfig::default(),
            proportional: ProportionalEditing::default(),
            overlay: OverlaySystem::new(),
            clipboard_mesh: None,
            pivot_point: PivotPoint::default(),
            orientation: TransformOrientation::default(),
            tool_settings: ToolSettings::default(),
        }
    }

    pub fn add_mesh(&mut self, name: &str, mesh: Mesh) -> Uuid {
        let id = Uuid::new_v4();
        let mut scene = self.scene.write();
        let obj = SceneObject::new(id, name.to_string(), mesh);
        scene.objects.insert(id, obj);
        self.modifier_stacks.insert(id, ModifierStack::new());
        self.materials.insert(id, materials::Material::default());
        self.undo_system.push(
            format!("Create {}", name),
            id.to_string(),
            &scene.objects[&id].mesh,
        );
        id
    }

    pub fn remove_mesh(&mut self, id: &Uuid) -> Option<SceneObject> {
        let mut scene = self.scene.write();
        self.modifier_stacks.remove(id);
        self.materials.remove(id);
        scene.objects.remove(id)
    }

    pub fn duplicate_mesh(&mut self, id: &Uuid) -> Option<Uuid> {
        let scene = self.scene.read();
        if let Some(obj) = scene.objects.get(id) {
            let new_mesh = obj.mesh.clone();
            let new_name = format!("{}_copy", obj.name);
            drop(scene);
            Some(self.add_mesh(&new_name, new_mesh))
        } else {
            None
        }
    }

    pub fn join_meshes(&mut self, ids: &[Uuid]) -> Option<Uuid> {
        if ids.len() < 2 { return None; }

        let mut scene = self.scene.write();
        let first = scene.objects.get(&ids[0])?.clone();
        let mut merged = first.mesh;

        for &id in &ids[1..] {
            if let Some(obj) = scene.objects.get(&id) {
                let offset = merged.vertices.len() as u32;
                for vert in &obj.mesh.vertices {
                    merged.add_vertex(vert.position);
                }
                for edge in &obj.mesh.edges {
                    merged.add_edge(
                        edge.vert_ids[0] + offset,
                        edge.vert_ids[1] + offset,
                    );
                }
                for face in &obj.mesh.faces {
                    let new_ids: Vec<u32> = face.vertex_ids.iter()
                        .map(|id| id + offset)
                        .collect();
                    merged.add_face(new_ids);
                }
            }
        }

        merged.recalculate_normals();
        let name = format!("Joined_{}", ids.len());
        let new_id = Uuid::new_v4();
        let obj = SceneObject::new(new_id, name, merged);
        scene.objects.insert(new_id, obj);

        for &id in ids {
            scene.objects.remove(&id);
            self.modifier_stacks.remove(&id);
            self.materials.remove(&id);
        }

        Some(new_id)
    }

    pub fn separate_selected(&mut self, id: &Uuid, vertex_ids: &[u32]) -> Option<Uuid> {
        let scene = self.scene.read();
        let obj = scene.objects.get(id)?;
        let mut new_mesh = Mesh::new();
        let id_map: Vec<u32> = vertex_ids.iter().map(|&vid| {
            new_mesh.add_vertex(obj.mesh.vertices[vid as usize].position)
        }).collect();

        for face in &obj.mesh.faces {
            let new_face_ids: Vec<u32> = face.vertex_ids.iter()
                .filter(|vid| vertex_ids.contains(vid))
                .map(|vid| {
                    let idx = vertex_ids.iter().position(|v| v == vid).unwrap();
                    id_map[idx]
                })
                .collect();
            if new_face_ids.len() >= 3 {
                new_mesh.add_face(new_face_ids);
            }
        }

        new_mesh.recalculate_normals();
        drop(scene);
        Some(self.add_mesh("Separated", new_mesh))
    }

    pub fn merge_vertices(&mut self, id: &Uuid, vertex_ids: &[u32], distance: f32) {
        let mut scene = self.scene.write();
        if let Some(obj) = scene.objects.get_mut(id) {
            let center: glam::Vec3 = vertex_ids.iter()
                .map(|&vid| obj.mesh.vertices[vid as usize].position)
                .sum::<glam::Vec3>() / vertex_ids.len() as f32;

            for &vid in vertex_ids {
                obj.mesh.vertices[vid as usize].position = center;
            }
            obj.mesh.recalculate_normals();
        }
    }

    pub fn fill_holes(&mut self, id: &Uuid) {
        let mut scene = self.scene.write();
        if let Some(obj) = scene.objects.get_mut(id) {
            obj.mesh.recalculate_normals();
        }
    }

    pub fn flip_normals(&mut self, id: &Uuid) {
        let mut scene = self.scene.write();
        if let Some(obj) = scene.objects.get_mut(id) {
            for vert in &mut obj.mesh.vertices {
                vert.normal = -vert.normal;
            }
            for face in &mut obj.mesh.faces {
                face.normal = -face.normal;
                face.vertex_ids.reverse();
            }
        }
    }

    pub fn apply_modifiers(&mut self, id: &Uuid) {
        if let Some(stack) = self.modifier_stacks.get(id) {
            let scene = self.scene.read();
            if let Some(obj) = scene.objects.get(id) {
                let result = stack.apply_all(&obj.mesh);
                drop(scene);
                let mut scene = self.scene.write();
                if let Some(obj) = scene.objects.get_mut(id) {
                    obj.mesh = result;
                }
                self.modifier_stacks.get_mut(id).unwrap().modifiers.clear();
            }
        }
    }

    pub fn recalculate_all_normals(&mut self) {
        let mut scene = self.scene.write();
        for obj in scene.objects.values_mut() {
            obj.mesh.recalculate_normals();
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}