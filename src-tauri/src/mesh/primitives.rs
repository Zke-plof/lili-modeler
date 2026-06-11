use super::Mesh;
use glam::{Vec3, Vec2};
use std::f32::consts::PI;

pub fn cube(size: f32) -> Mesh {
    let h = size * 0.5;
    let mut m = Mesh::new();
    
    let verts = [
        [-h, -h, -h], [ h, -h, -h], [ h,  h, -h], [-h,  h, -h],
        [-h, -h,  h], [ h, -h,  h], [ h,  h,  h], [-h,  h,  h],
    ];
    let normals = [
        [0.0, -1.0, 0.0], [0.0,  1.0, 0.0],
        [-1.0, 0.0, 0.0], [1.0,  0.0, 0.0],
        [0.0, 0.0, -1.0], [0.0,  0.0, 1.0],
    ];
    
    for v in verts {
        m.add_vertex(Vec3::from(v));
    }
    
    let faces = [
        vec![0, 1, 2, 3], vec![4, 5, 6, 7],
        vec![0, 1, 5, 4], vec![2, 3, 7, 6],
        vec![0, 3, 7, 4], vec![1, 2, 6, 5],
    ];
    
    for face in faces {
        let id = m.add_face(face);
        let normal = Vec3::from(normals[id as usize]);
        m.faces[id as usize].normal = normal;
        for &vid in &m.faces[id as usize].vertex_ids.clone() {
            m.vertices[vid as usize].normal = normal;
        }
    }
    
    m
}

pub fn sphere(segments: u32, radius: f32) -> Mesh {
    let mut m = Mesh::new();
    let segs = segments.max(3);
    
    m.add_vertex(Vec3::new(0.0, radius, 0.0));
    
    for i in 1..segs {
        let phi = PI * i as f32 / segs as f32;
        for j in 0..segs {
            let theta = 2.0 * PI * j as f32 / segs as f32;
            let x = radius * phi.sin() * theta.cos();
            let y = radius * phi.cos();
            let z = radius * phi.sin() * theta.sin();
            m.add_vertex(Vec3::new(x, y, z));
        }
    }
    
    m.add_vertex(Vec3::new(0.0, -radius, 0.0));
    
    for j in 0..segs {
        let v0 = 0;
        let v1 = 1 + j;
        let v2 = 1 + (j + 1) % segs;
        m.add_face(vec![v0, v2, v1]);
    }
    
    for i in 0..(segs - 2) {
        let base = 1 + i * segs;
        for j in 0..segs {
            let v0 = base + j;
            let v1 = base + (j + 1) % segs;
            let v2 = base + segs + j;
            let v3 = base + segs + (j + 1) % segs;
            m.add_face(vec![v0, v1, v3, v2]);
        }
    }
    
    let last = (m.vertices.len() - 1) as u32;
    let base = 1 + (segs - 2) * segs;
    for j in 0..segs {
        let v0 = base + j;
        let v1 = base + (j + 1) % segs;
        m.add_face(vec![v0, v1, last]);
    }
    
    m.recalculate_normals();
    m
}

pub fn cylinder(segments: u32, radius: f32) -> Mesh {
    let mut m = Mesh::new();
    let height = radius;
    let segs = segments.max(3);
    
    for i in 0..segs {
        let theta = 2.0 * PI * i as f32 / segs as f32;
        let x = radius * theta.cos();
        let z = radius * theta.sin();
        m.add_vertex(Vec3::new(x, height, z));
        m.add_vertex(Vec3::new(x, -height, z));
    }
    
    for i in 0..segs {
        let next = (i + 1) % segs;
        let v0 = (i * 2) as u32;
        let v1 = (i * 2 + 1) as u32;
        let v2 = (next * 2) as u32;
        let v3 = (next * 2 + 1) as u32;
        m.add_face(vec![v0, v2, v3, v1]);
    }
    
    let top_center = m.add_vertex(Vec3::new(0.0, height, 0.0));
    for i in 0..segs {
        let next = (i + 1) % segs;
        m.add_face(vec![top_center, (i * 2) as u32, (next * 2) as u32]);
    }
    
    let bottom_center = m.add_vertex(Vec3::new(0.0, -height, 0.0));
    for i in 0..segs {
        let next = (i + 1) % segs;
        m.add_face(vec![bottom_center, (next * 2 + 1) as u32, (i * 2 + 1) as u32]);
    }
    
    m.recalculate_normals();
    m
}

pub fn torus(segments: u32, radius: f32) -> Mesh {
    let mut m = Mesh::new();
    let major = radius;
    let minor = radius * 0.3;
    let segs = segments.max(4);
    
    for i in 0..segs {
        let theta = 2.0 * PI * i as f32 / segs as f32;
        for j in 0..segs {
            let phi = 2.0 * PI * j as f32 / segs as f32;
            let x = (major + minor * phi.cos()) * theta.cos();
            let y = minor * phi.sin();
            let z = (major + minor * phi.cos()) * theta.sin();
            m.add_vertex(Vec3::new(x, y, z));
        }
    }
    
    for i in 0..segs {
        for j in 0..segs {
            let v0 = (i * segs + j) as u32;
            let v1 = (i * segs + (j + 1) % segs) as u32;
            let v2 = (((i + 1) % segs) * segs + (j + 1) % segs) as u32;
            let v3 = (((i + 1) % segs) * segs + j) as u32;
            m.add_face(vec![v0, v1, v2, v3]);
        }
    }
    
    m.recalculate_normals();
    m
}

pub fn plane(size: f32) -> Mesh {
    let mut m = Mesh::new();
    let h = size * 0.5;
    
    m.add_vertex(Vec3::new(-h, 0.0, -h));
    m.add_vertex(Vec3::new( h, 0.0, -h));
    m.add_vertex(Vec3::new( h, 0.0,  h));
    m.add_vertex(Vec3::new(-h, 0.0,  h));
    
    m.vertices[0].uv = Vec2::new(0.0, 0.0);
    m.vertices[1].uv = Vec2::new(1.0, 0.0);
    m.vertices[2].uv = Vec2::new(1.0, 1.0);
    m.vertices[3].uv = Vec2::new(0.0, 1.0);
    
    m.add_face(vec![0, 1, 2, 3]);
    m.recalculate_normals();
    m
}

pub fn cone(segments: u32, radius: f32) -> Mesh {
    let mut m = Mesh::new();
    let height = radius * 2.0;
    let segs = segments.max(3);
    
    let tip = m.add_vertex(Vec3::new(0.0, height, 0.0));
    
    for i in 0..segs {
        let theta = 2.0 * PI * i as f32 / segs as f32;
        m.add_vertex(Vec3::new(radius * theta.cos(), 0.0, radius * theta.sin()));
    }
    
    for i in 0..segs {
        let next = (i + 1) % segs;
        m.add_face(vec![tip, (i + 1) as u32, (next + 1) as u32]);
    }
    
    let base_center = m.add_vertex(Vec3::new(0.0, 0.0, 0.0));
    for i in 0..segs {
        let next = (i + 1) % segs;
        m.add_face(vec![base_center, (next + 1) as u32, (i + 1) as u32]);
    }
    
    m.recalculate_normals();
    m
}

pub fn monkey() -> Mesh {
    let mut m = Mesh::new();
    
    let head = [
        [-0.5, 1.0, -0.5], [0.5, 1.0, -0.5], [0.5, 1.0, 0.5], [-0.5, 1.0, 0.5],
        [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5],
    ];
    
    for v in head {
        m.add_vertex(Vec3::from(v));
    }
    
    m.add_face(vec![0, 1, 2, 3]);
    m.add_face(vec![4, 5, 6, 7]);
    m.add_face(vec![0, 1, 5, 4]);
    m.add_face(vec![2, 3, 7, 6]);
    m.add_face(vec![0, 3, 7, 4]);
    m.add_face(vec![1, 2, 6, 5]);
    
    let ears = [
        [-0.8, 1.3, 0.0], [0.8, 1.3, 0.0],
        [-0.8, 0.8, 0.0], [0.8, 0.8, 0.0],
    ];
    
    for v in ears {
        m.add_vertex(Vec3::from(v));
    }
    
    m.add_face(vec![8, 10, 11, 9]);
    m.add_face(vec![9, 11, 10, 8]);
    
    let nose = [
        [0.0, 0.6, -0.8], [-0.2, 0.4, -0.8], [0.2, 0.4, -0.8],
    ];
    
    for v in nose {
        m.add_vertex(Vec3::from(v));
    }
    
    m.add_face(vec![12, 13, 14]);
    m.add_face(vec![14, 13, 12]);
    
    let eyes = [
        [-0.3, 0.9, -0.55], [0.3, 0.9, -0.55],
        [-0.2, 0.8, -0.55], [0.2, 0.8, -0.55],
    ];
    
    for v in eyes {
        m.add_vertex(Vec3::from(v));
    }
    
    m.add_face(vec![15, 16, 17, 18]);
    m.add_face(vec![18, 17, 16, 15]);
    
    m.recalculate_normals();
    m
}

pub fn tetrahedron(size: f32) -> Mesh {
    let mut m = Mesh::new();
    let s = size * 0.5;
    let verts = [
        [0.0, s * 1.2, 0.0],
        [-s, -s * 0.4, s * 0.8],
        [s, -s * 0.4, s * 0.8],
        [0.0, -s * 0.4, -s * 1.2],
    ];
    for v in verts {
        m.add_vertex(Vec3::from(v));
    }
    m.add_face(vec![0, 1, 2]);
    m.add_face(vec![0, 2, 3]);
    m.add_face(vec![0, 3, 1]);
    m.add_face(vec![1, 3, 2]);
    m.recalculate_normals();
    m
}

pub fn octahedron(size: f32) -> Mesh {
    let mut m = Mesh::new();
    let s = size * 0.5;
    let verts = [
        [0.0, s, 0.0], [0.0, -s, 0.0],
        [s, 0.0, 0.0], [-s, 0.0, 0.0],
        [0.0, 0.0, s], [0.0, 0.0, -s],
    ];
    for v in verts {
        m.add_vertex(Vec3::from(v));
    }
    m.add_face(vec![0, 4, 2]);
    m.add_face(vec![0, 2, 5]);
    m.add_face(vec![0, 5, 3]);
    m.add_face(vec![0, 3, 4]);
    m.add_face(vec![1, 2, 4]);
    m.add_face(vec![1, 5, 2]);
    m.add_face(vec![1, 3, 5]);
    m.add_face(vec![1, 4, 3]);
    m.recalculate_normals();
    m
}

pub fn icosahedron(size: f32) -> Mesh {
    let mut m = Mesh::new();
    let s = size * 0.5;
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let norm = (1.0 + t * t).sqrt();
    let a = s / norm;
    let b = a * t;

    let verts = [
        [-a, b, 0.0], [a, b, 0.0], [-a, -b, 0.0], [a, -b, 0.0],
        [0.0, -a, b], [0.0, a, b], [0.0, -a, -b], [0.0, a, -b],
        [b, 0.0, -a], [b, 0.0, a], [-b, 0.0, -a], [-b, 0.0, a],
    ];
    for v in verts {
        m.add_vertex(Vec3::from(v));
    }
    let faces: Vec<Vec<u32>> = vec![
        vec![0,11,5], vec![0,5,1], vec![0,1,7], vec![0,7,10], vec![0,10,11],
        vec![1,5,9], vec![5,11,4], vec![11,10,2], vec![10,7,6], vec![7,1,8],
        vec![3,9,4], vec![3,4,2], vec![3,2,6], vec![3,6,8], vec![3,8,9],
        vec![4,9,5], vec![2,4,11], vec![6,2,10], vec![8,6,7], vec![9,8,1],
    ];
    for f in faces {
        m.add_face(f);
    }
    m.recalculate_normals();
    m
}

pub fn dodecahedron(size: f32) -> Mesh {
    let mut m = Mesh::new();
    let s = size * 0.5;
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;

    let verts = [
        [1.0, 1.0, 1.0], [1.0, 1.0, -1.0], [1.0, -1.0, 1.0], [1.0, -1.0, -1.0],
        [-1.0, 1.0, 1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, -1.0, -1.0],
        [0.0, phi, 1.0/phi], [0.0, phi, -1.0/phi], [0.0, -phi, 1.0/phi], [0.0, -phi, -1.0/phi],
        [1.0/phi, 0.0, phi], [1.0/phi, 0.0, -phi], [-1.0/phi, 0.0, phi], [-1.0/phi, 0.0, -phi],
        [phi, 1.0/phi, 0.0], [phi, -1.0/phi, 0.0], [-phi, 1.0/phi, 0.0], [-phi, -1.0/phi, 0.0],
    ];
    let scale = s / phi;
    for v in verts {
        m.add_vertex(Vec3::new(v[0] * scale, v[1] * scale, v[2] * scale));
    }
    let faces: Vec<Vec<u32>> = vec![
        vec![0,16,1,9,8], vec![0,8,4,14,12], vec![0,12,2,17,16],
        vec![1,16,17,3,13], vec![1,13,5,18,9], vec![2,12,14,6,10],
        vec![2,10,11,3,17], vec![4,8,9,18,19], vec![4,19,7,15,14],
        vec![5,13,3,11,19], vec![5,18,9,1,13].into_iter().rev().collect::<Vec<_>>(),
        vec![6,14,15,7,10].into_iter().rev().collect::<Vec<_>>(),
    ];
    for f in faces {
        let center_idx = m.add_vertex(Vec3::new(0.0, -100.0, 0.0));
        let mut center = Vec3::new(0.0, 0.0, 0.0);
        for &vi in &f {
            center += m.vertices[vi as usize].position;
        }
        center /= f.len() as f32;
        m.vertices[center_idx as usize].position = center;
        for i in 0..f.len() {
            let next = (i + 1) % f.len();
            m.add_face(vec![center_idx, f[i], f[next]]);
        }
    }
    m.recalculate_normals();
    m
}

pub fn torus_knot(segments: u32, radius: f32) -> Mesh {
    let mut m = Mesh::new();
    let segs = segments.max(16);
    let tube_radius = radius * 0.15;
    let p = 2;
    let q = 3;
    let tube_segs = 8u32;

    for i in 0..segs {
        let t = 2.0 * PI * i as f32 / segs as f32;
        let r = radius * (2.0 + (q as f32 * t).cos()) / 3.0;
        let x = r * (p as f32 * t).cos();
        let y = r * (p as f32 * t).sin();
        let z = radius * (q as f32 * t).sin() / 3.0;
        let center = Vec3::new(x, y, z);

        let t_next = 2.0 * PI * ((i + 1) % segs) as f32 / segs as f32;
        let r_next = radius * (2.0 + (q as f32 * t_next).cos()) / 3.0;
        let forward = (Vec3::new(r_next * (p as f32 * t_next).cos(), r_next * (p as f32 * t_next).sin(), radius * (q as f32 * t_next).sin() / 3.0) - center).normalize();
        let up = Vec3::Y;
        let right = forward.cross(up).normalize_or_zero();
        let actual_up = right.cross(forward);

        for j in 0..tube_segs {
            let angle = 2.0 * PI * j as f32 / tube_segs as f32;
            let offset = right * angle.cos() + actual_up * angle.sin();
            m.add_vertex(center + offset * tube_radius);
        }
    }

    for i in 0..segs {
        for j in 0..tube_segs {
            let v0 = (i * tube_segs + j) as u32;
            let v1 = (i * tube_segs + (j + 1) % tube_segs) as u32;
            let v2 = (((i + 1) % segs) * tube_segs + (j + 1) % tube_segs) as u32;
            let v3 = (((i + 1) % segs) * tube_segs + j) as u32;
            m.add_face(vec![v0, v1, v2, v3]);
        }
    }

    m.recalculate_normals();
    m
}