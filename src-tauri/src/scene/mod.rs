use crate::engine::commands::ConstraintType;
use crate::mesh::Mesh;
use glam::{Vec3, Quat};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SceneObject {
    pub id: Uuid,
    pub name: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub mesh: Mesh,
    pub visible: bool,
    pub locked: bool,
    pub constraints: Vec<ConstraintType>,
}

impl SceneObject {
    pub fn new(id: Uuid, name: String, mesh: Mesh) -> Self {
        Self {
            id,
            name,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            mesh,
            visible: true,
            locked: false,
            constraints: Vec::new(),
        }
    }

    pub fn model_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(5.0, 5.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 45.0,
            near: 0.1,
            far: 1000.0,
            aspect: 16.0 / 9.0,
        }
    }
}

impl Camera {
    pub fn view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn projection_matrix(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect,
            self.near,
            self.far,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub size: f32,
    pub subdivisions: u32,
    pub visible: bool,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            size: 10.0,
            subdivisions: 10,
            visible: true,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Scene {
    pub objects: HashMap<Uuid, SceneObject>,
    pub camera: Camera,
    pub grid: Grid,
    pub ambient_light: Vec3,
    pub lights: Vec<Light>,
}

#[derive(Clone, Debug)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub light_type: LightType,
}

#[derive(Clone, Debug)]
pub enum LightType {
    Point,
    Directional,
    Spot,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            camera: Camera::default(),
            grid: Grid::default(),
            ambient_light: Vec3::new(0.2, 0.2, 0.2),
            lights: vec![Light {
                position: Vec3::new(5.0, 10.0, 5.0),
                color: Vec3::ONE,
                intensity: 1.0,
                light_type: LightType::Directional,
            }],
        }
    }

    pub fn add_object(&mut self, obj: SceneObject) {
        self.objects.insert(obj.id, obj);
    }

    pub fn remove_object(&mut self, id: &Uuid) -> Option<SceneObject> {
        self.objects.remove(id)
    }

    pub fn get_object(&self, id: &Uuid) -> Option<&SceneObject> {
        self.objects.get(id)
    }

    pub fn get_object_mut(&mut self, id: &Uuid) -> Option<&mut SceneObject> {
        self.objects.get_mut(id)
    }
}