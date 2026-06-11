use crate::mesh::Mesh;
use crate::io::ImportOptions;
use glam::{Vec3, Vec2};
use base64::Engine;

pub fn import_obj(data: &[u8], options: &ImportOptions) -> Result<Vec<(String, Mesh)>, String> {
    let text = std::str::from_utf8(data).map_err(|e| e.to_string())?;
    let mut meshes = Vec::new();
    let mut current_mesh = Mesh::new();
    let mut positions = Vec::<Vec3>::new();
    let mut normals = Vec::<Vec3>::new();
    let mut uvs = Vec::<Vec2>::new();
    let mut name = String::from("ImportedOBJ");

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "o" | "g" => {
                if !current_mesh.vertices.is_empty() {
                    meshes.push((name.clone(), current_mesh.clone()));
                    current_mesh = Mesh::new();
                }
                if parts.len() > 1 {
                    name = parts[1].to_string();
                }
            }
            "v" => {
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    let pos = Vec3::new(x, y, z) * options.scale;
                    positions.push(pos);
                    current_mesh.add_vertex(pos);
                }
            }
            "vn" => {
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    let mut normal = Vec3::new(x, y, z);
                    if options.flip_normals {
                        normal = -normal;
                    }
                    normals.push(normal);
                }
            }
            "vt" => {
                if parts.len() >= 3 {
                    let u: f32 = parts[1].parse().unwrap_or(0.0);
                    let v: f32 = parts[2].parse().unwrap_or(0.0);
                    let uv = if options.flip_uv {
                        Vec2::new(u, 1.0 - v)
                    } else {
                        Vec2::new(u, v)
                    };
                    uvs.push(uv);
                }
            }
            "f" => {
                if parts.len() >= 4 {
                    let mut face_verts = Vec::new();
                    for part in &parts[1..] {
                        let indices: Vec<&str> = part.split('/').collect();
                        if let Some(idx) = indices.first() {
                            if let Ok(i) = idx.parse::<i64>() {
                                let vert_idx = if i > 0 { (i - 1) as usize } else { (positions.len() as i64 + i) as usize };
                                if vert_idx < positions.len() {
                                    face_verts.push(vert_idx as u32);
                                }
                            }
                        }
                    }
                    if face_verts.len() >= 3 {
                        current_mesh.add_face(face_verts);
                    }
                }
            }
            _ => {}
        }
    }

    if !current_mesh.vertices.is_empty() {
        meshes.push((name, current_mesh));
    }

    for (_, mesh) in &mut meshes {
        mesh.recalculate_normals();
    }

    Ok(meshes)
}

pub fn import_stl(data: &[u8], options: &ImportOptions) -> Result<Vec<(String, Mesh)>, String> {
    let mut mesh = Mesh::new();
    
    if data.len() >= 84 {
        let num_triangles = u32::from_le_bytes([data[80], data[81], data[82], data[83]]);
        
        for i in 0..num_triangles {
            let offset = (84 + i * 50) as usize;
            if offset + 50 > data.len() {
                break;
            }
            
            let nx = f32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
            let ny = f32::from_le_bytes([data[offset+4], data[offset+5], data[offset+6], data[offset+7]]);
            let nz = f32::from_le_bytes([data[offset+8], data[offset+9], data[offset+10], data[offset+11]]);
            
            let mut normal = Vec3::new(nx, ny, nz);
            if options.flip_normals {
                normal = -normal;
            }
            
            let mut verts = Vec::new();
            for v in 0..3u32 {
                let vo = offset + 12 + v as usize * 12;
                let x = f32::from_le_bytes([data[vo], data[vo+1], data[vo+2], data[vo+3]]);
                let y = f32::from_le_bytes([data[vo+4], data[vo+5], data[vo+6], data[vo+7]]);
                let z = f32::from_le_bytes([data[vo+8], data[vo+9], data[vo+10], data[vo+11]]);
                let pos = Vec3::new(x, y, z) * options.scale;
                verts.push(mesh.add_vertex(pos));
            }
            
            mesh.add_face(verts);
        }
    }
    
    mesh.recalculate_normals();
    Ok(vec![("ImportedSTL".to_string(), mesh)])
}

pub fn import_gltf(data: &[u8], options: &ImportOptions) -> Result<Vec<(String, Mesh)>, String> {
    let mut meshes = Vec::new();
    
    match gltf::Gltf::from_slice(data) {
        Ok(gltf) => {
            let mut buffers = Vec::new();
            for buffer in gltf.buffers() {
                match buffer.source() {
                    gltf::buffer::Source::Uri(uri) => {
                        if uri.starts_with("data:") {
                            let data_str = uri.split(',').nth(1).unwrap_or("");
                            if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(data_str) {
                                buffers.push(decoded);
                            }
                        }
                    }
                    gltf::buffer::Source::Bin => {
                        // Bin buffers are embedded in the GLB; handled separately
                    }
                }
            }
            
            for mesh in gltf.meshes() {
                let mut m = Mesh::new();
                let mut positions = Vec::new();
                let mut normals = Vec::new();
                let mut uvs = Vec::new();
                
                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                    
                    if let Some(pos_iter) = reader.read_positions() {
                        for pos in pos_iter {
                            let p = Vec3::from(pos) * options.scale;
                            positions.push(p);
                            m.add_vertex(p);
                        }
                    }
                    
                    if let Some(norm_iter) = reader.read_normals() {
                        for norm in norm_iter {
                            let mut n = Vec3::from(norm);
                            if options.flip_normals {
                                n = -n;
                            }
                            normals.push(n);
                        }
                    }
                    
                    if let Some(uv_iter) = reader.read_tex_coords(0) {
                        for uv in uv_iter.into_f32() {
                            let mut u = Vec2::from(uv);
                            if options.flip_uv {
                                u.y = 1.0 - u.y;
                            }
                            uvs.push(u);
                        }
                    }
                    
                    if let Some(indices) = reader.read_indices() {
                        let idx_vec: Vec<u32> = indices.into_u32().collect();
                        for tri in idx_vec.chunks(3) {
                            if tri.len() == 3 {
                                let face = vec![tri[0], tri[1], tri[2]];
                                m.add_face(face);
                            }
                        }
                    }
                }
                
                m.recalculate_normals();
                let name = mesh.name().unwrap_or("ImportedGLTF").to_string();
                meshes.push((name, m));
            }
        }
        Err(e) => {
            return Err(format!("Failed to parse glTF: {}", e));
        }
    }
    
    Ok(meshes)
}