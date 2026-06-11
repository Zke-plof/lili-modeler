use crate::mesh::Mesh;
use crate::io::ExportOptions;

pub fn export_obj(meshes: &[(&str, &Mesh)], options: &ExportOptions) -> Result<Vec<u8>, String> {
    let mut output = String::from("# Exported by Lili Modeler\n");
    let mut vertex_offset = 0;

    for (name, mesh) in meshes {
        output.push_str(&format!("o {}\n", name));

        for vert in &mesh.vertices {
            let pos = vert.position * options.scale;
            output.push_str(&format!("v {} {} {}\n", pos.x, pos.y, pos.z));
        }

        if options.include_uv {
            for vert in &mesh.vertices {
                output.push_str(&format!("vt {} {}\n", vert.uv.x, vert.uv.y));
            }
        }

        if options.include_normals {
            for vert in &mesh.vertices {
                let mut normal = vert.normal;
                if options.flip_normals {
                    normal = -normal;
                }
                output.push_str(&format!("vn {} {} {}\n", normal.x, normal.y, normal.z));
            }
        }

        for face in &mesh.faces {
            let mut face_str = String::from("f ");
            for &vid in &face.vertex_ids {
                let idx = vid as usize + 1 + vertex_offset;
                if options.include_uv && options.include_normals {
                    face_str.push_str(&format!("{}/{}/{} ", idx, idx, idx));
                } else if options.include_normals {
                    face_str.push_str(&format!("{}/{} ", idx, idx));
                } else {
                    face_str.push_str(&format!("{} ", idx));
                }
            }
            face_str.push('\n');
            output.push_str(&face_str);
        }

        vertex_offset += mesh.vertices.len();
    }

    Ok(output.into_bytes())
}

pub fn export_stl(meshes: &[(&str, &Mesh)], options: &ExportOptions) -> Result<Vec<u8>, String> {
    let mut output = vec![0u8; 84];

    let mut triangles = 0u32;
    for (_, mesh) in meshes {
        for face in &mesh.faces {
            if face.vertex_ids.len() >= 3 {
                triangles += (face.vertex_ids.len() - 2) as u32;
            }
        }
    }

    output[80..84].copy_from_slice(&triangles.to_le_bytes());

    for (_, mesh) in meshes {
        for face in &mesh.faces {
            if face.vertex_ids.len() < 3 {
                continue;
            }

            let normal = face.normal;
            let mut normal_bytes = [0u8; 12];
            normal_bytes[0..4].copy_from_slice(&normal.x.to_le_bytes());
            normal_bytes[4..8].copy_from_slice(&normal.y.to_le_bytes());
            normal_bytes[8..12].copy_from_slice(&normal.z.to_le_bytes());
            output.extend_from_slice(&normal_bytes);

            for &vid in &face.vertex_ids {
                let pos = mesh.vertices[vid as usize].position * options.scale;
                let mut pos_bytes = [0u8; 12];
                pos_bytes[0..4].copy_from_slice(&pos.x.to_le_bytes());
                pos_bytes[4..8].copy_from_slice(&pos.y.to_le_bytes());
                pos_bytes[8..12].copy_from_slice(&pos.z.to_le_bytes());
                output.extend_from_slice(&pos_bytes);
            }

            output.extend_from_slice(&[0u8; 2]);
        }
    }

    Ok(output)
}

pub fn export_gltf(meshes: &[(&str, &Mesh)], options: &ExportOptions) -> Result<Vec<u8>, String> {
    let mut all_positions = Vec::new();
    let mut all_normals = Vec::new();
    let mut all_indices = Vec::new();
    let mut accessors = Vec::new();
    let mut buffer_views = Vec::new();
    let mut mesh_defs = Vec::new();

    let mut byte_offset = 0usize;

    for (name, mesh) in meshes {
        let vertex_start = all_positions.len() / 3;
        let index_start = all_indices.len();

        for vert in &mesh.vertices {
            let pos = vert.position * options.scale;
            all_positions.extend_from_slice(&[pos.x, pos.y, pos.z]);
            let mut normal = vert.normal;
            if options.flip_normals {
                normal = -normal;
            }
            all_normals.extend_from_slice(&[normal.x, normal.y, normal.z]);
        }

        for face in &mesh.faces {
            for i in 1..face.vertex_ids.len() - 1 {
                all_indices.push((vertex_start + face.vertex_ids[0] as usize) as u32);
                all_indices.push((vertex_start + face.vertex_ids[i] as usize) as u32);
                all_indices.push((vertex_start + face.vertex_ids[i + 1] as usize) as u32);
            }
        }

        let pos_bytes = all_positions[all_positions.len() - mesh.vertices.len() * 3..]
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect::<Vec<_>>();
        let norm_bytes = all_normals[all_normals.len() - mesh.vertices.len() * 3..]
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect::<Vec<_>>();
        let idx_bytes = all_indices[index_start..]
            .iter()
            .flat_map(|i| i.to_le_bytes())
            .collect::<Vec<_>>();

        let pos_bv_idx = buffer_views.len();
        buffer_views.push(serde_json::json!({
            "buffer": 0,
            "byteOffset": byte_offset,
            "byteLength": pos_bytes.len(),
            "target": 34962
        }));
        byte_offset += pos_bytes.len();

        let norm_bv_idx = buffer_views.len();
        buffer_views.push(serde_json::json!({
            "buffer": 0,
            "byteOffset": byte_offset,
            "byteLength": norm_bytes.len(),
            "target": 34962
        }));
        byte_offset += norm_bytes.len();

        let idx_bv_idx = buffer_views.len();
        buffer_views.push(serde_json::json!({
            "buffer": 0,
            "byteOffset": byte_offset,
            "byteLength": idx_bytes.len(),
            "target": 34963
        }));
        byte_offset += idx_bytes.len();

        let min_pos = [
            mesh.vertices.iter().map(|v| v.position.x).fold(f32::INFINITY, f32::min) * options.scale,
            mesh.vertices.iter().map(|v| v.position.y).fold(f32::INFINITY, f32::min) * options.scale,
            mesh.vertices.iter().map(|v| v.position.z).fold(f32::INFINITY, f32::min) * options.scale,
        ];
        let max_pos = [
            mesh.vertices.iter().map(|v| v.position.x).fold(f32::NEG_INFINITY, f32::max) * options.scale,
            mesh.vertices.iter().map(|v| v.position.y).fold(f32::NEG_INFINITY, f32::max) * options.scale,
            mesh.vertices.iter().map(|v| v.position.z).fold(f32::NEG_INFINITY, f32::max) * options.scale,
        ];

        let pos_acc_idx = accessors.len();
        accessors.push(serde_json::json!({
            "bufferView": pos_bv_idx,
            "componentType": 5126,
            "count": mesh.vertices.len(),
            "type": "VEC3",
            "min": min_pos,
            "max": max_pos
        }));

        let norm_acc_idx = accessors.len();
        accessors.push(serde_json::json!({
            "bufferView": norm_bv_idx,
            "componentType": 5126,
            "count": mesh.vertices.len(),
            "type": "VEC3"
        }));

        let idx_acc_idx = accessors.len();
        let num_idx = all_indices.len() - index_start;
        accessors.push(serde_json::json!({
            "bufferView": idx_bv_idx,
            "componentType": 5125,
            "count": num_idx,
            "type": "SCALAR"
        }));

        mesh_defs.push(serde_json::json!({
            "name": name,
            "primitives": [{
                "attributes": {
                    "POSITION": pos_acc_idx,
                    "NORMAL": norm_acc_idx
                },
                "indices": idx_acc_idx
            }]
        }));
    }

    let gltf = serde_json::json!({
        "asset": { "version": "2.0", "generator": "Lili Modeler" },
        "scene": 0,
        "scenes": [{ "nodes": [0] }],
        "nodes": [{ "mesh": 0 }],
        "meshes": mesh_defs,
        "accessors": accessors,
        "bufferViews": buffer_views,
        "buffers": [{
            "byteLength": byte_offset
        }]
    });

    let mut glb = Vec::new();
    glb.extend_from_slice(&[0x67, 0x6C, 0x54, 0x46]);
    glb.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]);

    let json_bytes = serde_json::to_vec(&gltf).map_err(|e| e.to_string())?;
    let json_padded_len = (json_bytes.len() + 3) & !3;
    let mut json_chunk = vec![0u8; json_padded_len];
    json_chunk[..json_bytes.len()].copy_from_slice(&json_bytes);

    glb.extend_from_slice(&(json_chunk.len() as u32).to_le_bytes());
    glb.extend_from_slice(&[0x4A, 0x53, 0x4F, 0x4E]);
    glb.extend_from_slice(&json_chunk);

    let mut bin_data = Vec::new();
    for (name, mesh) in meshes {
        for vert in &mesh.vertices {
            let pos = vert.position * options.scale;
            bin_data.extend_from_slice(&pos.x.to_le_bytes());
            bin_data.extend_from_slice(&pos.y.to_le_bytes());
            bin_data.extend_from_slice(&pos.z.to_le_bytes());
            let mut normal = vert.normal;
            if options.flip_normals {
                normal = -normal;
            }
            bin_data.extend_from_slice(&normal.x.to_le_bytes());
            bin_data.extend_from_slice(&normal.y.to_le_bytes());
            bin_data.extend_from_slice(&normal.z.to_le_bytes());
        }
        for face in &mesh.faces {
            for i in 1..face.vertex_ids.len() - 1 {
                bin_data.extend_from_slice(&face.vertex_ids[0].to_le_bytes());
                bin_data.extend_from_slice(&face.vertex_ids[i].to_le_bytes());
                bin_data.extend_from_slice(&face.vertex_ids[i + 1].to_le_bytes());
            }
        }
    }

    let bin_padded_len = (bin_data.len() + 3) & !3;
    let mut bin_chunk = vec![0u8; bin_padded_len];
    bin_chunk[..bin_data.len()].copy_from_slice(&bin_data);

    glb.extend_from_slice(&(bin_chunk.len() as u32).to_le_bytes());
    glb.extend_from_slice(&[0x42, 0x49, 0x4E, 0x00]);
    glb.extend_from_slice(&bin_chunk);

    let total_len = glb.len() as u32;
    glb[8..12].copy_from_slice(&total_len.to_le_bytes());

    Ok(glb)
}