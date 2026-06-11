use crate::mesh::bmesh::{BMesh};
use glam::{Vec3, Quat};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SculptBrush {
    pub brush_type: BrushType,
    pub radius: f32,
    pub strength: f32,
    pub autosmooth: f32,
    pub symmetry: Symmetry,
    pub spacing: f32,
    pub use_accumulate: bool,
    pub use_frontface: bool,
    pub use_edge_angle: bool,
    pub edge_angle_limit: f32,
    pub normal_weight: f32,
    pub plane_offset: f32,
    pub sculpt_plane: SculptPlane,
    pub mask_tool: MaskTool,
    pub detail_type: DetailType,
    pub target_detail: f32,
    pub constant_detail: f32,
    pub resolution: u32,
    pub smooth_iterations: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrushType {
    Draw,
    DrawSharp,
    Clay,
    ClayStrips,
    ClayThumb,
    Smooth,
    Flatten,
    Fill,
    Scrape,
    Pinch,
    Inflate,
    Blob,
    Grab,
    SnakeHook,
    Rotate,
    Twist,
    Nudge,
    Thumb,
    Smear,
    Simplify,
    MultiresDisplaceEraser,
    Paint,
    Mask,
    Blur,
    Average,
    Gravity,
    SmoothFill,
    SurfaceDraw,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Symmetry {
    None,
    X,
    Y,
    Z,
    XY,
    XZ,
    YZ,
    XYZ,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SculptPlane {
    View,
    Surface,
    AreaPlane,
    Tangent,
    Stroke,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MaskTool {
    Draw,
    Smooth,
    Fill,
    Invert,
    Box,
    Lasso,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetailType {
    Relative,
    Constant,
    Manual,
    Brush,
}

impl Default for SculptBrush {
    fn default() -> Self {
        Self {
            brush_type: BrushType::Draw,
            radius: 0.5,
            strength: 0.5,
            autosmooth: 0.0,
            symmetry: Symmetry::X,
            spacing: 10.0,
            use_accumulate: false,
            use_frontface: true,
            use_edge_angle: false,
            edge_angle_limit: 70.0,
            normal_weight: 0.5,
            plane_offset: 0.0,
            sculpt_plane: SculptPlane::Surface,
            mask_tool: MaskTool::Draw,
            detail_type: DetailType::Relative,
            target_detail: 0.9,
            constant_detail: 0.5,
            resolution: 128,
            smooth_iterations: 2,
        }
    }
}

pub struct SculptContext {
    pub brush: SculptBrush,
    pub symm_pos: Vec<f32>,
    pub last_pos: Vec3,
    pub pen_flip: bool,
    pub height: f32,
    pub plane_norm: Vec3,
}

pub struct DyntopoContext {
    pub detail_size: f32,
    pub symmetric: bool,
    pub threshold: f32,
    pub current_detail: f32,
}

impl Default for DyntopoContext {
    fn default() -> Self {
        Self {
            detail_size: 12.0,
            symmetric: true,
            threshold: 0.5,
            current_detail: 0.0,
        }
    }
}

pub struct MultiresLevel {
    pub levels: Vec<Vec<Vec3>>,
    pub current_level: u32,
    pub sculpt_level: u32,
    pub render_level: u32,
    pub total_levels: u32,
}

impl Default for MultiresLevel {
    fn default() -> Self {
        Self {
            levels: Vec::new(),
            current_level: 0,
            sculpt_level: 0,
            render_level: 0,
            total_levels: 0,
        }
    }
}

impl MultiresLevel {
    pub fn add_level(&mut self, positions: Vec<Vec3>) {
        self.levels.push(positions);
        self.total_levels += 1;
    }

    pub fn get_displaced_positions(&self, base_positions: &[Vec3], level: u32) -> Vec<Vec3> {
        let mut result = base_positions.to_vec();
        for l in 0..=level as usize {
            if l < self.levels.len() {
                for (i, pos) in self.levels[l].iter().enumerate() {
                    if i < result.len() {
                        result[i] += *pos;
                    }
                }
            }
        }
        result
    }

    pub fn subdivide(&mut self, positions: &[Vec3]) {
        let mut new_positions = Vec::new();
        for pos in positions {
            new_positions.push(*pos * 0.25);
        }
        self.add_level(new_positions);
    }

    pub fn unsubdivide(&mut self) {
        if !self.levels.is_empty() {
            self.levels.pop();
            self.total_levels = self.levels.len() as u32;
        }
    }
}

pub struct SculptEngine {
    pub brush: SculptBrush,
    pub dyntopo: DyntopoContext,
    pub multires: MultiresLevel,
    pub mask_data: Vec<f32>,
    pub face_set_data: Vec<u32>,
    pub cursor_pos: Vec3,
    pub symmetry_state: [bool; 3],
    pub stroke_count: u32,
}

impl Default for SculptEngine {
    fn default() -> Self {
        Self {
            brush: SculptBrush::default(),
            dyntopo: DyntopoContext::default(),
            multires: MultiresLevel::default(),
            mask_data: Vec::new(),
            face_set_data: Vec::new(),
            cursor_pos: Vec3::ZERO,
            symmetry_state: [false; 3],
            stroke_count: 0,
        }
    }
}

impl SculptEngine {
    pub fn new() -> Self { Self::default() }

    pub fn apply_brush(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        match self.brush.brush_type {
            BrushType::Draw => self.brush_draw(mesh, center, radius, strength, direction),
            BrushType::DrawSharp => self.brush_draw_sharp(mesh, center, radius, strength, direction),
            BrushType::Clay => self.brush_clay(mesh, center, radius, strength, direction),
            BrushType::ClayStrips => self.brush_clay_strips(mesh, center, radius, strength, direction),
            BrushType::Smooth => self.brush_smooth(mesh, center, radius, strength),
            BrushType::Flatten => self.brush_flatten(mesh, center, radius, strength),
            BrushType::Fill => self.brush_fill(mesh, center, radius, strength, direction),
            BrushType::Scrape => self.brush_scrape(mesh, center, radius, strength, direction),
            BrushType::Pinch => self.brush_pinch(mesh, center, radius, strength, direction),
            BrushType::Inflate => self.brush_inflate(mesh, center, radius, strength, direction),
            BrushType::Grab => self.brush_grab(mesh, center, radius, strength, Vec3::Y * direction),
            BrushType::SnakeHook => self.brush_snake_hook(mesh, center, radius, strength, Vec3::Y * direction),
            BrushType::Twist => self.brush_twist(mesh, center, radius, strength, direction),
            BrushType::Nudge => self.brush_nudge(mesh, center, radius, strength, Vec3::X * direction),
            BrushType::Simplify => self.brush_simplify(mesh, center, radius),
            BrushType::Mask => self.brush_mask(mesh, center, radius, strength),
            BrushType::Blur => self.brush_blur(mesh, center, radius, strength),
            BrushType::Average => self.brush_average(mesh, center, radius, strength),
            BrushType::Gravity => self.brush_gravity(mesh, center, radius, strength),
            _ => {}
        }
    }

    fn get_influence(&self, pos: Vec3, center: Vec3, radius: f32) -> f32 {
        let dist = pos.distance(center);
        if dist > radius { return 0.0; }
        let t = dist / radius;
        match self.brush.brush_type {
            BrushType::Smooth | BrushType::Average | BrushType::Blur => (1.0 - t * t).max(0.0),
            BrushType::Pinch | BrushType::DrawSharp => (1.0 - t).powi(3).max(0.0),
            BrushType::Grab | BrushType::SnakeHook => (1.0 - t).powi(2).max(0.0),
            _ => (1.0 - t * t).max(0.0),
        }
    }

    fn brush_draw(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let offset = vert.normal * strength * influence * direction * self.brush.radius;
                vert.position += offset;
            }
        }
    }

    fn brush_draw_sharp(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let sharp = influence * influence;
                let offset = vert.normal * strength * sharp * direction * self.brush.radius;
                vert.position += offset;
            }
        }
    }

    fn brush_clay(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let offset = vert.normal * strength * influence * direction * self.brush.radius * 0.5;
                vert.position += offset;
            }
        }
    }

    fn brush_clay_strips(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let strip = influence.sin() * influence;
                let offset = vert.normal * strength * strip * direction * self.brush.radius;
                vert.position += offset;
            }
        }
    }

    fn brush_smooth(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32) {
        let original: Vec<Vec3> = mesh.verts.iter().map(|v| v.position).collect();
        let avgs: Vec<Vec3> = mesh.verts.iter().map(|vert| {
            let edges = mesh.vert_edges(vert.id);
            let neighbors: Vec<Vec3> = edges.iter()
                .filter_map(|eid| {
                    let e = mesh.edge(*eid);
                    if e.vert == vert.id {
                        e.next.map(|n| original[mesh.edge(n).vert.0 as usize])
                    } else {
                        Some(original[e.vert.0 as usize])
                    }
                })
                .collect();
            if neighbors.is_empty() {
                vert.position
            } else {
                neighbors.iter().sum::<Vec3>() / neighbors.len() as f32
            }
        }).collect();
        for (i, vert) in mesh.verts.iter_mut().enumerate() {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                vert.position = vert.position.lerp(avgs[i], strength * influence);
            }
        }
    }

    fn brush_flatten(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32) {
        let avg_normal: Vec3 = mesh.verts.iter()
            .filter(|v| !v.hide && v.position.distance(center) < radius)
            .map(|v| v.normal)
            .sum::<Vec3>()
            .normalize_or_zero();

        let avg_pos: Vec3 = mesh.verts.iter()
            .filter(|v| !v.hide && v.position.distance(center) < radius)
            .map(|v| v.position)
            .sum::<Vec3>();
        let count = mesh.verts.iter()
            .filter(|v| !v.hide && v.position.distance(center) < radius)
            .count() as f32;
        let avg_pos = if count > 0.0 { avg_pos / count } else { center };

        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let projected = vert.position - avg_normal * vert.position.dot(avg_normal);
                let target = projected.lerp(avg_pos, 0.5);
                vert.position = vert.position.lerp(target, strength * influence);
            }
        }
    }

    fn brush_fill(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        let max_height: f32 = mesh.verts.iter()
            .filter(|v| !v.hide && v.position.distance(center) < radius)
            .map(|v| v.position.dot(v.normal))
            .fold(f32::NEG_INFINITY, f32::max);

        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let height = vert.position.dot(vert.normal);
                let target_height = max_height;
                let diff = target_height - height;
                if diff > 0.0 {
                    vert.position += vert.normal * diff * strength * influence * direction;
                }
            }
        }
    }

    fn brush_scrape(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        let min_height: f32 = mesh.verts.iter()
            .filter(|v| !v.hide && v.position.distance(center) < radius)
            .map(|v| v.position.dot(v.normal))
            .fold(f32::INFINITY, f32::min);

        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let height = vert.position.dot(vert.normal);
                let target_height = min_height;
                let diff = target_height - height;
                if diff < 0.0 {
                    vert.position += vert.normal * diff * strength * influence * direction;
                }
            }
        }
    }

    fn brush_pinch(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let to_center = (center - vert.position).normalize_or_zero();
                let offset = to_center * strength * influence * direction * self.brush.radius * 0.3;
                vert.position += offset;
            }
        }
    }

    fn brush_inflate(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let offset = vert.normal * strength * influence * direction * self.brush.radius * 0.5;
                vert.position += offset;
            }
        }
    }

    fn brush_grab(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, offset: Vec3) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                vert.position += offset * strength * influence;
            }
        }
    }

    fn brush_snake_hook(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, offset: Vec3) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let falloff = influence.powi(3);
                vert.position += offset * strength * falloff * self.brush.radius;
            }
        }
    }

    fn brush_twist(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: f32) {
        let axis = Vec3::Y;
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let to_vert = vert.position - center;
                let angle = strength * influence * direction * 0.5;
                let rotated = Quat::from_axis_angle(axis, angle) * to_vert;
                vert.position = center + rotated;
            }
        }
    }

    fn brush_nudge(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32, direction: Vec3) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                vert.position += direction * strength * influence * self.brush.radius * 0.3;
            }
        }
    }

    fn brush_simplify(&self, mesh: &mut BMesh, center: Vec3, radius: f32) {
        let _ = (mesh, center, radius);
    }

    fn brush_mask(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32) {
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                let _ = (vert, strength);
            }
        }
    }

    fn brush_blur(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32) {
        self.brush_smooth(mesh, center, radius, strength * 0.5);
    }

    fn brush_average(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32) {
        self.brush_smooth(mesh, center, radius, strength * 0.8);
    }

    fn brush_gravity(&self, mesh: &mut BMesh, center: Vec3, radius: f32, strength: f32) {
        let gravity = Vec3::new(0.0, -0.1, 0.0);
        for vert in &mut mesh.verts {
            if vert.hide { continue; }
            let influence = self.get_influence(vert.position, center, radius);
            if influence > 0.0 {
                vert.position += gravity * strength * influence;
            }
        }
    }

    pub fn apply_symmetry(&self, mesh: &mut BMesh) {
        if self.brush.symmetry == Symmetry::None { return; }

        let verts_snapshot: Vec<(Vec3, Vec3)> = mesh.verts.iter()
            .map(|v| (v.position, v.normal))
            .collect();

        for (i, vert) in mesh.verts.iter_mut().enumerate() {
            if vert.hide { continue; }
            let pos = verts_snapshot[i].0;

            match self.brush.symmetry {
                Symmetry::X => {
                    if pos.x < -0.001 {
                        let sym_pos = Vec3::new(-pos.x, pos.y, pos.z);
                        vert.position = sym_pos;
                    }
                }
                Symmetry::Y => {
                    if pos.y < -0.001 {
                        let sym_pos = Vec3::new(pos.x, -pos.y, pos.z);
                        vert.position = sym_pos;
                    }
                }
                Symmetry::Z => {
                    if pos.z < -0.001 {
                        let sym_pos = Vec3::new(pos.x, pos.y, -pos.z);
                        vert.position = sym_pos;
                    }
                }
                Symmetry::XY => {
                    if pos.x < -0.001 || pos.y < -0.001 {
                        vert.position = Vec3::new(pos.x.abs(), pos.y.abs(), pos.z);
                    }
                }
                Symmetry::XZ => {
                    if pos.x < -0.001 || pos.z < -0.001 {
                        vert.position = Vec3::new(pos.x.abs(), pos.y, pos.z.abs());
                    }
                }
                Symmetry::YZ => {
                    if pos.y < -0.001 || pos.z < -0.001 {
                        vert.position = Vec3::new(pos.x, pos.y.abs(), pos.z.abs());
                    }
                }
                Symmetry::XYZ => {
                    vert.position = Vec3::new(pos.x.abs(), pos.y.abs(), pos.z.abs());
                }
                _ => {}
            }
        }
    }
}
