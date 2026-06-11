use glam::{Vec3, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HEdgeId(pub u32);
impl From<u32> for HEdgeId { fn from(v: u32) -> Self { Self(v) } }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HVertId(pub u32);
impl From<u32> for HVertId { fn from(v: u32) -> Self { Self(v) } }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HFaceId(pub u32);
impl From<u32> for HFaceId { fn from(v: u32) -> Self { Self(v) } }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HLoopId(pub u32);
impl From<u32> for HLoopId { fn from(v: u32) -> Self { Self(v) } }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HVert {
    pub id: HVertId,
    pub position: Vec3,
    pub normal: Vec3,
    pub co_normals: Vec3,
    pub uv: Vec2,
    pub color: [f32; 4],
    pub head_edge: Option<HEdgeId>,
    pub select: SelectState,
    pub hide: bool,
    pub bevel_weight: f32,
    pub groups: Vec<VertexGroupWeight>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HEdge {
    pub id: HEdgeId,
    pub vert: HVertId,
    pub face: Option<HFaceId>,
    pub next: Option<HEdgeId>,
    pub prev: Option<HEdgeId>,
    pub radial: Option<HEdgeId>,
    pub select: SelectState,
    pub hide: bool,
    pub seam: bool,
    pub sharp: bool,
    crease: f32,
    bevel_weight: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HFace {
    pub id: HFaceId,
    pub loop_start: Option<HLoopId>,
    pub loop_total: u32,
    pub normal: Vec3,
    pub select: SelectState,
    pub hide: bool,
    pub material_index: u32,
    pub smooth: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HLoop {
    pub id: HLoopId,
    pub vert: HVertId,
    pub edge: HEdgeId,
    pub face: HFaceId,
    pub uv: Vec2,
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectState {
    #[default]
    None,
    Select,
    SelectFlush,
}

impl SelectState {
    pub fn is_selected(&self) -> bool {
        *self == SelectState::Select || *self == SelectState::SelectFlush
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VertexGroupWeight {
    pub group: u32,
    pub weight: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BMesh {
    pub verts: Vec<HVert>,
    pub edges: Vec<HEdge>,
    pub faces: Vec<HFace>,
    pub loops: Vec<HLoop>,
    pub vert_pool: SlotPool<HVertId>,
    pub edge_pool: SlotPool<HEdgeId>,
    pub face_pool: SlotPool<HFaceId>,
    pub loop_pool: SlotPool<HLoopId>,
    pub name: String,
    pub select_mode: SelectMode,
    pub total_select_verts: u32,
    pub total_select_edges: u32,
    pub total_select_faces: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectMode {
    #[default]
    Vertex,
    Edge,
    Face,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlotPool<T: Clone> {
    free: Vec<T>,
    next_id: u32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + From<u32>> SlotPool<T> {
    pub fn new() -> Self {
        Self {
            free: Vec::new(),
            next_id: 0,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn alloc(&mut self) -> T {
        if let Some(id) = self.free.pop() {
            id
        } else {
            let id = T::from(self.next_id);
            self.next_id += 1;
            id
        }
    }
    pub fn free(&mut self, id: T) {
        self.free.push(id);
    }
}

impl Default for SlotPool<HEdgeId> {
    fn default() -> Self { Self::new() }
}
impl Default for SlotPool<HVertId> {
    fn default() -> Self { Self::new() }
}
impl Default for SlotPool<HFaceId> {
    fn default() -> Self { Self::new() }
}
impl Default for SlotPool<HLoopId> {
    fn default() -> Self { Self::new() }
}

impl BMesh {
    pub fn new() -> Self {
        Self {
            verts: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            loops: Vec::new(),
            vert_pool: SlotPool::new(),
            edge_pool: SlotPool::new(),
            face_pool: SlotPool::new(),
            loop_pool: SlotPool::new(),
            name: String::new(),
            select_mode: SelectMode::Vertex,
            total_select_verts: 0,
            total_select_edges: 0,
            total_select_faces: 0,
        }
    }

    pub fn vert(&self, id: HVertId) -> &HVert {
        &self.verts[id.0 as usize]
    }

    pub fn vert_mut(&mut self, id: HVertId) -> &mut HVert {
        &mut self.verts[id.0 as usize]
    }

    pub fn edge(&self, id: HEdgeId) -> &HEdge {
        &self.edges[id.0 as usize]
    }

    pub fn edge_mut(&mut self, id: HEdgeId) -> &mut HEdge {
        &mut self.edges[id.0 as usize]
    }

    pub fn face(&self, id: HFaceId) -> &HFace {
        &self.faces[id.0 as usize]
    }

    pub fn face_mut(&mut self, id: HFaceId) -> &mut HFace {
        &mut self.faces[id.0 as usize]
    }

    pub fn loop_(&self, id: HLoopId) -> &HLoop {
        &self.loops[id.0 as usize]
    }

    pub fn loop_mut(&mut self, id: HLoopId) -> &mut HLoop {
        &mut self.loops[id.0 as usize]
    }

    pub fn add_vert(&mut self, position: Vec3) -> HVertId {
        let id = HVertId(self.verts.len() as u32);
        self.verts.push(HVert {
            id,
            position,
            normal: Vec3::Y,
            co_normals: Vec3::Y,
            uv: Vec2::ZERO,
            color: [0.8, 0.8, 0.8, 1.0],
            head_edge: None,
            select: SelectState::None,
            hide: false,
            bevel_weight: 0.0,
            groups: Vec::new(),
        });
        id
    }

    pub fn add_edge(&mut self, v0: HVertId, v1: HVertId) -> HEdgeId {
        let id = HEdgeId(self.edges.len() as u32);
        self.edges.push(HEdge {
            id,
            vert: v0,
            face: None,
            next: None,
            prev: None,
            radial: None,
            select: SelectState::None,
            hide: false,
            seam: false,
            sharp: false,
            crease: 0.0,
            bevel_weight: 0.0,
        });

        if self.vert(v0).head_edge.is_none() {
            self.vert_mut(v0).head_edge = Some(id);
        }
        id
    }

    pub fn add_face(&mut self, vert_ids: &[HVertId]) -> HFaceId {
        if vert_ids.len() < 3 {
            panic!("Face needs at least 3 vertices");
        }

        let face_id = HFaceId(self.faces.len() as u32);
        let loop_start = HLoopId(self.loops.len() as u32);

        let mut edge_ids = Vec::new();
        for i in 0..vert_ids.len() {
            let v0 = vert_ids[i];
            let v1 = vert_ids[(i + 1) % vert_ids.len()];

            let existing_edge = self.find_edge(v0, v1);
            let edge_id = if let Some(eid) = existing_edge {
                eid
            } else {
                self.add_edge(v0, v1)
            };

            let lid = HLoopId(self.loops.len() as u32);
            self.loops.push(HLoop {
                id: lid,
                vert: v0,
                edge: edge_id,
                face: face_id,
                uv: self.vert(v0).uv,
                color: self.vert(v0).color,
            });

            let edge = self.edge_mut(edge_id);
            edge.face = Some(face_id);
            edge_ids.push(edge_id);
        }

        for i in 0..edge_ids.len() {
            let next_idx = (i + 1) % edge_ids.len();
            let curr = edge_ids[i];
            let next = edge_ids[next_idx];
            self.edge_mut(curr).next = Some(next);
            self.edge_mut(next).prev = Some(curr);
        }

        let normal = self.calculate_face_normal(&vert_ids);

        self.faces.push(HFace {
            id: face_id,
            loop_start: Some(loop_start),
            loop_total: vert_ids.len() as u32,
            normal,
            select: SelectState::None,
            hide: false,
            material_index: 0,
            smooth: false,
        });

        face_id
    }

    pub fn find_edge(&self, v0: HVertId, v1: HVertId) -> Option<HEdgeId> {
        let mut e = self.vert(v0).head_edge;
        let start = e;
        loop {
            match e {
                Some(eid) => {
                    let edge = self.edge(eid);
                    if edge.vert == v0 {
                        let next_vert = self.edge(edge.next.unwrap_or(eid)).vert;
                        if next_vert == v1 {
                            return Some(eid);
                        }
                    }
                    if edge.vert == v1 {
                        let next_vert = self.edge(edge.next.unwrap_or(eid)).vert;
                        if next_vert == v0 {
                            return Some(eid);
                        }
                    }
                    e = edge.next;
                    if e == start { break; }
                }
                None => break,
            }
        }
        None
    }

    pub fn vert_edges(&self, vid: HVertId) -> Vec<HEdgeId> {
        let mut result = Vec::new();
        let mut e = self.vert(vid).head_edge;
        let start = e;
        loop {
            match e {
                Some(eid) => {
                    result.push(eid);
                    let edge = self.edge(eid);
                    e = edge.next;
                    if e == start { break; }
                }
                None => break,
            }
        }
        result
    }

    pub fn vert_faces(&self, vid: HVertId) -> Vec<HFaceId> {
        self.vert_edges(vid).iter()
            .filter_map(|eid| self.edge(*eid).face)
            .collect()
    }

    pub fn face_verts(&self, fid: HFaceId) -> Vec<HVertId> {
        let face = self.face(fid);
        let mut result = Vec::new();
        if let Some(start) = face.loop_start {
            let mut lid = start;
            loop {
                result.push(self.loop_(lid).vert);
                let l = self.loop_(lid);
                lid = match l.id.0 + 1 < self.loops.len() as u32 {
                    true => HLoopId(l.id.0 + 1),
                    false => break,
                };
                if !self.loop_(lid).face.eq(&fid) { break; }
            }
        }
        result
    }

    pub fn face_edges(&self, fid: HFaceId) -> Vec<HEdgeId> {
        let face = self.face(fid);
        let mut result = Vec::new();
        if let Some(start) = face.loop_start {
            let mut lid = start;
            for _ in 0..face.loop_total {
                result.push(self.loop_(lid).edge);
                let l = self.loop_(lid);
                lid = match self.loop_(lid).id.0 + 1 < self.loops.len() as u32 {
                    true => HLoopId(l.id.0 + 1),
                    false => break,
                };
            }
        }
        result
    }

    pub fn calculate_face_normal(&self, vert_ids: &[HVertId]) -> Vec3 {
        if vert_ids.len() < 3 { return Vec3::Y; }
        let v0 = self.vert(vert_ids[0]).position;
        let v1 = self.vert(vert_ids[1]).position;
        let v2 = self.vert(vert_ids[2]).position;
        (v1 - v0).cross(v2 - v0).normalize_or_zero()
    }

    pub fn recalculate_normals(&mut self) {
        let mut vert_normals = vec![Vec3::ZERO; self.verts.len()];

        for face in &self.faces {
            if face.hide { continue; }
            let normal = face.normal;
            if let Some(start) = face.loop_start {
                let mut lid = start;
                for _ in 0..face.loop_total {
                    let vid = self.loop_(lid).vert;
                    if !self.vert(vid).hide {
                        vert_normals[vid.0 as usize] += normal;
                    }
                    let next = self.loop_(lid).id.0 + 1;
                    if next >= self.loops.len() as u32 { break; }
                    lid = HLoopId(next);
                }
            }
        }

        for (i, vert) in self.verts.iter_mut().enumerate() {
            vert.normal = vert_normals[i].normalize_or_zero();
            vert.co_normals = vert.normal;
        }

        let face_info: Vec<(Option<HLoopId>, HFaceId)> = self.faces.iter()
            .map(|f| (f.loop_start, f.id))
            .collect();
        for (loop_start, fid) in face_info {
            let _vid0 = self.loop_(loop_start.unwrap()).vert;
            let vids = self.face_verts(fid);
            let normal = self.calculate_face_normal(&vids);
            self.face_mut(fid).normal = normal;
        }
    }

    pub fn select_all() {
    }

    pub fn select_flush(&mut self) {
        self.total_select_verts = 0;
        self.total_select_edges = 0;
        self.total_select_faces = 0;

        match self.select_mode {
            SelectMode::Vertex => {
                for vert in &mut self.verts {
                    if vert.select.is_selected() {
                        self.total_select_verts += 1;
                    }
                }
            }
            SelectMode::Edge => {
                let edge_info: Vec<(HEdgeId, bool)> = self.edges.iter()
                    .map(|e| (e.id, e.select.is_selected()))
                    .collect();
                for (eid, is_selected) in &edge_info {
                    if *is_selected {
                        self.total_select_edges += 1;
                        let v0 = self.edge(*eid).vert;
                        self.vert_mut(v0).select = SelectState::Select;
                    }
                }
            }
            SelectMode::Face => {
                for face in &mut self.faces {
                    if face.select.is_selected() {
                        self.total_select_faces += 1;
                    }
                }
            }
        }
    }

    pub fn dissolve_verts(&mut self, vert_ids: &[HVertId]) {
        for &vid in vert_ids {
            let edges = self.vert_edges(vid);
            if edges.len() == 2 {
                let e0 = self.edge(edges[0]);
                let e1 = self.edge(edges[1]);
                let _ = (e0, e1);
            }
            self.vert_mut(vid).hide = true;
        }
    }

    pub fn dissolve_edges(&mut self, edge_ids: &[HEdgeId]) {
        for &eid in edge_ids {
            self.edge_mut(eid).hide = true;
        }
    }

    pub fn dissolve_faces(&mut self, face_ids: &[HFaceId]) {
        for &fid in face_ids {
            self.face_mut(fid).hide = true;
        }
    }

    pub fn merge_verts(&mut self, target: HVertId, source: HVertId) {
        let pos = self.vert(target).position;
        let src_pos = self.vert(source).position;
        let _ = (pos, src_pos);

        let edges = self.vert_edges(source);
        for eid in edges {
            let edge = self.edge(eid);
            if edge.vert == source {
                self.edge_mut(eid).vert = target;
            }
        }

        self.vert_mut(source).hide = true;
    }

    pub fn recalculate_face_normals(&mut self) {
        let face_ids: Vec<HFaceId> = self.faces.iter().map(|f| f.id).collect();
        for fid in face_ids {
            let vids = self.face_verts(fid);
            let normal = self.calculate_face_normal(&vids);
            self.face_mut(fid).normal = normal;
        }
    }

    pub fn fill_hole(&mut self, edge_loop: &[HEdgeId]) -> Option<HFaceId> {
        if edge_loop.len() < 3 { return None; }
        let verts: Vec<HVertId> = edge_loop.iter().map(|eid| self.edge(*eid).vert).collect();
        Some(self.add_face(&verts))
    }

    pub fn flip_faces(&mut self) {
        let face_info: Vec<(Option<HLoopId>, HFaceId, u32)> = self.faces.iter()
            .map(|f| (f.loop_start, f.id, f.loop_total))
            .collect();
        for (loop_start, fid, loop_total) in face_info {
            let vids = self.face_verts(fid);
            let mut reversed = vids;
            reversed.reverse();

            if let Some(start) = loop_start {
                let first_vert = self.loop_(start).vert;
                for i in 0..loop_total {
                    let lid = HLoopId(start.0 + i);
                    self.loop_mut(lid).vert = reversed[i as usize];
                }
                let _ = first_vert;
            }

            let normal = self.face(fid).normal;
            self.face_mut(fid).normal = -normal;
        }
    }

    pub fn extrude_face_region(&mut self, face_ids: &[HFaceId], offset: Vec3) -> Vec<HFaceId> {
        let mut new_faces = Vec::new();
        let mut vert_map: HashMap<HVertId, HVertId> = HashMap::new();

        for &fid in face_ids {
            let face = self.face(fid).clone();
            let vids = self.face_verts(fid);
            let mut new_vids = Vec::new();

            for &vid in &vids {
                if !vert_map.contains_key(&vid) {
                    let new_pos = self.vert(vid).position + offset;
                    let new_vid = self.add_vert(new_pos);
                    vert_map.insert(vid, new_vid);
                }
                new_vids.push(vert_map[&vid]);
            }

            let new_fid = self.add_face(&new_vids);
            new_faces.push(new_fid);
        }

        for &fid in face_ids {
            let vids = self.face_verts(fid);
            let mut side_vids = Vec::new();
            for &vid in &vids {
                side_vids.push(vid);
                side_vids.push(vert_map[&vid]);
            }

            for i in (0..side_vids.len()).step_by(2) {
                let v0 = side_vids[i];
                let v1 = side_vids[(i + 2) % side_vids.len()];
                let nv0 = side_vids[i + 1];
                let nv1 = side_vids[(i + 3) % side_vids.len()];
                self.add_face(&[v0, nv0, nv1, v1]);
            }
        }

        new_faces
    }

    pub fn inset_face_region(&mut self, face_ids: &[HFaceId], thickness: f32) -> Vec<HFaceId> {
        let mut new_faces = Vec::new();

        for &fid in face_ids {
            let vids = self.face_verts(fid);
            let center: Vec3 = vids.iter()
                .map(|vid| self.vert(*vid).position)
                .sum::<Vec3>() / vids.len() as f32;

            let mut inner_vids = Vec::new();
            for &vid in &vids {
                let pos = self.vert(vid).position;
                let new_pos = pos.lerp(center, thickness);
                let new_vid = self.add_vert(new_pos);
                inner_vids.push(new_vid);
            }

            let inner_fid = self.add_face(&inner_vids);
            new_faces.push(inner_fid);
        }

        new_faces
    }

    pub fn bevel_edge_region(&mut self, edge_ids: &[HEdgeId], segments: u32) {
        for &eid in edge_ids {
            let edge = self.edge(eid).clone();
            let v0_pos = self.vert(edge.vert).position;

            let next_edge = edge.next.map(|n| self.edge(n).clone());
            if let Some(next) = next_edge {
                let v1_pos = self.vert(next.vert).position;

                for seg in 1..segments {
                    let t = seg as f32 / segments as f32;
                    let pos = v0_pos.lerp(v1_pos, t);
                    let _ = self.add_vert(pos);
                }
            }
        }
    }

    pub fn loop_cut(&mut self, _edge_loop_id: HEdgeId, cuts: u32) {
        let _ = cuts;
    }

    pub fn subdivide_edges(&mut self, edge_ids: &[HEdgeId], cuts: u32) {
        for &eid in edge_ids {
            let edge = self.edge(eid).clone();
            let v0 = self.vert(edge.vert).position;
            let next = edge.next.map(|n| self.edge(n).vert);
            if let Some(next_vid) = next {
                let v1 = self.vert(next_vid).position;
                for c in 1..=cuts {
                    let t = c as f32 / (cuts + 1) as f32;
                    let pos = v0.lerp(v1, t);
                    let _ = self.add_vert(pos);
                }
            }
        }
    }

    pub fn smooth_verts(&mut self, factor: f32, iterations: u32) {
        for _ in 0..iterations {
            let original: Vec<Vec3> = self.verts.iter().map(|v| v.position).collect();
            let avgs: Vec<Vec3> = self.verts.iter().map(|vert| {
                let edges = self.vert_edges(vert.id);
                let neighbors: Vec<Vec3> = edges.iter()
                    .filter_map(|eid| {
                        let e = self.edge(*eid);
                        if e.vert == vert.id {
                            e.next.map(|n| original[self.edge(n).vert.0 as usize])
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
            for (i, vert) in self.verts.iter_mut().enumerate() {
                vert.position = vert.position.lerp(avgs[i], factor);
            }
        }
    }

    pub fn laplacian_smooth_verts(&mut self, factor: f32, iterations: u32) {
        for _ in 0..iterations {
            let original: Vec<Vec3> = self.verts.iter().map(|v| v.position).collect();
            let avgs: Vec<Vec3> = self.verts.iter().map(|vert| {
                let edges = self.vert_edges(vert.id);
                let neighbors: Vec<Vec3> = edges.iter()
                    .filter_map(|eid| {
                        let e = self.edge(*eid);
                        if e.vert == vert.id {
                            e.next.map(|n| original[self.edge(n).vert.0 as usize])
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
            for (i, vert) in self.verts.iter_mut().enumerate() {
                let laplacian: Vec3 = avgs[i] - original[i];
                vert.position += laplacian * factor;
            }
        }
    }

    pub fn triangulate(&mut self) {
        let face_ids: Vec<HFaceId> = self.faces.iter().map(|f| f.id).collect();
        for fid in face_ids {
            let vids = self.face_verts(fid);
            if vids.len() > 3 {
                self.face_mut(fid).hide = true;
                for i in 1..vids.len() - 1 {
                    self.add_face(&[vids[0], vids[i], vids[i + 1]]);
                }
            }
        }
    }

    pub fn calculate_volume(&self) -> f32 {
        let mut volume = 0.0;
        for face in &self.faces {
            if face.hide { continue; }
            let vids = self.face_verts(face.id);
            if vids.len() < 3 { continue; }
            for i in 1..vids.len() - 1 {
                let v0 = self.vert(vids[0]).position;
                let v1 = self.vert(vids[i]).position;
                let v2 = self.vert(vids[i + 1]).position;
                volume += v0.dot(v1.cross(v2)) / 6.0;
            }
        }
        volume.abs()
    }

    pub fn calculate_surface_area(&self) -> f32 {
        let mut area = 0.0;
        for face in &self.faces {
            if face.hide { continue; }
            let vids = self.face_verts(face.id);
            if vids.len() < 3 { continue; }
            for i in 1..vids.len() - 1 {
                let v0 = self.vert(vids[0]).position;
                let v1 = self.vert(vids[i]).position;
                let v2 = self.vert(vids[i + 1]).position;
                area += (v1 - v0).cross(v2 - v0).length() * 0.5;
            }
        }
        area
    }

    pub fn to_vertex_buffer(&self) -> Vec<[f32; 12]> {
        let mut buffer = Vec::new();
        for face in &self.faces {
            if face.hide { continue; }
            let vids = self.face_verts(face.id);
            if vids.len() < 3 { continue; }
            for i in 1..vids.len() - 1 {
                for &vid in &[vids[0], vids[i], vids[i + 1]] {
                    let v = self.vert(vid);
                    buffer.push([
                        v.position.x, v.position.y, v.position.z,
                        v.normal.x, v.normal.y, v.normal.z,
                        v.uv.x, v.uv.y,
                        v.color[0], v.color[1], v.color[2], v.color[3],
                    ]);
                }
            }
        }
        buffer
    }

    pub fn vertex_count(&self) -> u32 {
        self.verts.iter().filter(|v| !v.hide).count() as u32
    }

    pub fn edge_count(&self) -> u32 {
        self.edges.iter().filter(|e| !e.hide).count() as u32
    }

    pub fn face_count(&self) -> u32 {
        self.faces.iter().filter(|f| !f.hide).count() as u32
    }
}

impl Default for BMesh {
    fn default() -> Self { Self::new() }
}
