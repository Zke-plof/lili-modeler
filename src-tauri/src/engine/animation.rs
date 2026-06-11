use glam::{Vec3, Quat, EulerRot};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimData {
    pub scenes: Vec<Scene>,
    pub actions: Vec<Action>,
    pub nla_tracks: Vec<NlaTrack>,
    pub current_scene: usize,
    pub current_action: usize,
}

impl Default for AnimData {
    fn default() -> Self {
        Self {
            scenes: vec![Scene::default()],
            actions: vec![Action::default()],
            nla_tracks: Vec::new(),
            current_scene: 0,
            current_action: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scene {
    pub frame_start: u32,
    pub frame_end: u32,
    pub frame_current: u32,
    pub frame_step: u32,
    pub fps: u32,
    pub play_audio: bool,
    pub sync_mode: SyncMode,
    pub objects: Vec<ObjectAnim>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            frame_start: 1,
            frame_end: 250,
            frame_current: 1,
            frame_step: 1,
            fps: 24,
            play_audio: false,
            sync_mode: SyncMode::None,
            objects: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncMode {
    None,
    AVSync,
    FrameDrop,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAnim {
    pub object_id: Uuid,
    pub animation_data: Option<AnimationData>,
    pub constraints: Vec<Constraint>,
    pub modifiers: Vec<AnimModifier>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimationData {
    pub action: Option<String>,
    pub channels: Vec<FCurve>,
    pub drivers: Vec<Driver>,
    pub use_nla: bool,
    pub nla_strength: f32,
}

impl Default for AnimationData {
    fn default() -> Self {
        Self {
            action: None,
            channels: Vec::new(),
            drivers: Vec::new(),
            use_nla: false,
            nla_strength: 1.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub id_root: String,
    pub curves: Vec<FCurve>,
    pub groups: Vec<FCurveGroup>,
    pub slot_markers: Vec<SlotMarker>,
    pub frame_range: (f32, f32),
}

impl Default for Action {
    fn default() -> Self {
        Self {
            name: "Action".to_string(),
            id_root: "Object".to_string(),
            curves: Vec::new(),
            groups: Vec::new(),
            slot_markers: Vec::new(),
            frame_range: (1.0, 250.0),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FCurve {
    pub data_path: String,
    pub array_index: u32,
    pub keyframes: Vec<Keyframe>,
    pub interpolation: Interpolation,
    pub extrapolation: Extrapolation,
    pub color_mode: FCurveColorMode,
    pub locked: bool,
    pub muted: bool,
    pub visible: bool,
    pub active: bool,
    pub manual_handle: bool,
    pub driver: Option<Driver>,
    pub modifier: Option<FCurveModifier>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Interpolation {
    Constant,
    Linear,
    Bezier,
    Sinusoidal,
    Quadratic,
    Cubic,
    Quartic,
    Quintic,
    Exponential,
    Circular,
    Back,
    Bounce,
    Elastic,
    Auto,
}

impl Default for Interpolation {
    fn default() -> Self { Self::Bezier }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Extrapolation {
    Nothing,
    Constant,
    Linear,
}

impl Default for Extrapolation {
    fn default() -> Self { Self::Nothing }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FCurveColorMode {
    Rgb,
    HSV,
    Neon,
}

impl Default for FCurveColorMode {
    fn default() -> Self { Self::Rgb }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keyframe {
    pub frame: f32,
    pub value: f32,
    pub handle_left: KeyframeHandle,
    pub handle_right: KeyframeHandle,
    pub easing: Easing,
    pub amplitude: f32,
    pub period: f32,
    pub back: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyframeHandle {
    pub handle_type: HandleType,
    pub position: (f32, f32),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum HandleType {
    Free,
    Aligned,
    Vector,
    Auto,
    AutoClamped,
    Fixed,
}

impl Default for HandleType {
    fn default() -> Self { Self::Auto }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Easing {
    Default,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Default for Easing {
    fn default() -> Self { Self::Default }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FCurveGroup {
    pub name: String,
    pub channels: Vec<usize>,
    pub locked: bool,
    pub muted: bool,
    pub visible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlotMarker {
    pub name: String,
    pub frame: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Driver {
    pub driver_type: DriverType,
    pub variables: Vec<DriverVariable>,
    pub expression: String,
    pub expression_simple: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriverType {
    AveragedValue,
    SumValues,
    ScriptedExpression,
    PythonExpression,
}

impl Default for DriverType {
    fn default() -> Self { Self::ScriptedExpression }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DriverVariable {
    pub name: String,
    pub var_type: DriverVarType,
    pub targets: Vec<DriverTarget>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriverVarType {
    SingleProp,
    TransformChannel,
    RotationalDifference,
    Distance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DriverTarget {
    pub id: Option<String>,
    pub data_path: String,
    pub transform_type: TransformChannel,
    pub transform_space: TransformSpace,
    pub bone_target: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransformChannel {
    LocX, LocY, LocZ,
    RotX, RotY, RotZ,
    ScaleX, ScaleY, ScaleZ,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransformSpace {
    WorldSpace,
    LocalSpace,
    PoseSpace,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FCurveModifier {
    Cycles {
        before_mode: FModifierExtend,
        after_mode: FModifierExtend,
    },
    Noise {
        phase: f32,
        scale: f32,
        strength: f32,
        offset: f32,
        depth: u32,
        blend_type: FModifierBlend,
    },
    Limit {
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
        use_min_x: bool,
        use_max_x: bool,
        use_min_y: bool,
        use_max_y: bool,
    },
    SteppedInterpolation {
        step_size: f32,
        offset: f32,
        use_frame_number: bool,
    },
    Polynomial {
        coeffs: [f32; 4],
    },
    Generator {
        coeffs: Vec<f32>,
        mode: GeneratorMode,
    },
    Envelope {
        control_points: Vec<EnvelopeControlPoint>,
        reference_value: f32,
        default_min: f32,
        default_max: f32,
    },
    Bounds {
        min: f32,
        max: f32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FModifierExtend {
    Nothing,
    Constant,
    Extrapolation,
    Cycles,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FModifierBlend {
    Replace,
    Add,
    Subtract,
    Multiply,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeControlPoint {
    pub frame: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GeneratorMode {
    Polynomial,
    Power,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AnimModifier {
    Armature {
        use_deform: bool,
        object: Option<String>,
    },
    Curve {
        object: Option<String>,
        deform_axis: DeformAxis,
    },
    Lattice {
        object: Option<String>,
    },
    ParticleSystem {
        particle_system: String,
    },
    Shrinkwrap {
        target: Option<String>,
        offset: f32,
    },
    SimpleDeform {
        deform_type: SimpleDeformType,
        factor: f32,
        limits: (f32, f32),
        origin: Option<String>,
        lock: bool,
    },
    Smooth {
        factor: [f32; 3],
        iterations: u32,
        use_x: bool,
        use_y: bool,
        use_z: bool,
    },
    CorrectiveSmooth {
        factor: f32,
        iterations: u32,
        scale: f32,
        use_only_smooth: bool,
        use_pin_boundary: bool,
    },
    LaplacianSmooth {
        factor: f32,
        iterations: u32,
        smoothing: [f32; 3],
        use_volume: bool,
    },
    SurfaceDeform {
        target: Option<String>,
        vertex_group: String,
        falloff: f32,
    },
    Hook {
        object: Option<String>,
        bone: String,
        vertex_group: String,
        strength: f32,
    },
    Mask {
        mode: MaskMode,
        threshold: f32,
        invert: bool,
    },
    Mirror {
        axis: [bool; 3],
        use_clip: bool,
        mirror_object: Option<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeformAxis {
    X, Y, Z,
    NegX, NegY, NegZ,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimpleDeformType {
    Twist,
    Taper,
    Bend,
    Stretch,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MaskMode {
    VertGroup,
    Armature,
    VertexWeight,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Constraint {
    ChildOf {
        target: Option<String>,
        bone: String,
        use_location: [bool; 3],
        use_rotation: [bool; 3],
        use_scale: [bool; 3],
        inverse: bool,
    },
    FollowPath {
        target: Option<String>,
        use_curve_follow: bool,
        use_curve_radius: bool,
        forward_axis: FollowAxis,
        up_axis: FollowAxis,
        offset: f32,
        offset_factor: f32,
    },
    IK {
        target: Option<String>,
        bone: String,
        pole_target: Option<String>,
        pole_bone: String,
        chain_count: i32,
        pole_angle: f32,
        use_location: bool,
    },
    TrackTo {
        target: Option<String>,
        bone: String,
        track_axis: TrackAxis,
        up_axis: TrackAxis,
        use_target_z: bool,
    },
    CopyLocation {
        target: Option<String>,
        bone: String,
        use_offset: bool,
    },
    CopyRotation {
        target: Option<String>,
        bone: String,
        use_offset: bool,
        mix_mode: CopyRotationMix,
    },
    CopyScale {
        target: Option<String>,
        bone: String,
        use_offset: bool,
    },
    LimitDistance {
        target: Option<String>,
        bone: String,
        distance: f32,
        limit_mode: LimitMode,
    },
    LimitLocation {
        use_min_x: bool,
        use_min_y: bool,
        use_min_z: bool,
        use_max_x: bool,
        use_max_y: bool,
        use_max_z: bool,
        min: [f32; 3],
        max: [f32; 3],
    },
    LimitRotation {
        use_limit_x: bool,
        use_limit_y: bool,
        use_limit_z: bool,
        min: [f32; 3],
        max: [f32; 3],
    },
    LimitScale {
        use_min_x: bool,
        use_min_y: bool,
        use_min_z: bool,
        use_max_x: bool,
        use_max_y: bool,
        use_max_z: bool,
        min: [f32; 3],
        max: [f32; 3],
    },
    MaintainVolume {
        target: Option<String>,
        volume: f32,
        uniform: f32,
    },
    StretchTo {
        target: Option<String>,
        bone: String,
        bulge: f32,
        use_bulge_min: bool,
        use_bulge_max: bool,
        bulge_min: f32,
        bulge_max: f32,
        volume: f32,
        rest_length: f32,
    },
    DampedTrack {
        target: Option<String>,
        bone: String,
        track_axis: TrackAxis,
    },
    LockedTrack {
        target: Option<String>,
        bone: String,
        track_axis: TrackAxis,
        lock_axis: TrackAxis,
    },
    ClampTo {
        target: Option<String>,
        clamp_type: ClampType,
        main_axis: bool,
    },
    Transform {
        target: Option<String>,
        use_motion_extrapolate: bool,
    },
    Shrinkwrap {
        target: Option<String>,
        offset: f32,
        wrap_method: ShrinkwrapMethod,
        wrap_mode: ShrinkwrapMode,
    },
    PivotTarget {
        target: Option<String>,
        bone: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FollowAxis {
    ForwardX, ForwardY, ForwardZ,
    UpX, UpY, UpZ,
    TrackX, TrackY, TrackZ,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrackAxis {
    X, Y, Z,
    NegX, NegY, NegZ,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CopyRotationMix {
    Replace,
    Add,
    BeforeOriginal,
    AfterOriginal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LimitMode {
    Inside,
    Outside,
    OnSurface,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClampType {
    Plain,
    Tangential,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShrinkwrapMethod {
    NearestSurface,
    Project,
    NearestVertex,
    NearestEdge,
    Face,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShrinkwrapMode {
    OnSurface,
    Inside,
    Outside,
    AboveSurface,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NlaTrack {
    pub name: String,
    pub strips: Vec<NlaStrip>,
    pub muted: bool,
    pub solo: bool,
    pub lock: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NlaStrip {
    pub name: String,
    pub start_frame: f32,
    pub end_frame: f32,
    pub repeat: f32,
    pub scale: f32,
    pub blend_mode: NlaBlendMode,
    pub blend_in: f32,
    pub blend_out: f32,
    pub extrapolation: NlaExtrapolate,
    pub action: Option<String>,
    pub modifiers: Vec<FCurveModifier>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NlaBlendMode {
    Replace,
    Combine,
    Add,
    Subtract,
    Multiply,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NlaExtrapolate {
    Nothing,
    Hold,
    HoldForward,
}

pub struct AnimEvaluator {
    pub frame: f32,
    pub result_cache: HashMap<String, AnimResult>,
}

#[derive(Clone, Debug)]
pub enum AnimResult {
    Float(f32),
    Vec3(Vec3),
    Quaternion(Quat),
    Euler((f32, f32, f32)),
    Color([f32; 4]),
    Boolean(bool),
    Int(i32),
}

impl Default for AnimEvaluator {
    fn default() -> Self {
        Self {
            frame: 1.0,
            result_cache: HashMap::new(),
        }
    }
}

impl AnimEvaluator {
    pub fn new() -> Self { Self::default() }

    pub fn evaluate(&mut self, anim_data: &AnimationData, frame: f32) {
        self.frame = frame;
        self.result_cache.clear();

        for curve in &anim_data.channels {
            if curve.muted || !curve.visible { continue; }
            let value = self.evaluate_fcurve(curve, frame);
            self.result_cache.insert(
                format!("{}_{}", curve.data_path, curve.array_index),
                AnimResult::Float(value),
            );
        }
    }

    fn evaluate_fcurve(&self, curve: &FCurve, frame: f32) -> f32 {
        if curve.keyframes.is_empty() { return 0.0; }

        let mut sorted = curve.keyframes.clone();
        sorted.sort_by(|a, b| a.frame.partial_cmp(&b.frame).unwrap());

        if frame <= sorted[0].frame {
            return match curve.extrapolation {
                Extrapolation::Nothing => sorted[0].value,
                Extrapolation::Constant => sorted[0].value,
                Extrapolation::Linear => {
                    if sorted.len() > 1 {
                        let slope = (sorted[1].value - sorted[0].value) / (sorted[1].frame - sorted[0].frame);
                        sorted[0].value + slope * (frame - sorted[0].frame)
                    } else {
                        sorted[0].value
                    }
                }
            };
        }

        if frame >= sorted.last().unwrap().frame {
            return match curve.extrapolation {
                Extrapolation::Nothing => sorted.last().unwrap().value,
                Extrapolation::Constant => sorted.last().unwrap().value,
                Extrapolation::Linear => {
                    if sorted.len() > 1 {
                        let last = sorted.last().unwrap();
                        let prev = &sorted[sorted.len() - 2];
                        let slope = (last.value - prev.value) / (last.frame - prev.frame);
                        last.value + slope * (frame - last.frame)
                    } else {
                        sorted.last().unwrap().value
                    }
                }
            };
        }

        for i in 0..sorted.len() - 1 {
            if frame >= sorted[i].frame && frame <= sorted[i + 1].frame {
                let t = (frame - sorted[i].frame) / (sorted[i + 1].frame - sorted[i].frame);
                return match curve.interpolation {
                    Interpolation::Constant => sorted[i].value,
                    Interpolation::Linear => sorted[i].value + (sorted[i + 1].value - sorted[i].value) * t,
                    Interpolation::Bezier => {
                        let h1 = sorted[i].handle_right.position;
                        let h2 = sorted[i + 1].handle_left.position;
                        let dx = sorted[i + 1].frame - sorted[i].frame;
                        let dy = sorted[i + 1].value - sorted[i].value;
                        let _ = (h1, h2, dx, dy);
                        let t2 = t * t;
                        let t3 = t2 * t;
                        sorted[i].value * (2.0 * t3 - 3.0 * t2 + 1.0) +
                        sorted[i + 1].value * (-2.0 * t3 + 3.0 * t2)
                    }
                    Interpolation::Sinusoidal => {
                        sorted[i].value + (sorted[i + 1].value - sorted[i].value) * (1.0 - (t * std::f32::consts::PI).cos()) * 0.5
                    }
                    Interpolation::Quadratic => {
                        sorted[i].value + (sorted[i + 1].value - sorted[i].value) * t * t
                    }
                    Interpolation::Cubic => {
                        sorted[i].value + (sorted[i + 1].value - sorted[i].value) * t * t * t
                    }
                    Interpolation::Exponential => {
                        sorted[i].value + (sorted[i + 1].value - sorted[i].value) * (10.0_f32.powf(t - 1.0))
                    }
                    Interpolation::Circular => {
                        sorted[i].value + (sorted[i + 1].value - sorted[i].value) * (1.0 - (1.0 - t * t).sqrt())
                    }
                    Interpolation::Bounce => {
                        let mut bt = t;
                        if bt < 0.5 { bt = 1.0 - bt * 2.0; }
                        else { bt = bt * 2.0 - 1.0; }
                        let b = 1.0 - bt * bt;
                        if t < 0.5 { sorted[i].value + (sorted[i + 1].value - sorted[i].value) * (1.0 - b) * 0.5 }
                        else { sorted[i].value + (sorted[i + 1].value - sorted[i].value) * (1.0 + b) * 0.5 }
                    }
                    Interpolation::Elastic => {
                        if t == 0.0 || t == 1.0 {
                            sorted[i].value + (sorted[i + 1].value - sorted[i].value) * t
                        } else {
                            let p = 0.3;
                            let s = p / 4.0;
                            let pow = 10.0 * t - 10.0;
                            sorted[i].value + (sorted[i + 1].value - sorted[i].value) * (-(2.0_f32).powf(pow) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() * 1.0 + 1.0)
                        }
                    }
                    _ => sorted[i].value + (sorted[i + 1].value - sorted[i].value) * t,
                };
            }
        }

        sorted.last().unwrap().value
    }

    pub fn get_float(&self, data_path: &str, index: u32) -> f32 {
        let key = format!("{}_{}", data_path, index);
        match self.result_cache.get(&key) {
            Some(AnimResult::Float(v)) => *v,
            _ => 0.0,
        }
    }

    pub fn get_location(&self) -> Vec3 {
        Vec3::new(
            self.get_float("location", 0),
            self.get_float("location", 1),
            self.get_float("location", 2),
        )
    }

    pub fn get_rotation_euler(&self) -> Vec3 {
        Vec3::new(
            self.get_float("rotation_euler", 0),
            self.get_float("rotation_euler", 1),
            self.get_float("rotation_euler", 2),
        )
    }

    pub fn get_scale(&self) -> Vec3 {
        Vec3::new(
            self.get_float("scale", 0),
            self.get_float("scale", 1),
            self.get_float("scale", 2),
        )
    }
}

pub struct DopeSheetFilter {
    pub show_selected_only: bool,
    pub show_hidden: bool,
    pub show_only_errors: bool,
    pub show_only_errors_in_range: bool,
    pub show_summary: bool,
    pub show_mesh: bool,
    pub show_armature: bool,
    pub show_transverts: bool,
    pub show_particle: bool,
}

impl Default for DopeSheetFilter {
    fn default() -> Self {
        Self {
            show_selected_only: false,
            show_hidden: false,
            show_only_errors: false,
            show_only_errors_in_range: false,
            show_summary: true,
            show_mesh: true,
            show_armature: true,
            show_transverts: true,
            show_particle: true,
        }
    }
}

pub struct GraphEditorState {
    pub visible: bool,
    pub lock_time_to_selection: bool,
    pub auto_snap: AutoSnap,
    pub show_cursor: bool,
    pub use_limit_view: bool,
    pub view_min: (f32, f32),
    pub view_max: (f32, f32),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AutoSnap {
    None,
    NearestFrame,
    NearestSecond,
    NearestMarker,
}

impl Default for GraphEditorState {
    fn default() -> Self {
        Self {
            visible: true,
            lock_time_to_selection: false,
            auto_snap: AutoSnap::None,
            show_cursor: true,
            use_limit_view: false,
            view_min: (1.0, -10.0),
            view_max: (250.0, 10.0),
        }
    }
}