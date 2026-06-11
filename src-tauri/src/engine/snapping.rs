use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnappingConfig {
    pub enabled: bool,
    pub snap_type: SnapType,
    pub grid_size: f32,
    pub increment: f32,
    pub vertex_tolerance: f32,
    pub edge_tolerance: f32,
    pub angle_snap: f32,
    pub target: SnapTarget,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SnapType {
    None,
    Grid,
    Vertex,
    Edge,
    Face,
    Increment,
    Angle,
    Volume,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SnapTarget {
    Absolute,
    Relative,
    Median,
    Individual,
}

impl Default for SnappingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            snap_type: SnapType::Increment,
            grid_size: 0.1,
            increment: 0.1,
            vertex_tolerance: 0.1,
            edge_tolerance: 0.1,
            angle_snap: 5.0,
            target: SnapTarget::Median,
        }
    }
}

impl SnappingConfig {
    pub fn snap_position(&self, pos: Vec3, targets: &[Vec3]) -> Vec3 {
        if !self.enabled {
            return pos;
        }

        match self.snap_type {
            SnapType::Grid => self.snap_to_grid(pos),
            SnapType::Vertex => self.snap_to_vertex(pos, targets),
            SnapType::Increment => self.snap_to_increment(pos),
            SnapType::Angle => self.snap_angle(pos),
            _ => pos,
        }
    }

    fn snap_to_grid(&self, pos: Vec3) -> Vec3 {
        let g = self.grid_size;
        Vec3::new(
            (pos.x / g).round() * g,
            (pos.y / g).round() * g,
            (pos.z / g).round() * g,
        )
    }

    fn snap_to_vertex(&self, pos: Vec3, targets: &[Vec3]) -> Vec3 {
        let mut closest = pos;
        let mut min_dist = self.vertex_tolerance;

        for target in targets {
            let dist = pos.distance(*target);
            if dist < min_dist {
                min_dist = dist;
                closest = *target;
            }
        }
        closest
    }

    fn snap_to_increment(&self, pos: Vec3) -> Vec3 {
        let inc = self.increment;
        Vec3::new(
            (pos.x / inc).round() * inc,
            (pos.y / inc).round() * inc,
            (pos.z / inc).round() * inc,
        )
    }

    fn snap_angle(&self, pos: Vec3) -> Vec3 {
        let angle = self.angle_snap.to_radians();
        let len = pos.length();
        if len < 1e-6 {
            return pos;
        }
        let theta = (pos.z.atan2(pos.x) / angle).round() * angle;
        let phi = (pos.y / len).clamp(-1.0, 1.0).acos();
        let phi_snapped = (phi / angle).round() * angle;
        Vec3::new(
            len * phi_snapped.sin() * theta.cos(),
            len * phi_snapped.cos(),
            len * phi_snapped.sin() * theta.sin(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProportionalEditing {
    pub enabled: bool,
    pub radius: f32,
    pub falloff_type: FalloffType,
    pub connected_only: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FalloffType {
    Smooth,
    Sphere,
    Root,
    Sharp,
    Linear,
    Constant,
    Random,
}

impl Default for ProportionalEditing {
    fn default() -> Self {
        Self {
            enabled: false,
            radius: 1.0,
            falloff_type: FalloffType::Smooth,
            connected_only: false,
        }
    }
}

impl ProportionalEditing {
    pub fn get_influence(&self, distance: f32) -> f32 {
        if !self.enabled || distance > self.radius {
            return 0.0;
        }

        let t = distance / self.radius;
        match self.falloff_type {
            FalloffType::Smooth => 1.0 - (t * std::f32::consts::PI).cos() * 0.5 + 0.5,
            FalloffType::Sphere => (1.0 - t * t).max(0.0),
            FalloffType::Root => (1.0 - t.sqrt()).max(0.0),
            FalloffType::Sharp => (1.0 - t).powi(2).max(0.0),
            FalloffType::Linear => (1.0 - t).max(0.0),
            FalloffType::Constant => 1.0,
            FalloffType::Random => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                ((distance * 1000.0) as u32).hash(&mut hasher);
                (hasher.finish() % 1000) as f32 / 1000.0
            }
        }
    }
}