use glam::{Vec3, Quat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::engine::materials::Material;
use crate::engine::modifiers::ModifierStack;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub objects: Vec<Uuid>,
    pub color: [f32; 3],
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
}

impl Collection {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            visible: true,
            locked: false,
            objects: Vec::new(),
            color: [0.5, 0.5, 0.5],
            parent: None,
            children: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayerState {
    pub active_layer: usize,
    pub layers: Vec<Collection>,
}

impl Default for LayerState {
    fn default() -> Self {
        let root = Collection::new("Scene Collection");
        Self {
            active_layer: 0,
            layers: vec![root],
        }
    }
}

impl LayerState {
    pub fn new() -> Self { Self::default() }

    pub fn add_collection(&mut self, name: &str) -> Uuid {
        let col = Collection::new(name);
        let id = col.id;
        self.layers.push(col);
        id
    }

    pub fn remove_collection(&mut self, id: &Uuid) {
        self.layers.retain(|c| c.id != *id);
    }

    pub fn add_object_to_collection(&mut self, col_id: &Uuid, obj_id: &Uuid) {
        if let Some(col) = self.layers.iter_mut().find(|c| c.id == *col_id) {
            col.objects.push(*obj_id);
        }
    }

    pub fn remove_object_from_collection(&mut self, col_id: &Uuid, obj_id: &Uuid) {
        if let Some(col) = self.layers.iter_mut().find(|c| c.id == *col_id) {
            col.objects.retain(|id| id != obj_id);
        }
    }

    pub fn toggle_visibility(&mut self, id: &Uuid) {
        if let Some(col) = self.layers.iter_mut().find(|c| c.id == *id) {
            col.visible = !col.visible;
        }
    }

    pub fn is_visible(&self, id: &Uuid) -> bool {
        self.layers.iter()
            .find(|c| c.id == *id)
            .map(|c| c.visible)
            .unwrap_or(false)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapGuide {
    pub points: Vec<Vec3>,
    pub lines: Vec<(Vec3, Vec3)>,
    pub visible: bool,
}

impl Default for SnapGuide {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            lines: Vec::new(),
            visible: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Annotation {
    pub id: Uuid,
    pub text: String,
    pub position: Vec3,
    pub color: [f32; 3],
    pub font_size: f32,
    pub visible: bool,
}

impl Annotation {
    pub fn new(text: &str, position: Vec3) -> Self {
        Self {
            id: Uuid::new_v4(),
            text: text.to_string(),
            position,
            color: [1.0, 1.0, 0.0],
            font_size: 14.0,
            visible: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measurement {
    pub id: Uuid,
    pub point_a: Vec3,
    pub point_b: Vec3,
    pub distance: f32,
    pub angle_from: Option<Vec3>,
    pub angle_to: Option<Vec3>,
    pub angle: Option<f32>,
    pub label: String,
    pub color: [f32; 3],
    pub visible: bool,
}

impl Measurement {
    pub fn distance(a: Vec3, b: Vec3) -> Self {
        let dist = a.distance(b);
        Self {
            id: Uuid::new_v4(),
            point_a: a,
            point_b: b,
            distance: dist,
            angle_from: None,
            angle_to: None,
            angle: None,
            label: format!("{:.3}", dist),
            color: [1.0, 0.8, 0.0],
            visible: true,
        }
    }

    pub fn angle(a: Vec3, vertex: Vec3, b: Vec3) -> Self {
        let dir_a = (a - vertex).normalize();
        let dir_b = (b - vertex).normalize();
        let angle = dir_a.angle_between(dir_b).to_degrees();

        Self {
            id: Uuid::new_v4(),
            point_a: a,
            point_b: b,
            distance: a.distance(b),
            angle_from: Some(a),
            angle_to: Some(b),
            angle: Some(angle),
            label: format!("{:.1}°", angle),
            color: [0.0, 0.8, 1.0],
            visible: true,
        }
    }
}

pub struct OverlaySystem {
    pub annotations: Vec<Annotation>,
    pub measurements: Vec<Measurement>,
    pub snap_guide: SnapGuide,
}

impl Default for OverlaySystem {
    fn default() -> Self {
        Self {
            annotations: Vec::new(),
            measurements: Vec::new(),
            snap_guide: SnapGuide::default(),
        }
    }
}

impl OverlaySystem {
    pub fn new() -> Self { Self::default() }

    pub fn add_annotation(&mut self, text: &str, pos: Vec3) -> Uuid {
        let ann = Annotation::new(text, pos);
        let id = ann.id;
        self.annotations.push(ann);
        id
    }

    pub fn add_distance_measure(&mut self, a: Vec3, b: Vec3) -> Uuid {
        let m = Measurement::distance(a, b);
        let id = m.id;
        self.measurements.push(m);
        id
    }

    pub fn add_angle_measure(&mut self, a: Vec3, vertex: Vec3, b: Vec3) -> Uuid {
        let m = Measurement::angle(a, vertex, b);
        let id = m.id;
        self.measurements.push(m);
        id
    }

    pub fn remove_annotation(&mut self, id: &Uuid) {
        self.annotations.retain(|a| a.id != *id);
    }

    pub fn remove_measurement(&mut self, id: &Uuid) {
        self.measurements.retain(|m| m.id != *id);
    }

    pub fn clear_all(&mut self) {
        self.annotations.clear();
        self.measurements.clear();
        self.snap_guide = SnapGuide::default();
    }
}