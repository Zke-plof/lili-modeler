use glam::{Vec3, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderEngine {
    pub engine_type: RenderEngineType,
    pub samples: u32,
    pub max_bounces: u32,
    pub diffuse_bounces: u32,
    pub glossy_bounces: u32,
    pub transmission_bounces: u32,
    pub volume_bounces: u32,
    pub transparent_bounces: u32,
    pub use_denoising: bool,
    pub denoiser: Denoiser,
    pub denoising_strength: f32,
    pub use_adaptive_sampling: bool,
    pub adaptive_threshold: f32,
    pub adaptive_min_samples: u32,
    pub use_clamp_direct: bool,
    pub clamp_direct: f32,
    pub use_clamp_indirect: bool,
    pub clamp_indirect: f32,
    pub use_light_tree: bool,
    pub use_persistent_data: bool,
    pub use_use_spatial_splits: bool,
    pub use_two_sided: bool,
    pub filter_width: f32,
    pub pixel_filter: PixelFilter,
    pub use_poisson_disk: bool,
    pub use_sample_all_direct_lights: bool,
    pub use_sample_all_indirect_lights: bool,
    pub use_use_bsdf_flags: bool,
    pub use_use_mis: bool,
    pub use_volume_step_rate: f32,
    pub use_volume_max_steps: u32,
    pub use_volume_homogeneous: bool,
    pub use_volume_density_threshold: f32,
    pub use_use_portal: bool,
    pub use_use_light_sampling: bool,
    pub use_use_light_tree_bvh: bool,
    pub use_use_multiple_importance: bool,
    pub use_use_volume_light_sampling: bool,
    pub use_use_volume_direct_light_sampling: bool,
    pub use_use_volume_indirect_light_sampling: bool,
    pub use_use_volume_multiple_importance: bool,
    pub use_use_volume_mis_strength: f32,
    pub use_use_volume_step_rate: f32,
    pub use_use_volume_max_steps: u32,
    pub use_use_volume_homogeneous: bool,
    pub use_use_volume_density_threshold: f32,
    pub use_use_volume_step_rate_light: f32,
    pub use_use_volume_max_steps_light: u32,
    pub use_use_volume_step_rate_transmission: f32,
    pub use_use_volume_max_steps_transmission: u32,
    pub use_use_volume_step_rate_shadow: f32,
    pub use_use_volume_max_steps_shadow: u32,
    pub use_use_volume_step_rate_transparent: f32,
    pub use_use_volume_max_steps_transparent: u32,
    pub use_use_volume_step_rate_ao: f32,
    pub use_use_volume_max_steps_ao: u32,
    pub use_use_volume_step_rate_sample_all: f32,
    pub use_use_volume_max_steps_sample_all: u32,
    pub use_use_volume_step_rate_sample_direct: f32,
    pub use_use_volume_max_steps_sample_direct: u32,
    pub use_use_volume_step_rate_sample_indirect: f32,
    pub use_use_volume_max_steps_sample_indirect: u32,
    pub use_use_volume_step_rate_sample_volume: f32,
    pub use_use_volume_max_steps_sample_volume: u32,
    pub use_use_volume_step_rate_sample_transmission: f32,
    pub use_use_volume_max_steps_sample_transmission: u32,
    pub use_use_volume_step_rate_sample_shadow: f32,
    pub use_use_volume_max_steps_sample_shadow: u32,
    pub use_use_volume_step_rate_sample_transparent: f32,
    pub use_use_volume_max_steps_sample_transparent: u32,
    pub use_use_volume_step_rate_sample_ao: f32,
    pub use_use_volume_max_steps_sample_ao: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RenderEngineType {
    PathTracer,
    Eevee,
    Workbench,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Denoiser {
    OpenImageDenoise,
    OptiX,
    Nlm,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PixelFilter {
    Box,
    Gaussian,
    BlackmanHarris,
    Mitchell,
    Lanczos,
    CatmullRom,
}

impl Default for RenderEngine {
    fn default() -> Self {
        Self {
            engine_type: RenderEngineType::PathTracer,
            samples: 4096,
            max_bounces: 8,
            diffuse_bounces: 4,
            glossy_bounces: 4,
            transmission_bounces: 8,
            volume_bounces: 2,
            transparent_bounces: 8,
            use_denoising: true,
            denoiser: Denoiser::OpenImageDenoise,
            denoising_strength: 0.5,
            use_adaptive_sampling: true,
            adaptive_threshold: 0.01,
            adaptive_min_samples: 0,
            use_clamp_direct: false,
            clamp_direct: 0.0,
            use_clamp_indirect: false,
            clamp_indirect: 10.0,
            use_light_tree: true,
            use_persistent_data: true,
            use_use_spatial_splits: true,
            use_two_sided: true,
            filter_width: 1.5,
            pixel_filter: PixelFilter::Gaussian,
            use_poisson_disk: true,
            use_sample_all_direct_lights: true,
            use_sample_all_indirect_lights: true,
            use_use_bsdf_flags: true,
            use_use_mis: true,
            use_volume_step_rate: 1.0,
            use_volume_max_steps: 256,
            use_volume_homogeneous: false,
            use_volume_density_threshold: 0.001,
            use_use_portal: true,
            use_use_light_sampling: true,
            use_use_light_tree_bvh: true,
            use_use_multiple_importance: true,
            use_use_volume_light_sampling: true,
            use_use_volume_direct_light_sampling: true,
            use_use_volume_indirect_light_sampling: true,
            use_use_volume_multiple_importance: true,
            use_use_volume_mis_strength: 1.0,
            use_use_volume_step_rate: 1.0,
            use_use_volume_max_steps: 256,
            use_use_volume_homogeneous: false,
            use_use_volume_density_threshold: 0.001,
            use_use_volume_step_rate_light: 1.0,
            use_use_volume_max_steps_light: 256,
            use_use_volume_step_rate_transmission: 1.0,
            use_use_volume_max_steps_transmission: 256,
            use_use_volume_step_rate_shadow: 1.0,
            use_use_volume_max_steps_shadow: 256,
            use_use_volume_step_rate_transparent: 1.0,
            use_use_volume_max_steps_transparent: 256,
            use_use_volume_step_rate_ao: 1.0,
            use_use_volume_max_steps_ao: 256,
            use_use_volume_step_rate_sample_all: 1.0,
            use_use_volume_max_steps_sample_all: 256,
            use_use_volume_step_rate_sample_direct: 1.0,
            use_use_volume_max_steps_sample_direct: 256,
            use_use_volume_step_rate_sample_indirect: 1.0,
            use_use_volume_max_steps_sample_indirect: 256,
            use_use_volume_step_rate_sample_volume: 1.0,
            use_use_volume_max_steps_sample_volume: 256,
            use_use_volume_step_rate_sample_transmission: 1.0,
            use_use_volume_max_steps_sample_transmission: 256,
            use_use_volume_step_rate_sample_shadow: 1.0,
            use_use_volume_max_steps_sample_shadow: 256,
            use_use_volume_step_rate_sample_transparent: 1.0,
            use_use_volume_max_steps_sample_transparent: 256,
            use_use_volume_step_rate_sample_ao: 1.0,
            use_use_volume_max_steps_sample_ao: 256,
        }
    }
}

pub struct PathTracer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec3>,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub camera: Camera,
    pub world: World,
    pub objects: Vec<TraceObject>,
    pub lights: Vec<TraceLight>,
    pub accumulated_samples: u32,
    pub running: bool,
    pub tile_size: u32,
    pub current_tile: (u32, u32),
    pub total_tiles: (u32, u32),
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focal_distance: f32,
    pub lens: CameraLens,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CameraLens {
    Perspective,
    Orthographic,
    Panoramic,
    OrthographicFull,
    Stereo3D,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0),
            look_at: Vec3::ZERO,
            up: Vec3::Y,
            fov: 40.0,
            aspect: 1.0,
            aperture: 0.0,
            focal_distance: 5.0,
            lens: CameraLens::Perspective,
        }
    }
}

pub struct World {
    pub color: Vec3,
    pub strength: f32,
    pub use_mis: bool,
    pub use_solid: bool,
    pub surface: WorldSurface,
}

#[derive(Clone, Debug)]
pub enum WorldSurface {
    Background { color: Vec3, strength: f32 },
    EnvironmentTexture { path: String, strength: f32 },
    SkyTexture { sky_type: SkyType },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkyType {
    Preetham,
    HosekWilkie,
    Nishita,
}

impl Default for World {
    fn default() -> Self {
        Self {
            color: Vec3::new(0.05, 0.05, 0.05),
            strength: 1.0,
            use_mis: true,
            use_solid: true,
            surface: WorldSurface::Background {
                color: Vec3::new(0.05, 0.05, 0.05),
                strength: 1.0,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct TraceObject {
    pub position: Vec3,
    pub rotation: glam::Quat,
    pub scale: Vec3,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub material: TraceMaterial,
    pub visible: bool,
    pub shadow: bool,
}

#[derive(Clone, Debug)]
pub struct TraceMaterial {
    pub material_type: TraceMaterialType,
    pub base_color: Vec3,
    pub metallic: f32,
    pub roughness: f32,
    pub specular: f32,
    pub ior: f32,
    pub transmission: f32,
    pub emission: Vec3,
    pub emission_strength: f32,
    pub alpha: f32,
    pub subsurface: f32,
    pub subsurface_radius: Vec3,
    pub subsurface_color: Vec3,
    pub anisotropic: f32,
    pub anisotropic_rotation: f32,
    pub sheen: f32,
    pub sheen_tint: f32,
    pub clearcoat: f32,
    pub clearcoat_roughness: f32,
    pub thin_film_thickness: f32,
    pub thin_film_ior: f32,
    pub density: f32,
    pub volume_density: f32,
    volume_color: Vec3,
    volume_anisotropy: f32,
    pub blend_method: MaterialBlendMethod,
    pub use_screen_space_refraction: bool,
    pub use_backface_culling: bool,
    pub use_subsurface_translucency: bool,
    pub use_nodes: bool,
    pub blend_method_render: MaterialBlendMethod,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraceMaterialType {
    Principled,
    Diffuse,
    Glossy,
    Glass,
    Emission,
    Transparent,
    Refraction,
    Translucent,
    Velvet,
    Subsurface,
    Toon,
    Volume,
    Mix,
    Add,
    Wireframe,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterialBlendMethod {
    Opaque,
    AlphaBlend,
    AlphaHashed,
    AlphaClip,
}

#[derive(Clone, Debug)]
pub struct TraceLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub color: Vec3,
    pub strength: f32,
    pub size: f32,
    pub light_type: TraceLightType,
    pub use_mis: bool,
    pub use_shadow: bool,
    pub shadow_soft_size: f32,
    pub contact_shadow: bool,
    pub contact_shadow_distance: f32,
    pub contact_shadow_bias: f32,
    pub contact_shadow_thickness: f32,
    pub diffuse_factor: f32,
    pub specular_factor: f32,
    pub volume_factor: f32,
    pub max_bounces: u32,
    pub use_multiple_importance: bool,
    pub use_shadow_catcher: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraceLightType {
    Point,
    Sun,
    Spot,
    Area,
}

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub t_min: f32,
    pub t_max: f32,
    pub depth: u32,
    pub time: f32,
    pub importance: f32,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            direction: Vec3::Z,
            t_min: 0.001,
            t_max: f32::INFINITY,
            depth: 0,
            time: 0.0,
            importance: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub material: Option<usize>,
    pub object_index: usize,
    pub face_index: u32,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: f32::INFINITY,
            point: Vec3::ZERO,
            normal: Vec3::Y,
            uv: Vec2::ZERO,
            material: None,
            object_index: 0,
            face_index: 0,
            front_face: true,
        }
    }
}

pub struct BVHNode {
    pub bounds_min: Vec3,
    pub bounds_max: Vec3,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub object_index: Option<usize>,
    pub split_axis: u8,
}

pub struct BVH {
    pub nodes: Vec<BVHNode>,
    pub root: Option<usize>,
}

impl Default for BVH {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }
}

impl BVH {
    pub fn new() -> Self { Self::default() }

    pub fn build(&mut self, objects: &[TraceObject]) {
        if objects.is_empty() { return; }
        self.nodes.clear();
        self.root = Some(self.build_recursive(objects, 0, objects.len()));
    }

    fn build_recursive(&mut self, objects: &[TraceObject], start: usize, end: usize) -> usize {
        let mut bounds_min = Vec3::splat(f32::INFINITY);
        let mut bounds_max = Vec3::splat(f32::NEG_INFINITY);

        for obj in &objects[start..end] {
            for v in &obj.vertices {
                let world_v = obj.position + *v;
                bounds_min = bounds_min.min(world_v);
                bounds_max = bounds_max.max(world_v);
            }
        }

        let node_idx = self.nodes.len();
        self.nodes.push(BVHNode {
            bounds_min,
            bounds_max,
            left: None,
            right: None,
            object_index: None,
            split_axis: 0,
        });

        if end - start == 1 {
            self.nodes[node_idx].object_index = Some(start);
            return node_idx;
        }

        let extent = bounds_max - bounds_min;
        let split_axis = if extent.x > extent.y && extent.x > extent.z { 0 }
            else if extent.y > extent.z { 1 } else { 2 };

        let mid = start + (end - start) / 2;
        let _ = split_axis;

        let left = self.build_recursive(objects, start, mid);
        let right = self.build_recursive(objects, mid, end);

        self.nodes[node_idx].left = Some(left);
        self.nodes[node_idx].right = Some(right);
        self.nodes[node_idx].split_axis = split_axis as u8;

        node_idx
    }

    pub fn intersect(&self, ray: &Ray, objects: &[TraceObject]) -> Option<HitRecord> {
        self.root.map(|root| self.intersect_node(root, ray, objects)).flatten()
    }

    fn intersect_node(&self, node_idx: usize, ray: &Ray, objects: &[TraceObject]) -> Option<HitRecord> {
        let node = &self.nodes[node_idx];

        if !self.intersect_bounds(node, ray) {
            return None;
        }

        if let Some(obj_idx) = node.object_index {
            return self.intersect_object(&objects[obj_idx], ray, obj_idx);
        }

        let mut closest = None;
        if let Some(left) = node.left {
            closest = self.intersect_node(left, ray, objects);
        }
        if let Some(right) = node.right {
            let right_hit = self.intersect_node(right, ray, objects);
            if let Some(rh) = right_hit {
                if closest.as_ref().map_or(true, |ch| rh.t < ch.t) {
                    closest = Some(rh);
                }
            }
        }

        closest
    }

    fn intersect_bounds(&self, node: &BVHNode, ray: &Ray) -> bool {
        let inv_dir = Vec3::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        let mut tmin = (node.bounds_min - ray.origin) * inv_dir;
        let mut tmax = (node.bounds_max - ray.origin) * inv_dir;

        let t0 = tmin.min(tmax);
        let t1 = tmin.max(tmax);

        let tmin = t0.x.max(t0.y).max(t0.z);
        let tmax = t1.x.min(t1.y).min(t1.z);

        tmax >= tmin.max(0.0)
    }

    fn intersect_object(&self, obj: &TraceObject, ray: &Ray, obj_idx: usize) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;

        for i in (0..obj.indices.len()).step_by(3) {
            let v0 = obj.position + obj.vertices[obj.indices[i] as usize];
            let v1 = obj.position + obj.vertices[obj.indices[i + 1] as usize];
            let v2 = obj.position + obj.vertices[obj.indices[i + 2] as usize];

            if let Some(hit) = self.intersect_triangle(ray, v0, v1, v2) {
                if hit.t > ray.t_min && hit.t < ray.t_max {
                if closest.as_ref().map_or(true, |c: &HitRecord| hit.t < c.t) {
                        let normal = (v1 - v0).cross(v2 - v0).normalize();
                        let front_face = ray.direction.dot(normal) < 0.0;
                        let normal = if front_face { normal } else { -normal };

                        closest = Some(HitRecord {
                            t: hit.t,
                            point: ray.origin + ray.direction * hit.t,
                            normal,
                            uv: Vec2::ZERO,
                            material: None,
                            object_index: obj_idx,
                            face_index: (i / 3) as u32,
                            front_face,
                        });
                    }
                }
            }
        }

        closest
    }

    fn intersect_triangle(&self, ray: &Ray, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<HitRecord> {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -0.00001 && a < 0.00001 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > 0.00001 {
            Some(HitRecord {
                t,
                point: ray.origin + ray.direction * t,
                normal: edge1.cross(edge2).normalize(),
                uv: Vec2::new(u, v),
                material: None,
                object_index: 0,
                face_index: 0,
                front_face: true,
            })
        } else {
            None
        }
    }
}

impl PathTracer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Vec3::ZERO; (width * height) as usize],
            samples_per_pixel: 16,
            max_depth: 8,
            camera: Camera::default(),
            world: World::default(),
            objects: Vec::new(),
            lights: Vec::new(),
            accumulated_samples: 0,
            running: false,
            tile_size: 64,
            current_tile: (0, 0),
            total_tiles: ((width + 63) / 64, (height + 63) / 64),
        }
    }

    pub fn render_pixel(&self, x: u32, y: u32) -> Vec3 {
        let mut color = Vec3::ZERO;
        for _ in 0..self.samples_per_pixel {
            let u = (x as f32 + rand_f32()) / self.width as f32;
            let v = (y as f32 + rand_f32()) / self.height as f32;
            let ray = self.camera.get_ray(u, v);
            color += self.trace(&ray, 0);
        }
        color / self.samples_per_pixel as f32
    }

    fn trace(&self, ray: &Ray, depth: u32) -> Vec3 {
        if depth >= self.max_depth {
            return Vec3::ZERO;
        }

        if let Some(hit) = self.world.intersect(ray, &self.objects) {
            let mut color = Vec3::ZERO;

            if let Some(emission) = hit.material.and_then(|m| self.objects.get(hit.object_index).map(|o| o.material.emission * o.material.emission_strength)) {
                color += emission;
            }

            let scattered = self.scatter(ray, &hit);
            if let Some((scattered_ray, albedo)) = scattered {
                color += albedo * self.trace(&scattered_ray, depth + 1);
            }

            color
        } else {
            self.world.background_color(ray)
        }
    }

    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let material = &self.objects[hit.object_index].material;

        match material.material_type {
            TraceMaterialType::Diffuse => {
                let target = hit.point + hit.normal + random_in_unit_sphere();
                let scattered = Ray {
                    origin: hit.point,
                    direction: (target - hit.point).normalize(),
                    t_min: 0.001,
                    t_max: f32::INFINITY,
                    depth: ray.depth + 1,
                    time: ray.time,
                    importance: ray.importance,
                };
                Some((scattered, material.base_color))
            }
            TraceMaterialType::Glossy => {
                let reflected = reflect(ray.direction.normalize(), hit.normal);
                let scattered = Ray {
                    origin: hit.point,
                    direction: reflected + random_in_unit_sphere() * material.roughness,
                    t_min: 0.001,
                    t_max: f32::INFINITY,
                    depth: ray.depth + 1,
                    time: ray.time,
                    importance: ray.importance,
                };
                Some((scattered, material.base_color))
            }
            TraceMaterialType::Glass => {
                let refracted = refract(ray.direction.normalize(), hit.normal, material.ior);
                if let Some(refracted_dir) = refracted {
                    let scattered = Ray {
                        origin: hit.point,
                        direction: refracted_dir,
                        t_min: 0.001,
                        t_max: f32::INFINITY,
                        depth: ray.depth + 1,
                        time: ray.time,
                        importance: ray.importance,
                    };
                    Some((scattered, material.base_color))
                } else {
                    let reflected = reflect(ray.direction.normalize(), hit.normal);
                    let scattered = Ray {
                        origin: hit.point,
                        direction: reflected,
                        t_min: 0.001,
                        t_max: f32::INFINITY,
                        depth: ray.depth + 1,
                        time: ray.time,
                        importance: ray.importance,
                    };
                    Some((scattered, material.base_color))
                }
            }
            _ => {
                let target = hit.point + hit.normal + random_in_unit_sphere();
                let scattered = Ray {
                    origin: hit.point,
                    direction: (target - hit.point).normalize(),
                    t_min: 0.001,
                    t_max: f32::INFINITY,
                    depth: ray.depth + 1,
                    time: ray.time,
                    importance: ray.importance,
                };
                Some((scattered, material.base_color))
            }
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Option<Vec3> {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    Some(r_out_perp + r_out_parallel)
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p;
    loop {
        p = Vec3::new(rand_f32() * 2.0 - 1.0, rand_f32() * 2.0 - 1.0, rand_f32() * 2.0 - 1.0);
        if p.length_squared() < 1.0 {
            break;
        }
    }
    p
}

fn rand_f32() -> f32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    (hasher.finish() % 1000000) as f32 / 1000000.0
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let half_height = (self.fov.to_radians() / 2.0).tan();
        let half_width = half_height * self.aspect;

        let w = (self.position - self.look_at).normalize();
        let u_axis = self.up.cross(w).normalize();
        let v_axis = w.cross(u_axis);

        let lower_left = self.position - half_width * u_axis - half_height * v_axis - w;
        let direction = lower_left + u * 2.0 * half_width * u_axis + v * 2.0 * half_height * v_axis - self.position;

        Ray {
            origin: self.position,
            direction: direction.normalize(),
            t_min: 0.001,
            t_max: f32::INFINITY,
            depth: 0,
            time: 0.0,
            importance: 1.0,
        }
    }
}

impl World {
    fn intersect(&self, ray: &Ray, objects: &[TraceObject]) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;
        for (i, obj) in objects.iter().enumerate() {
            if !obj.visible { continue; }
            let local_ray = Ray {
                origin: ray.origin - obj.position,
                direction: ray.direction,
                t_min: ray.t_min,
                t_max: ray.t_max,
                depth: ray.depth,
                time: ray.time,
                importance: ray.importance,
            };
            if let Some(mut hit) = self.intersect_object(obj, &local_ray) {
                hit.object_index = i;
                hit.point += obj.position;
                if closest.as_ref().map_or(true, |c| hit.t < c.t) {
                    closest = Some(hit);
                }
            }
        }
        closest
    }

    fn intersect_object(&self, obj: &TraceObject, ray: &Ray) -> Option<HitRecord> {
        let mut closest = None;
        for i in (0..obj.indices.len()).step_by(3) {
            let v0 = obj.vertices[obj.indices[i] as usize];
            let v1 = obj.vertices[obj.indices[i + 1] as usize];
            let v2 = obj.vertices[obj.indices[i + 2] as usize];

            if let Some(t) = self.intersect_triangle(ray, v0, v1, v2) {
                if t > ray.t_min && t < ray.t_max {
                    if closest.map_or(true, |c: f32| t < c) {
                        closest = Some(t);
                    }
                }
            }
        }

        closest.map(|t| {
            let point = ray.origin + ray.direction * t;
            let edge1 = obj.vertices[obj.indices[1] as usize] - obj.vertices[obj.indices[0] as usize];
            let edge2 = obj.vertices[obj.indices[2] as usize] - obj.vertices[obj.indices[0] as usize];
            let normal = edge1.cross(edge2).normalize();
            let front_face = ray.direction.dot(normal) < 0.0;
            let normal = if front_face { normal } else { -normal };

            HitRecord {
                t,
                point: point + obj.position,
                normal,
                uv: Vec2::ZERO,
                material: None,
                object_index: 0,
                face_index: 0,
                front_face,
            }
        })
    }

    fn intersect_triangle(&self, ray: &Ray, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<f32> {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -0.00001 && a < 0.00001 { return None; }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 { return None; }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 { return None; }

        let t = f * edge2.dot(q);
        if t > 0.00001 { Some(t) } else { None }
    }

    fn background_color(&self, ray: &Ray) -> Vec3 {
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0) * self.color
    }
}