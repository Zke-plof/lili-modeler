#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod mesh;
mod render;
mod scene;
mod ui;
mod io;

use engine::Engine;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = Engine::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .manage(parking_lot::RwLock::new(engine))
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            engine::commands::create_primitive,
            engine::commands::delete_object,
            engine::commands::transform_object,
            engine::commands::set_selection_mode,
            engine::commands::extrude_selection,
            engine::commands::inset_faces,
            engine::commands::bevel_edges,
            engine::commands::loop_cut,
            engine::commands::boolean_operation,
            engine::commands::import_mesh,
            engine::commands::export_mesh,
            engine::commands::get_scene_data,
            engine::commands::set_camera,
            engine::commands::add_constraint,
            engine::commands::measure_distance,
            engine::commands::measure_angle,
            engine::commands::get_mesh_info,
            engine::commands::get_mesh_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}