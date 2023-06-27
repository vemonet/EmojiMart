// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::{thread, time::Duration};

// Time waited for the paste to be done, before closing the window, in ms
const SPAWN_WAIT: u64 = 50;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![trigger_paste])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(false) => {
                // Close the window automatically when the user clicks out
                // Use thread to avoid killing before pasting is done
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
async fn trigger_paste(window: tauri::Window) {
    println!("PASTING!");
    #[cfg(target_os = "linux")]
    {
        Command::new("xdotool")
            .arg("key")
            .arg("ctrl+v")
            .spawn()
            .expect("paste command failed to start");

        // Spawning a thread to close the app after xdotool has been executed
        // Otherwise it has not the time to paste
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(SPAWN_WAIT));
            window.close().unwrap();
        });
    }

    #[cfg(not(target_os = "linux"))]
    window.close().unwrap();
}

// Enigo bug with Tauri: https://github.com/enigo-rs/enigo/issues/153
// https://github.com/tauri-apps/tauri/issues/6421
// use enigo::*;
// let mut enigo = Enigo::new();
// enigo.key_down(Key::Control);
// enigo.key_click(Key::Layout('v'));
// enigo.key_up(Key::Control);
