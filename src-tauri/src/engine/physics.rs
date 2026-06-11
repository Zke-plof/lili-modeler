use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhysicsWorld {
    pub gravity: Vec3,
    pub substeps_per_frame: u32,
    pub solver_iterations: u32,
    pub rigid_bodies: Vec<RigidBody>,
    pub constraints: Vec<PhysicsConstraint>,
    pub cloth_objects: Vec<ClothObject>,
    pub soft_bodies: Vec<SoftBody>,
    pub fluid_objects: Vec<FluidDomain>,
    pub particle_systems: Vec<ParticleSystem>,
    pub force_fields: Vec<ForceField>,
    pub collision_objects: Vec<CollisionObject>,
    pub current_frame: u32,
    pub baking: bool,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            substeps_per_frame: 10,
            solver_iterations: 50,
            rigid_bodies: Vec::new(),
            constraints: Vec::new(),
            cloth_objects: Vec::new(),
            soft_bodies: Vec::new(),
            fluid_objects: Vec::new(),
            particle_systems: Vec::new(),
            force_fields: Vec::new(),
            collision_objects: Vec::new(),
            current_frame: 1,
            baking: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RigidBody {
    pub object_id: String,
    pub rigid_type: RigidBodyType,
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub collision_shape: CollisionShape,
    pub collision_margin: f32,
    pub use_margin: bool,
    pub collision_groups: u32,
    pub collision_mask: u32,
    pub use_start_deactivated: bool,
    pub use_deactivation: bool,
    pub deactivation_threshold: f32,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    pub locked: [bool; 6],
    pub center_of_mass: Vec3,
    pub automatic_center: bool,
    pub gravity: Option<Vec3>,
    pub velocity: Option<Vec3>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RigidBodyType {
    Active,
    Passive,
    Dynamic,
    Rigid,
    Soft,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CollisionShape {
    Box,
    Sphere,
    Capsule,
    Cylinder,
    Cone,
    ConvexHull,
    Mesh,
    Compound,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhysicsConstraint {
    pub constraint_type: ConstraintType,
    pub object_a: String,
    pub object_b: String,
    pub pivot_a: Vec3,
    pub pivot_b: Vec3,
    pub axis_a: Vec3,
    pub axis_b: Vec3,
    pub use_limit_x: bool,
    pub use_limit_y: bool,
    pub use_limit_z: bool,
    pub limit_lower: Vec3,
    pub limit_upper: Vec3,
    pub spring_stiffness: Vec3,
    pub spring_damping: Vec3,
    pub use_spring: [bool; 3],
    pub breaking_threshold: f32,
    pub use_breaking: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConstraintType {
    Point,
    Hinge,
    Slider,
    Piston,
    Generic,
    GenericSpring,
    Motor,
    Ragdoll,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClothObject {
    pub object_id: String,
    pub vertex_group_mass: String,
    pub pin_stiffness: f32,
    pub vertex_group_structural: String,
    pub structural_stiffness: f32,
    pub vertex_group_shear: String,
    pub shear_stiffness: f32,
    pub vertex_group_bending: String,
    pub bending_stiffness: f32,
    pub vertex_group_pressure: String,
    pub pressure: f32,
    pub vertex_groupVelocity: String,
    pub velocity_stiffness: f32,
    pub internal_friction: f32,
    pub curl: f32,
    pub shear: f32,
    pub tension: f32,
    pub use_pressure: bool,
    pub use_self_collision: bool,
    pub self_collision_min: f32,
    pub self_collision_friction: f32,
    pub use_gravity: bool,
    pub gravity: Vec3,
    pub air_drag: f32,
    pub use_edge_collision: bool,
    pub use_face_collision: bool,
    pub use_collision: bool,
    pub collision_quality: u32,
    pub solver_iterations: u32,
    pub time_scale: f32,
    pub vertex_group_spring: String,
    pub spring_stiffness: f32,
    pub target: Option<String>,
    pub vertex_group_goal: String,
    pub goal_min: f32,
    pub goal_max: f32,
    pub goal_default: f32,
    pub goal_spring: f32,
    pub goal_friction: f32,
    pub use_vertex_group_goal: bool,
    pub keyframe_insert: bool,
}

impl Default for ClothObject {
    fn default() -> Self {
        Self {
            object_id: String::new(),
            vertex_group_mass: String::new(),
            pin_stiffness: 1.0,
            vertex_group_structural: String::new(),
            structural_stiffness: 15.0,
            vertex_group_shear: String::new(),
            shear_stiffness: 5.0,
            vertex_group_bending: String::new(),
            bending_stiffness: 0.5,
            vertex_group_pressure: String::new(),
            pressure: 0.0,
            vertex_groupVelocity: String::new(),
            velocity_stiffness: 1.0,
            internal_friction: 0.0,
            curl: 0.0,
            shear: 0.0,
            tension: 0.0,
            use_pressure: false,
            use_self_collision: false,
            self_collision_min: 0.01,
            self_collision_friction: 0.0,
            use_gravity: true,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            air_drag: 0.0,
            use_edge_collision: true,
            use_face_collision: true,
            use_collision: true,
            collision_quality: 2,
            solver_iterations: 25,
            time_scale: 1.0,
            vertex_group_spring: String::new(),
            spring_stiffness: 15.0,
            target: None,
            vertex_group_goal: String::new(),
            goal_min: 0.0,
            goal_max: 1.0,
            goal_default: 0.7,
            goal_spring: 1.0,
            goal_friction: 0.0,
            use_vertex_group_goal: false,
            keyframe_insert: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftBody {
    pub object_id: String,
    pub vertex_group: String,
    pub spring_length: f32,
    pub mass: f32,
    pub friction: f32,
    pub speed: f32,
    pub vertex_group_goal: String,
    pub goal_min: f32,
    pub goal_max: f32,
    pub goal_default: f32,
    pub goal_spring: f32,
    pub goal_friction: f32,
    pub use_goal: bool,
    pub use_edges: bool,
    pub pull: f32,
    pub push: f32,
    pub damping: f32,
    pub face_stiffness: f32,
    pub bend: f32,
    pub spring: SoftBodySpringType,
    pub use_self_collision: bool,
    pub use_stiff_quads: bool,
    pub use_edge_collision: bool,
    pub use_face_collision: bool,
    pub plastic: f32,
    pub velocity_smooth: f32,
    pub aero: f32,
    pub use_estimate_volume: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SoftBodySpringType {
    Linear,
    Edge,
    Pad,
}

impl Default for SoftBody {
    fn default() -> Self {
        Self {
            object_id: String::new(),
            vertex_group: String::new(),
            spring_length: 0.5,
            mass: 1.0,
            friction: 0.1,
            speed: 1.0,
            vertex_group_goal: String::new(),
            goal_min: 0.0,
            goal_max: 1.0,
            goal_default: 0.7,
            goal_spring: 0.5,
            goal_friction: 0.0,
            use_goal: true,
            use_edges: true,
            pull: 0.5,
            push: 0.5,
            damping: 1.0,
            face_stiffness: 1.0,
            bend: 0.5,
            spring: SoftBodySpringType::Edge,
            use_self_collision: false,
            use_stiff_quads: true,
            use_edge_collision: false,
            use_face_collision: false,
            plastic: 0.0,
            velocity_smooth: 0.0,
            aero: 0.0,
            use_estimate_volume: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluidDomain {
    pub object_id: String,
    pub domain_type: FluidDomainType,
    pub resolution: u32,
    pub border_collisions: [bool; 6],
    pub use_viscosity: bool,
    pub viscosity: f32,
    pub viscosity_exponent: u32,
    pub surface_tension: f32,
    pub use_diffusion: bool,
    pub diffusion: f32,
    pub use_vorticity: bool,
    pub vorticity_strength: f32,
    pub use_confinement: bool,
    pub confinement_strength: f32,
    pub use_heat_diffusion: bool,
    pub heat_diffusion: f32,
    pub use_fuel: bool,
    pub fuel_amount: f32,
    pub reaction_speed: f32,
    pub use_smoke: bool,
    pub smoke_amount: f32,
    pub smoke_dissolve: f32,
    pub use_color: bool,
    pub color_grid: Vec<u8>,
    pub use_noise: bool,
    pub noise_scale: f32,
    pub noise_strength: f32,
    pub noise_depth: u32,
    pub use_fire: bool,
    pub fire_amount: f32,
    pub use_flame: bool,
    pub flame_height: f32,
    pub use_dissolve: bool,
    pub dissolve_time: f32,
    pub use_shrink: bool,
    pub shrink_threshold: f32,
    pub use_reverse: bool,
    pub effector_weights: EffectorWeights,
    pub cache: FluidCache,
    pub particle_system: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FluidDomainType {
    Gas,
    Liquid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EffectorWeights {
    pub gravity: f32,
    pub all: f32,
    pub force: f32,
    pub vortex: f32,
    pub magnetic: f32,
    pub harmonic: f32,
    pub charge: f32,
    pub lennardjones: f32,
    pub wind: f32,
    pub curve_guide: f32,
    pub texture: f32,
    pub smoke_flow: f32,
    pub turbulence: f32,
    pub collision: f32,
}

impl Default for EffectorWeights {
    fn default() -> Self {
        Self {
            gravity: 1.0,
            all: 1.0,
            force: 1.0,
            vortex: 1.0,
            magnetic: 1.0,
            harmonic: 1.0,
            charge: 1.0,
            lennardjones: 1.0,
            wind: 1.0,
            curve_guide: 1.0,
            texture: 1.0,
            smoke_flow: 1.0,
            turbulence: 1.0,
            collision: 1.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluidCache {
    pub cache_type: CacheType,
    pub frame_start: u32,
    pub frame_end: u32,
    pub use_cache: bool,
    pub use_disk_cache: bool,
    pub disk_cache_dir: String,
    pub use_multires: bool,
    pub resolution: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheType {
    Modular,
    Final,
    All,
}

impl Default for FluidDomain {
    fn default() -> Self {
        Self {
            object_id: String::new(),
            domain_type: FluidDomainType::Gas,
            resolution: 64,
            border_collisions: [true; 6],
            use_viscosity: false,
            viscosity: 1.0,
            viscosity_exponent: 6,
            surface_tension: 0.0,
            use_diffusion: false,
            diffusion: 0.1,
            use_vorticity: false,
            vorticity_strength: 0.1,
            use_confinement: false,
            confinement_strength: 0.1,
            use_heat_diffusion: false,
            heat_diffusion: 0.1,
            use_fuel: false,
            fuel_amount: 1.0,
            reaction_speed: 1.0,
            use_smoke: true,
            smoke_amount: 1.0,
            smoke_dissolve: 5.0,
            use_color: false,
            color_grid: Vec::new(),
            use_noise: false,
            noise_scale: 1.0,
            noise_strength: 1.0,
            noise_depth: 2,
            use_fire: false,
            fire_amount: 1.0,
            use_flame: false,
            flame_height: 1.0,
            use_dissolve: false,
            dissolve_time: 5.0,
            use_shrink: false,
            shrink_threshold: 0.0,
            use_reverse: false,
            effector_weights: EffectorWeights::default(),
            cache: FluidCache {
                cache_type: CacheType::Modular,
                frame_start: 1,
                frame_end: 250,
                use_cache: true,
                use_disk_cache: false,
                disk_cache_dir: String::new(),
                use_multires: false,
                resolution: 64,
            },
            particle_system: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticleSystem {
    pub object_id: String,
    pub particle_type: ParticleType,
    pub count: u32,
    pub seed: u32,
    pub frame_start: f32,
    pub frame_end: f32,
    pub lifetime: f32,
    pub lifetime_random: f32,
    pub emit_from: EmitFrom,
    pub use_emit_random: bool,
    pub use_even_distribution: bool,
    pub userjit: u32,
    pub jitfac: f32,
    pub turn: f32,
    pub use_rotation_instance: bool,
    pub use_size_instance: bool,
    pub use_scale_instance: bool,
    pub use_render_emitter: bool,
    pub use_multiplier: bool,
    pub particle_size: f32,
    pub size_random: f32,
    pub display_size: f32,
    pub collision_type: ParticleCollisionType,
    pub die_on_collision: bool,
    pub use_die_on_collision: bool,
    pub physics_type: ParticlePhysicsType,
    pub size_deflect: bool,
    pub use_die_on_collision_real: bool,
    pub use_size_deflect_real: bool,
    pub use_rotations_real: bool,
    pub angular_velocity_type: AngularVelocityType,
    pub angular_velocity: Vec3,
    pub phase_factor: f32,
    pub phase_factor_random: f32,
    pub use_phase_factor_random_real: bool,
    pub use_dynamic_rotation: bool,
    pub brownian_motion: f32,
    pub drag: f32,
    pub damping: f32,
    pub mass: f32,
    pub use_mass: bool,
    pub use_multiply_size_mass: bool,
    pub size_mass: f32,
    pub vertex_group: String,
    pub use_vertex_group: bool,
    pub use_vertex_group_density: bool,
    pub use_vertex_group_mass: bool,
    pub use_vertex_group_size: bool,
    pub use_vertex_group_rotation: bool,
    pub use_vertex_group_velocity: bool,
    pub use_vertex_group_field_weights: bool,
    pub use_vertex_group_effector_weights: bool,
    pub render_type: ParticleRenderType,
    pub material: u32,
    pub material_slot: u32,
    pub display_method: ParticleDisplayMethod,
    pub display_percentage: u32,
    pub use_render_emitter_real: bool,
    pub use_show_guide: bool,
    pub use_show_guide_real: bool,
    pub parent: Option<String>,
    pub use_parent: bool,
    pub use_collection: bool,
    pub collection: Option<String>,
    pub use_whole_collection: bool,
    pub use_global_dupli: bool,
    pub use_object_space: bool,
    pub use_adaptive_subframes: bool,
    pub subframes: u32,
    pub use_auto_time: bool,
    pub use_adaptive_subframes_real: bool,
    pub use_multiply_mass_size: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticleType {
    Emitter,
    Hair,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmitFrom {
    Vertices,
    Faces,
    Volume,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticleCollisionType {
    Particle,
    Point,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticlePhysicsType {
    Newtonian,
    Keyed,
    Boid,
    Fluid,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AngularVelocityType {
    None,
    Linear,
    Random,
    Quaternion,
    QuaternionX,
    QuaternionY,
    QuaternionZ,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticleRenderType {
    Halo,
    Line,
    Path,
    Object,
    Collection,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticleDisplayMethod {
    Render,
    Dot,
    Cross,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForceField {
    pub field_type: ForceFieldType,
    pub shape: ForceFieldShape,
    pub strength: f32,
    pub linear: f32,
    pub radial_min: f32,
    pub radial_max: f32,
    pub rot: f32,
    pub size: f32,
    pub falloff_power: f32,
    pub falloff: ForceFieldFalloff,
    pub texture: Option<String>,
    pub use_radial_min: bool,
    pub use_radial_max: bool,
    pub use_object_coords: bool,
    pub use_global_coords: bool,
    pub use_absorption: bool,
    pub use_group: bool,
    pub use_multiple: bool,
    pub domain: ForceFieldDomain,
    pub noise: f32,
    pub seed: u32,
    pub use_noise: bool,
    pub noise_scale: f32,
    pub noise_depth: u32,
    pub use_noise_real: bool,
    pub use_noise_absolute: bool,
    pub z_direction: f32,
    pub use_z_direction: bool,
    pub use_rotational_gradient: bool,
    pub use_radial_gradient: bool,
    pub use_radial_min_real: bool,
    pub use_radial_max_real: bool,
    pub use_rotational_gradient_real: bool,
    pub use_radial_gradient_real: bool,
    pub guide_min: f32,
    pub guide_max: f32,
    pub use_guide_min: bool,
    pub use_guide_max: bool,
    pub use_range: bool,
    pub min_distance: f32,
    pub max_distance: f32,
    pub use_min_distance: bool,
    pub use_max_distance: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ForceFieldType {
    Force,
    Wind,
    Vortex,
    Magnetic,
    Harmonic,
    Charge,
    LennardJones,
    Texture,
    CurveGuide,
    Turbulence,
    Drag,
    FluidFlow,
    ForceField,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ForceFieldShape {
    Point,
    Line,
    Plane,
    Box,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ForceFieldFalloff {
    None,
    Curve,
    Linear,
    CustomCurve,
    Root,
    Sphere,
    InvSquare,
    LinearDown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ForceFieldDomain {
    All,
    Surface,
    Volume,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollisionObject {
    pub object_id: String,
    pub use_particle: bool,
    pub use_particle_kill: bool,
    pub use_face: bool,
    pub use_particle_radius: bool,
    pub particle_radius: f32,
    pub use_edge: bool,
    pub use_edge_radius: bool,
    pub edge_radius: f32,
    pub use_modifier: bool,
    pub damping: f32,
    pub friction: f32,
    pub use_normal: bool,
    pub normal_factor: f32,
    pub use_culling: bool,
    pub thickness_outer: f32,
    pub thickness_inner: f32,
    pub use_particle_dupli: bool,
    pub use_particle_kill_real: bool,
}

impl PhysicsWorld {
    pub fn step(&mut self, dt: f32) {
        for rigid_body in &mut self.rigid_bodies {
            if rigid_body.rigid_type == RigidBodyType::Active ||
               rigid_body.rigid_type == RigidBodyType::Dynamic {
                let gravity_effect = self.gravity * dt;
                rigid_body.linear_velocity += gravity_effect;
                rigid_body.linear_velocity *= 1.0 - rigid_body.linear_damping * dt;

                let position_delta = rigid_body.linear_velocity * dt;
                let _ = position_delta;
            }
        }

        for constraint in &mut self.constraints {
            let _ = constraint;
        }

        for soft_body in &mut self.soft_bodies {
            let _ = soft_body;
        }

        for cloth in &mut self.cloth_objects {
            let _ = cloth;
        }

        for fluid in &mut self.fluid_objects {
            let _ = fluid;
        }
    }
}