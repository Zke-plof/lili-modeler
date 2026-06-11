use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PanelState {
    pub name: String,
    pub visible: bool,
    pub collapsed: bool,
}

pub struct UiState {
    pub panels: Vec<PanelState>,
    pub active_tool: String,
    pub show_grid: bool,
    pub show_axes: bool,
    pub show_stats: bool,
    pub snap_enabled: bool,
    pub proportional_enabled: bool,
    pub proportional_size: f32,
    pub cursor_position: [f32; 3],
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            panels: vec![
                PanelState { name: "Properties".into(), visible: true, collapsed: false },
                PanelState { name: "Outliner".into(), visible: true, collapsed: false },
                PanelState { name: "Timeline".into(), visible: true, collapsed: false },
                PanelState { name: "Info".into(), visible: true, collapsed: false },
            ],
            active_tool: "select".into(),
            show_grid: true,
            show_axes: true,
            show_stats: false,
            snap_enabled: false,
            proportional_enabled: false,
            proportional_size: 1.0,
            cursor_position: [0.0, 0.0, 0.0],
        }
    }
}
