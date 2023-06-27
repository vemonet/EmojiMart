// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{GlobalShortcutManager, Manager, Window, AppHandle};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
// use dbus::{blocking::Connection, arg};
// use std::time::Duration;

const BUILTIN_SHORTCUT: &str = "Alt+Space";
// NOTE: using the super key + E also write down the E letter. CommandOrControl is possible

fn build_menu() -> SystemTrayMenu {
  SystemTrayMenu::new()
      .add_item(CustomMenuItem::new("show".to_string(), "Emoji Mart picker 🏪"))
      .add_item(CustomMenuItem::new("shortcut".to_string(), format!("Shortcut: {}", BUILTIN_SHORTCUT)))
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}

fn show_window(window: &Window) {
  window.show().unwrap();
  window.center().unwrap();
  window.set_always_on_top(true).unwrap();
  window.set_focus().unwrap();
}

// TODO: use dbus for autostart https://github.com/diwic/dbus-rs/blob/master/dbus/examples/properties.rs
// https://github.com/rustdesk/rustdesk/blob/56eac7294c706ffbe3bf9043b5f1d9b1bc2c4f5a/libs/scrap/src/wayland/pipewire.rs#L304
// https://github.com/AlfioEmanueleFresta/xdg-credentials-portal/blob/eb87ea691a7f97b6ffd9d75f673ff54e13794e82/libwebauthn/src/ui.rs#L92
// fn linux_autostart() {
// let proxy: Proxy<&'conn Connection> = conn.with_proxy(
//   "org.freedesktop.portal.Desktop",  // iface
//   "/org/freedesktop/portal/desktop", // object
//   Duration::from_millis(5000),
// );
// let mut options = HashMap::new();
// options.insert("reason", "Emoji Mart autostart");
// options.insert("autostart", true);
// options.insert("commandline", "emoji-mart");
// proxy.method_call(
//   "org.freedesktop.portal.Background",
//   "RequestBackground",
//   ("".to_string(), options),
// )?;

// fn main_window(app: &AppHandle) -> &Window {
//   return &app.get_window("main").unwrap();
// }

fn main() {
    let tray_menu = build_menu();

    // #[cfg(target_os = "linux")]
    // linux_autostart();

    // NOTE: keep the frontend running in the background https://tauri.app/fr/v1/guides/features/system-tray/#preventing-the-app-from-closing
    tauri::Builder::default()
      .system_tray(SystemTray::new().with_menu(tray_menu))
      .on_system_tray_event(move |app, event| match event {
          // NOTE: right and double click don't seems to work
          SystemTrayEvent::RightClick { position: _, size: _, .. } => {
              show_window(&app.get_window("main").unwrap());
          }
          SystemTrayEvent::DoubleClick { position: _, size: _, .. } => {
              show_window(&app.get_window("main").unwrap());
          }
          SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
              "quit" => {
                  std::process::exit(0);
              }
              "show" => {
                  show_window(&app.get_window("main").unwrap());
              }
              "shortcut" => {
                  // TODO: implement a window to enable the user to change the shortcut
                  show_window(&app.get_window("main").unwrap());
              }
              _ => {}
          },
          _ => {}
      })
      .setup(|app| {
        // Don't show on the taskbar/springboard, this is purely a personal taste thing
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);

        let window = app.get_window("main").unwrap();
        // let window = main_window(app.app_handle());

        // Register shortcut to open the app running in the background
        let mut shortcut = app.global_shortcut_manager();
        shortcut
            .register(BUILTIN_SHORTCUT, move || {
                if window.is_visible().unwrap() {
                  window.hide().unwrap();
                } else {
                  // TODO: should not be owned
                  // show_window(window);
                  window.show().unwrap();
                  window.center().unwrap();
                  window.set_always_on_top(true).unwrap();
                  window.set_focus().unwrap();
                }
            })
            .unwrap_or_else(|err| println!("{:?}", err));

        let window = &app.get_window("main").unwrap();
        // show_window(window);
        window.show().unwrap();
        window.center().unwrap();
        window.set_always_on_top(true).unwrap();
        window.set_focus().unwrap();
        Ok(())
      })
      .on_window_event(|event| match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            // Don't kill the app when the user clicks close
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
