// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{GlobalShortcutManager, Manager, Window};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

fn build_menu() -> SystemTrayMenu {
  SystemTrayMenu::new()
      .add_item(CustomMenuItem::new("show".to_string(), "Emoji Mart picker 🏪"))
      .add_item(CustomMenuItem::new("shortcut".to_string(), "Shortcut: Alt+E"))
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}

fn show_window(window: Window) {
  window.show().unwrap();
  window.center().unwrap();
  window.set_always_on_top(true).unwrap();
  window.set_focus().unwrap();
}

fn main() {
    let tray_menu = build_menu();

    // NOTE: keep the frontend running in the background https://tauri.app/fr/v1/guides/features/system-tray/#preventing-the-app-from-closing
    tauri::Builder::default()
      .system_tray(SystemTray::new().with_menu(tray_menu))
      .on_system_tray_event(move |app, event| match event {
          SystemTrayEvent::RightClick { position: _, size: _, .. } => {
              show_window(app.get_window("main").unwrap());
          }
          SystemTrayEvent::DoubleClick { position: _, size: _, .. } => {
              show_window(app.get_window("main").unwrap());
          }
          SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
              "quit" => {
                  std::process::exit(0);
              }
              "show" => {
                  show_window(app.get_window("main").unwrap());
              }
              "shortcut" => {
                  // TODO: implement a window to enable changing the shortcut
                  show_window(app.get_window("main").unwrap());
              }
              _ => {}
          },
          _ => {}
      })
      .setup(|app| {
        // Don't show on the taskbar/springboard, this is purely a personal taste thing
        // #[cfg(target_os = "macos")]
        // app.set_activation_policy(tauri::ActivationPolicy::Accessory);

        let window = app.get_window("main").unwrap();

        // Register shortcut to open the app running in the background
        let mut shortcut = app.global_shortcut_manager();
        shortcut
            // NOTE: using the super key also write down the letter we hit
            .register("Alt+E", move || {
                if window.is_visible().unwrap() {
                  window.hide().unwrap();
                } else {
                  show_window(window.to_owned());
                }
            })
            .unwrap_or_else(|err| println!("{:?}", err));

        let window = app.get_window("main").unwrap();
        show_window(window);
        Ok(())
      })
      .on_window_event(|event| match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            // don't kill the app when the user clicks close. this is important
            event.window().hide().unwrap();
            api.prevent_close();
        }
        tauri::WindowEvent::Focused(false) => {
            // Hide the window automaticall when the user clicks out
            event.window().hide().unwrap();
        }
        _ => {}
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");

}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }