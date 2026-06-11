use super::{Engine, SelectMode};
use crate::mesh::{Mesh, primitives, Vertex};
use crate::scene::SceneObject;
use crate::io::{importers, exporters, ImportOptions, ExportOptions};
use glam::{Vec3, Quat};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrimitiveArgs {
    pub name: String,
    pub primitive_type: PrimitiveType,
    pub segments: u32,
    pub size: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveType {
    Cube,
    Sphere,
    Cylinder,
    Torus,
    Plane,
    Cone,
    Monkey,
    Tetrahedron,
    Octahedron,
    Icosahedron,
    Dodecahedron,
    Torusknot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformArgs {
    pub object_id: String,
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrudeArgs {
    pub distance: f32,
    pub face_ids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsetArgs {
    pub thickness: f32,
    pub face_ids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BevelArgs {
    pub segments: u32,
    pub edge_ids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopCutArgs {
    pub cuts: u32,
    pub edge_loop_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BooleanOp {
    Union,
    Difference,
    Intersect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanArgs {
    pub target_id: String,
    pub operator_id: String,
    pub operation: BooleanOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportArgs {
    pub file_path: String,
    pub options: ImportOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportArgs {
    pub file_path: String,
    pub object_ids: Vec<String>,
    pub options: ExportOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraArgs {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintArgs {
    pub object_id: String,
    pub constraint_type: ConstraintType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    Position { axis: [bool; 3] },
    Rotation { axis: [bool; 3] },
    Scale { axis: [bool; 3] },
    Fixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureArgs {
    pub point_a: [f32; 3],
    pub point_b: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshInfo {
    pub vertex_count: u32,
    pub edge_count: u32,
    pub face_count: u32,
    pub triangle_count: u32,
    pub volume: f32,
    pub surface_area: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    pub objects: Vec<SceneObjectData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneObjectData {
    pub id: String,
    pub name: String,
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
    pub vertex_count: u32,
    pub face_count: u32,
    pub visible: bool,
}

// ═══════════════════════════════════════════════════════════════════
// Commands
// ═══════════════════════════════════════════════════════════════════

#[tauri::command]
pub fn create_primitive(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: CreatePrimitiveArgs,
) -> Result<String, String> {
    let mesh = match args.primitive_type {
        PrimitiveType::Cube => primitives::cube(args.size),
        PrimitiveType::Sphere => primitives::sphere(args.segments, args.size),
        PrimitiveType::Cylinder => primitives::cylinder(args.segments, args.size),
        PrimitiveType::Torus => primitives::torus(args.segments, args.size),
        PrimitiveType::Plane => primitives::plane(args.size),
        PrimitiveType::Cone => primitives::cone(args.segments, args.size),
        PrimitiveType::Monkey => primitives::monkey(),
        PrimitiveType::Tetrahedron => primitives::tetrahedron(args.size),
        PrimitiveType::Octahedron => primitives::octahedron(args.size),
        PrimitiveType::Icosahedron => primitives::icosahedron(args.size),
        PrimitiveType::Dodecahedron => primitives::dodecahedron(args.size),
        PrimitiveType::Torusknot => primitives::torus_knot(args.segments, args.size),
    };

    let mut engine = state.write();
    let id = engine.add_mesh(&args.name, mesh);
    Ok(id.to_string())
}

#[tauri::command]
pub fn delete_object(
    state: State<'_, parking_lot::RwLock<Engine>>,
    object_id: String,
) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&object_id).map_err(|e| e.to_string())?;
    let mut engine = state.write();
    engine.remove_mesh(&id).ok_or("Object not found")?;
    Ok(())
}

#[tauri::command]
pub fn transform_object(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: TransformArgs,
) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&args.object_id).map_err(|e| e.to_string())?;
    let mut engine = state.write();
    let mut scene = engine.scene.write();
    
    if let Some(obj) = scene.objects.get_mut(&id) {
        obj.position = Vec3::from(args.position);
        obj.rotation = Quat::from_xyzw(args.rotation[0], args.rotation[1], args.rotation[2], args.rotation[3]);
        obj.scale = Vec3::from(args.scale);
        Ok(())
    } else {
        Err("Object not found".into())
    }
}

#[tauri::command]
pub fn set_selection_mode(
    state: State<'_, parking_lot::RwLock<Engine>>,
    mode: SelectMode,
) -> Result<(), String> {
    let mut engine = state.write();
    engine.selection.mode = mode;
    Ok(())
}

#[tauri::command]
pub fn extrude_selection(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: ExtrudeArgs,
) -> Result<(), String> {
    let mut engine = state.write();
    let mut scene = engine.scene.write();
    
    for obj in scene.objects.values_mut() {
        if !args.face_ids.is_empty() {
            obj.mesh = obj.mesh.extrude_faces(&args.face_ids, args.distance);
        }
    }
    
    Ok(())
}

#[tauri::command]
pub fn inset_faces(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: InsetArgs,
) -> Result<(), String> {
    let mut engine = state.write();
    let mut scene = engine.scene.write();
    
    for obj in scene.objects.values_mut() {
        if !args.face_ids.is_empty() {
            obj.mesh = obj.mesh.inset_faces(&args.face_ids, args.thickness);
        }
    }
    
    Ok(())
}

#[tauri::command]
pub fn bevel_edges(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: BevelArgs,
) -> Result<(), String> {
    let mut engine = state.write();
    let mut scene = engine.scene.write();
    
    for obj in scene.objects.values_mut() {
        if !args.edge_ids.is_empty() {
            obj.mesh = obj.mesh.bevel_edges(&args.edge_ids, args.segments);
        }
    }
    
    Ok(())
}

#[tauri::command]
pub fn loop_cut(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: LoopCutArgs,
) -> Result<(), String> {
    let mut engine = state.write();
    let mut scene = engine.scene.write();
    
    for obj in scene.objects.values_mut() {
        obj.mesh = obj.mesh.loop_cut(args.edge_loop_id, args.cuts);
    }
    
    Ok(())
}

#[tauri::command]
pub fn boolean_operation(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: BooleanArgs,
) -> Result<(), String> {
    let target_id = uuid::Uuid::parse_str(&args.target_id).map_err(|e| e.to_string())?;
    let operator_id = uuid::Uuid::parse_str(&args.operator_id).map_err(|e| e.to_string())?;
    
    let mut engine = state.write();
    let mut scene = engine.scene.write();
    
    let operator_mesh = scene.objects.remove(&operator_id)
        .ok_or("Operator object not found")?
        .mesh;
    
    if let Some(target_obj) = scene.objects.get_mut(&target_id) {
        target_obj.mesh = match args.operation {
            BooleanOp::Union => target_obj.mesh.boolean_union(&operator_mesh),
            BooleanOp::Difference => target_obj.mesh.boolean_difference(&operator_mesh),
            BooleanOp::Intersect => target_obj.mesh.boolean_intersect(&operator_mesh),
        };
    }
    
    Ok(())
}

#[tauri::command]
pub async fn import_mesh(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: ImportArgs,
) -> Result<Vec<String>, String> {
    let data = std::fs::read(&args.file_path).map_err(|e| e.to_string())?;
    let extension = std::path::Path::new(&args.file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    let meshes = match extension.to_lowercase().as_str() {
        "obj" => importers::import_obj(&data, &args.options),
        "stl" => importers::import_stl(&data, &args.options),
        "gltf" | "glb" => importers::import_gltf(&data, &args.options),
        _ => return Err(format!("Unsupported format: {}", extension)),
    }.map_err(|e| e.to_string())?;
    
    let mut engine = state.write();
    let mut ids = Vec::new();
    
    for (name, mesh) in meshes {
        let id = engine.add_mesh(&name, mesh);
        ids.push(id.to_string());
    }
    
    Ok(ids)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub id: String,
    pub name: String,
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub vertex_count: u32,
    pub face_count: u32,
}

#[tauri::command]
pub async fn get_mesh_data(
    state: State<'_, parking_lot::RwLock<Engine>>,
    object_id: String,
) -> Result<ImportResult, String> {
    let id = uuid::Uuid::parse_str(&object_id).map_err(|e| e.to_string())?;
    let engine = state.read();
    let scene = engine.scene.read();

    let obj = scene.objects.get(&id).ok_or("Object not found")?;
    let mesh = &obj.mesh;

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    for vert in &mesh.vertices {
        positions.push(vert.position.x);
        positions.push(vert.position.y);
        positions.push(vert.position.z);
        normals.push(vert.normal.x);
        normals.push(vert.normal.y);
        normals.push(vert.normal.z);
    }

    let mut indices = Vec::new();
    for face in &mesh.faces {
        let vids = &face.vertex_ids;
        if vids.len() < 3 { continue; }
        for i in 1..vids.len() - 1 {
            indices.push(vids[0]);
            indices.push(vids[i]);
            indices.push(vids[i + 1]);
        }
    }

    Ok(ImportResult {
        id: id.to_string(),
        name: obj.name.clone(),
        positions,
        indices,
        normals,
        vertex_count: mesh.vertices.len() as u32,
        face_count: mesh.faces.len() as u32,
    })
}

#[tauri::command]
pub async fn export_mesh(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: ExportArgs,
) -> Result<(), String> {
    let engine = state.read();
    let scene = engine.scene.read();
    
    let mut name_mesh_pairs: Vec<(String, &Mesh)> = Vec::new();
    for id_str in &args.object_ids {
        let id = uuid::Uuid::parse_str(id_str).map_err(|e| e.to_string())?;
        if let Some(obj) = scene.objects.get(&id) {
            name_mesh_pairs.push((obj.name.clone(), &obj.mesh));
        }
    }
    let meshes: Vec<(&str, &Mesh)> = name_mesh_pairs.iter()
        .map(|(name, mesh)| (name.as_str(), *mesh))
        .collect();
    
    let extension = std::path::Path::new(&args.file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    let data = match extension.to_lowercase().as_str() {
        "obj" => exporters::export_obj(&meshes, &args.options),
        "stl" => exporters::export_stl(&meshes, &args.options),
        "gltf" | "glb" => exporters::export_gltf(&meshes, &args.options),
        _ => return Err(format!("Unsupported format: {}", extension)),
    }.map_err(|e| e.to_string())?;
    
    std::fs::write(&args.file_path, data).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub fn get_scene_data(state: State<'_, parking_lot::RwLock<Engine>>) -> Result<SceneData, String> {
    let engine = state.read();
    let scene = engine.scene.read();
    
    let objects: Vec<SceneObjectData> = scene.objects.values().map(|obj| {
        SceneObjectData {
            id: obj.id.to_string(),
            name: obj.name.clone(),
            position: [obj.position.x, obj.position.y, obj.position.z],
            rotation: [obj.rotation.x, obj.rotation.y, obj.rotation.z, obj.rotation.w],
            scale: [obj.scale.x, obj.scale.y, obj.scale.z],
            vertex_count: obj.mesh.vertices.len() as u32,
            face_count: obj.mesh.faces.len() as u32,
            visible: obj.visible,
        }
    }).collect();
    
    Ok(SceneData { objects })
}

#[tauri::command]
pub fn set_camera(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: CameraArgs,
) -> Result<(), String> {
    let mut engine = state.write();
    let mut scene = engine.scene.write();
    
    scene.camera.position = Vec3::from(args.position);
    scene.camera.target = Vec3::from(args.target);
    scene.camera.fov = args.fov;
    scene.camera.near = args.near;
    scene.camera.far = args.far;
    
    Ok(())
}

#[tauri::command]
pub fn add_constraint(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: ConstraintArgs,
) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&args.object_id).map_err(|e| e.to_string())?;
    let mut engine = state.write();
    let mut scene = engine.scene.write();
    
    if let Some(obj) = scene.objects.get_mut(&id) {
        obj.constraints.push(args.constraint_type);
        Ok(())
    } else {
        Err("Object not found".into())
    }
}

#[tauri::command]
pub fn measure_distance(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: MeasureArgs,
) -> Result<f32, String> {
    let a = Vec3::from(args.point_a);
    let b = Vec3::from(args.point_b);
    Ok(a.distance(b))
}

#[tauri::command]
pub fn measure_angle(
    state: State<'_, parking_lot::RwLock<Engine>>,
    args: [MeasureArgs; 2],
) -> Result<f32, String> {
    let a = Vec3::from(args[0].point_a) - Vec3::from(args[0].point_b);
    let b = Vec3::from(args[1].point_a) - Vec3::from(args[1].point_b);
    let angle = a.angle_between(b);
    Ok(angle.to_degrees())
}

#[tauri::command]
pub fn get_mesh_info(
    state: State<'_, parking_lot::RwLock<Engine>>,
    object_id: String,
) -> Result<MeshInfo, String> {
    let id = uuid::Uuid::parse_str(&object_id).map_err(|e| e.to_string())?;
    let engine = state.read();
    let scene = engine.scene.read();
    
    let obj = scene.objects.get(&id).ok_or("Object not found")?;
    let mesh = &obj.mesh;
    
    let triangle_count: u32 = mesh.faces.iter()
        .map(|face| face.vertex_ids.len() as u32 - 2)
        .sum();
    
    let volume = mesh.calculate_volume();
    let surface_area = mesh.calculate_surface_area();
    
    Ok(MeshInfo {
        vertex_count: mesh.vertices.len() as u32,
        edge_count: mesh.edges.len() as u32,
        face_count: mesh.faces.len() as u32,
        triangle_count,
        volume,
        surface_area,
    })
}