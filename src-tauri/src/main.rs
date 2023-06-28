// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::process::Command;
use std::{thread, time::Duration};
use tauri::ClipboardManager;

// Time waited for the paste to be done, before closing the window, in ms
const SPAWN_WAIT: u64 = 50;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![trigger_paste])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(false) => {
                // Close the window automatically when the user clicks out
                // Use thread sleep to avoid killing before pasting is done
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(SPAWN_WAIT));
                    event.window().close().unwrap();
                });
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn trigger_paste(emoji: &str, app_handle: tauri::AppHandle) -> Result<String, ()> {
    // TODO: for some reason when "xdotool key something" or "xdotool type something" is triggered from rust
    // it erases the clipboard. It does not happen when xdotool is run directly from the terminal though
    #[cfg(target_os = "linux")]
    {
        // Only paste on x11
        let window_session = env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "wayland".to_string());
        if window_session.to_lowercase() == "x11" {
            Command::new("xdotool")
                .arg("key")
                .arg("ctrl+shift+v")
                .spawn()
                .expect("paste command failed to start");
            // For some reason adding this additional paste of the emoji allows to keep the previous clipboard
            app_handle.clipboard_manager().write_text(emoji).unwrap();
        }
    }

    // #[cfg(not(target_os = "linux"))]
    // window.close().unwrap();
    Ok(emoji.to_string())
}

// Enigo bug with Tauri: https://github.com/enigo-rs/enigo/issues/153
// https://github.com/tauri-apps/tauri/issues/6421
// use enigo::*;
// let mut enigo = Enigo::new();
// enigo.key_down(Key::Control);
// enigo.key_click(Key::Layout('v'));
// enigo.key_up(Key::Control);
