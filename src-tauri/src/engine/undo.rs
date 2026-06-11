use std::collections::VecDeque;
use crate::mesh::Mesh;

pub struct UndoSystem {
    history: VecDeque<UndoEntry>,
    redo_stack: VecDeque<UndoEntry>,
    max_steps: usize,
}

#[derive(Clone)]
pub struct UndoEntry {
    pub name: String,
    pub snapshot: Vec<u8>,
    pub object_id: String,
    pub timestamp: u64,
}

impl UndoSystem {
    pub fn new(max_steps: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_steps),
            redo_stack: VecDeque::new(),
            max_steps,
        }
    }

    pub fn push(&mut self, name: String, object_id: String, mesh: &Mesh) {
        let snapshot = bincode::serialize(mesh).unwrap_or_default();
        let entry = UndoEntry {
            name,
            snapshot,
            object_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };

        self.history.push_back(entry);
        self.redo_stack.clear();

        while self.history.len() > self.max_steps {
            self.history.pop_front();
        }
    }

    pub fn undo(&mut self) -> Option<(String, Mesh)> {
        let entry = self.history.pop_back()?;
        let mesh: Mesh = bincode::deserialize(&entry.snapshot).unwrap_or_default();
        Some((entry.object_id, mesh))
    }

    pub fn redo(&mut self) -> Option<(String, Mesh)> {
        let entry = self.redo_stack.pop_back()?;
        let mesh: Mesh = bincode::deserialize(&entry.snapshot).unwrap_or_default();
        Some((entry.object_id, mesh))
    }

    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.redo_stack.clear();
    }

    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    pub fn get_history_names(&self) -> Vec<String> {
        self.history.iter().map(|e| e.name.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undo_redo() {
        let mut undo = UndoSystem::new(10);
        let mesh = Mesh::new();
        undo.push("Create".into(), "obj1".into(), &mesh);
        assert!(undo.can_undo());
        assert!(!undo.can_redo());

        let result = undo.undo();
        assert!(result.is_some());
        assert!(!undo.can_undo());
        assert!(undo.can_redo());
    }
}