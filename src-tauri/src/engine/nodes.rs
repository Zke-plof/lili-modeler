use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeTree {
    pub name: String,
    pub tree_type: NodeTreeType,
    pub nodes: Vec<Node>,
    pub links: Vec<NodeLink>,
    pub active: bool,
    pub view_center: Vec2,
    pub edit_mode: bool,
    pub compact: bool,
    pub nodewidget_width: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeTreeType {
    Shader,
    Compositor,
    Geometry,
    Texture,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
    pub name: String,
    pub location: Vec2,
    pub width: f32,
    pub height: f32,
    pub hide: bool,
    pub mute: bool,
    pub select: bool,
    pub show_options: bool,
    pub show_preview: bool,
    pub show_texture: bool,
    pub use_custom_color: bool,
    pub color: [f32; 3],
    pub inputs: Vec<NodeSocket>,
    pub outputs: Vec<NodeSocket>,
    pub internal_links: Vec<NodeLink>,
    pub parent: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    // Shader nodes
    ShaderOutput,
    PrincipledBSDF,
    DiffuseBSDF,
    GlossyBSDF,
    GlassBSDF,
    Emission,
    TransparentBSDF,
    RefractionBSDF,
    TranslucentBSDF,
    VelvetBSDF,
    AnisotropicBSDF,
    SubsurfaceScattering,
    ToonBSDF,
    HairBSDF,
    VolumeScatter,
    VolumeAbsorption,
    VolumePrincipled,
    Background,
    MixShader,
    AddShader,
    Math,
    VectorMath,
    Mapping,
    TextureCoordinate,
    ObjectInfo,
    ObjectInfoFloat,
    GeometryInfo,
    CameraData,
    Fresnel,
    LayerWeight,
    Wireframe,
    Normal,
    NormalMap,
    Bump,
    Displacement,
    VectorDisplacement,
    VectorTransform,
    RGB,
    Value,
    HueSaturation,
    Gamma,
    BrightContrast,
    Invert,
    GammaNode,
    ColorRamp,
    SeparateRGB,
    CombineRGB,
    SeparateXYZ,
    CombineXYZ,
    SeparateHSV,
    CombineHSV,
    // Texture nodes
    ImageTexture,
    EnvironmentTexture,
    BrickTexture,
    CheckerTexture,
    GradientTexture,
    MagicTexture,
    MusgraveTexture,
    NoiseTexture,
    VoronoiTexture,
    WaveTexture,
    WhiteNoiseTexture,
    PointDensity,
    SkyTexture,
    // Vector nodes
    Combine,
    Separate,
    // Group
    NodeGroupInput,
    NodeGroupOutput,
    // Compositor
    Viewer,
    Composite,
    FileOutput,
    Levels,
    ColorBalance,
    ColorCorrection,
    CurveRGB,
    HueSaturationCorrect,
    InvertNode,
    Normalize,
    MapRange,
    MapValue,
    Blur,
    Defocus,
    Glare,
    LensDistortion,
    Filter,
    // Geometry
    GeometryInput,
    GeometryOutput,
    GeometryTransform,
    GeometrySet,
    GeometrySeparateGeometry,
    GeometryJoinGeometry,
    GeometryInstanceOnPoints,
    GeometryInstanceOnFaces,
    GeometryMeshToPoints,
    GeometryPointsToMesh,
    GeometrySubdivideMesh,
    GeometrySubdivideCurve,
    GeometryResampleCurve,
    GeometryCurveToMesh,
    GeometryMeshToCurve,
    GeometryFillCurve,
    GeometryExtrudeMesh,
    GeometryScaleElements,
    GeometryTransformGeometry,
    GeometrySetPosition,
    GeometryCaptureAttribute,
    GeometrySampleNearest,
    GeometrySampleIndex,
    GeometrySampleNearestSurface,
    GeometryRaycast,
    GeometryInterpolateGeometry,
    GeometryFieldAtIndex,
    MathAdd,
    MathSubtract,
    MathMultiply,
    MathDivide,
    MathPower,
    MathSqrt,
    MathLogarithm,
    MathExp,
    MathSin,
    MathCos,
    MathTan,
    MathArcsin,
    MathArccos,
    MathArctan,
    MathArctan2,
    MathRound,
    MathFloor,
    MathCeil,
    MathFract,
    MathModulo,
    MathSign,
    MathMin,
    MathMax,
    MathClamp,
    MathMix,
    MathSmoothMin,
    MathSmoothMax,
    MathCompare,
    MathLessThan,
    MathGreaterThan,
    MathEqual,
    MathNotEqual,
    MathAnd,
    MathOr,
    MathNot,
    MathNand,
    MathXor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeSocket {
    pub name: String,
    pub socket_type: SocketType,
    pub default_value: SocketValue,
    pub min_value: f32,
    pub max_value: f32,
    pub hide_value: bool,
    pub hide: bool,
    pub select: bool,
    pub enabled: bool,
    pub linked: bool,
    pub node_id: String,
    pub index: u32,
    pub in_out: SocketDirection,
    pub color: [f32; 3],
    pub display_shape: SocketDisplayShape,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SocketDirection {
    Input,
    Output,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SocketType {
    Float,
    Vector,
    Color,
    Shader,
    Geometry,
    String,
    Boolean,
    Integer,
    Closure,
    Object,
    Collection,
    Texture,
    Material,
    Rotation,
    Matrix,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SocketValue {
    Float(f32),
    Vector(Vec2),
    Color([f32; 4]),
    Boolean(bool),
    Integer(i32),
    String(String),
}

impl Default for SocketValue {
    fn default() -> Self { Self::Float(0.0) }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SocketDisplayShape {
    Circle,
    Diamond,
    Square,
    Triangle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeLink {
    pub from_node: String,
    pub from_socket: String,
    pub to_node: String,
    pub to_socket: String,
    pub multi: bool,
    pub hide: bool,
    pub select: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeGroup {
    pub name: String,
    pub tree: NodeTree,
    pub inputs: Vec<NodeSocket>,
    pub outputs: Vec<NodeSocket>,
    pub interface: NodeInterface,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInterface {
    pub items: Vec<NodeInterfaceItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInterfaceItem {
    pub name: String,
    pub socket_type: SocketType,
    pub in_out: SocketDirection,
    pub default_value: SocketValue,
    pub min_value: f32,
    pub max_value: f32,
    pub hide_value: bool,
    pub description: String,
}

impl NodeTree {
    pub fn new(name: &str, tree_type: NodeTreeType) -> Self {
        Self {
            name: name.to_string(),
            tree_type,
            nodes: Vec::new(),
            links: Vec::new(),
            active: true,
            view_center: Vec2::ZERO,
            edit_mode: false,
            compact: false,
            nodewidget_width: 250,
        }
    }

    pub fn add_node(&mut self, node_type: NodeType, location: Vec2) -> String {
        let id = format!("node_{}", self.nodes.len());
        let node_type_name = format!("{:?}", node_type);
        let node = Node {
            id: id.clone(),
            node_type,
            name: node_type_name,
            location,
            width: 150.0,
            height: 100.0,
            hide: false,
            mute: false,
            select: false,
            show_options: true,
            show_preview: false,
            show_texture: false,
            use_custom_color: false,
            color: [0.5, 0.5, 0.5],
            inputs: Vec::new(),
            outputs: Vec::new(),
            internal_links: Vec::new(),
            parent: None,
        };
        self.nodes.push(node);
        id
    }

    pub fn add_link(&mut self, from_node: &str, from_socket: &str, to_node: &str, to_socket: &str) {
        self.links.push(NodeLink {
            from_node: from_node.to_string(),
            from_socket: from_socket.to_string(),
            to_node: to_node.to_string(),
            to_socket: to_socket.to_string(),
            multi: false,
            hide: false,
            select: false,
        });
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.links.retain(|l| l.from_node != node_id && l.to_node != node_id);
        self.nodes.retain(|n| n.id != node_id);
    }

    pub fn remove_link(&mut self, index: usize) {
        if index < self.links.len() {
            self.links.remove(index);
        }
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn links_from(&self, node_id: &str) -> Vec<&NodeLink> {
        self.links.iter().filter(|l| l.from_node == node_id).collect()
    }

    pub fn links_to(&self, node_id: &str) -> Vec<&NodeLink> {
        self.links.iter().filter(|l| l.to_node == node_id).collect()
    }

    pub fn is_linked(&self, node_id: &str, socket_name: &str, direction: SocketDirection) -> bool {
        self.links.iter().any(|l| {
            match direction {
                SocketDirection::Input => l.to_node == node_id && l.to_socket == socket_name,
                SocketDirection::Output => l.from_node == node_id && l.from_socket == socket_name,
            }
        })
    }
}

pub struct ShaderNodePresets;

impl ShaderNodePresets {
    pub fn principled_base(color: [f32; 3], roughness: f32, metallic: f32) -> NodeTree {
        let mut tree = NodeTree::new("PrincipledBase", NodeTreeType::Shader);

        let output = tree.add_node(NodeType::ShaderOutput, Vec2::new(400.0, 0.0));
        let principled = tree.add_node(NodeType::PrincipledBSDF, Vec2::new(0.0, 0.0));

        tree.add_link(&principled, "BSDF", &output, "Surface");

        tree
    }

    pub fn glass(color: [f32; 3], roughness: f32, ior: f32) -> NodeTree {
        let mut tree = NodeTree::new("Glass", NodeTreeType::Shader);

        let output = tree.add_node(NodeType::ShaderOutput, Vec2::new(400.0, 0.0));
        let glass = tree.add_node(NodeType::GlassBSDF, Vec2::new(0.0, 0.0));
        let coord = tree.add_node(NodeType::TextureCoordinate, Vec2::new(-400.0, 0.0));

        tree.add_link(&glass, "BSDF", &output, "Surface");
        tree.add_link(&coord, "Generated", &glass, "Normal");

        tree
    }

    pub fn emissive(color: [f32; 3], strength: f32) -> NodeTree {
        let mut tree = NodeTree::new("Emissive", NodeTreeType::Shader);

        let output = tree.add_node(NodeType::ShaderOutput, Vec2::new(400.0, 0.0));
        let emission = tree.add_node(NodeType::Emission, Vec2::new(0.0, 0.0));

        tree.add_link(&emission, "Emission", &output, "Surface");

        tree
    }

    pub fn procedural_noise(scale: f32) -> NodeTree {
        let mut tree = NodeTree::new("ProceduralNoise", NodeTreeType::Shader);

        let output = tree.add_node(NodeType::ShaderOutput, Vec2::new(600.0, 0.0));
        let principled = tree.add_node(NodeType::PrincipledBSDF, Vec2::new(200.0, 0.0));
        let noise = tree.add_node(NodeType::NoiseTexture, Vec2::new(-200.0, 0.0));
        let coord = tree.add_node(NodeType::TextureCoordinate, Vec2::new(-600.0, 0.0));
        let color_ramp = tree.add_node(NodeType::ColorRamp, Vec2::new(0.0, 0.0));

        tree.add_link(&principled, "BSDF", &output, "Surface");
        tree.add_link(&coord, "Generated", &noise, "Vector");
        tree.add_link(&noise, "Color", &color_ramp, "Fac");
        tree.add_link(&color_ramp, "Color", &principled, "Base Color");

        tree
    }
}