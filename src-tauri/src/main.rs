#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;
mod model;
mod db;
mod pacientes;
mod consultas;

use crate::pacientes::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_pacientes, 
            get_paciente,
            create_paciente,
            update_paciente,
            delete_paciente
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
