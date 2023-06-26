// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    // NOTE: try to keep the frontend and backend running in the background,
    // but it seems like a completely new app is started when AppImage is called
    // https://tauri.app/fr/v1/guides/features/system-tray/#preventing-the-app-from-closing
    tauri::Builder::default().on_window_event(|event| match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
          event.window().hide().unwrap();
          api.prevent_close();
        }
        _ => {}
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");

    // Backend in background:
    // tauri::Builder::default()
    //     .build(tauri::generate_context!())
    //     .expect("error while running tauri application")
    //     .run(|_app_handle, event| match event {
    //         tauri::RunEvent::ExitRequested { api, .. } => {
    //           api.prevent_exit();
    //         }
    //         _ => {}
    //     });

    // No background
    // tauri::Builder::default()
    //     // .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application")
}
