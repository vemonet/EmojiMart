// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::process::Command;
use std::{thread, time::Duration};
use tauri::ClipboardManager;

// Time waited for the paste to be done, before closing the window, in ms
const SPAWN_WAIT: u64 = 500;

// Wayland paste: use wayland_client::protocol::wl_keyboard::WlKeyboard;
// https://github.com/search?q=wl_keyboard+language%3ARust&type=code&l=Rust

fn main() {
    // ydotoold --socket-path="$HOME/.ydotool_socket" --socket-own="$(id -u):$(id -g)"
    match Command::new("ydotoold")
        .spawn()
        {
            Ok(_child) => {println!("[EmojiMart] ydotoold daemon started successfully");}
            Err(error) => {eprintln!("[EmojiMart] ydotoold daemon failed to start: {}", error);}
        }

    println!("STARTING");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![trigger_paste])
        // .setup(|app| {
        //     match app.get_cli_matches() {
        //         // `matches` here is a Struct with { args, subcommand }.
        //         // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
        //         // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
        //         Ok(matches) => {
        //             println!("{:?}", matches);
        //             if matches.args.contains_key("no-copy") {
        //                 add_to_clipboard = false
        //             }
        //         }
        //         Err(_) => {}
        //     }
        //     Ok(())
        // })

        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(false) => {
                // Close the window automatically when the user clicks out
                // Use thread sleep to avoid killing before pasting is done
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(SPAWN_WAIT));
                    event.window().close().unwrap();
                    // event.window().hide().unwrap();
                });
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
async fn trigger_paste(emoji: &str, previous: Option<&str>, keep: Option<bool>, app_handle: tauri::AppHandle) -> Result<String, ()> {
    println!("PASTE");

    #[cfg(target_os = "linux")]
    {
        let window_session = env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "wayland".to_string());
        if window_session.to_lowercase() == "x11" {
            // Paste on x11 with xdotool
            // TODO: for some reason when "xdotool key something" or "xdotool type something" is triggered from rust
            // it erases the clipboard. It does not happen when xdotool is run directly from the terminal though
            Command::new("xdotool")
                .arg("key")
                .arg("ctrl+shift+v")
                .spawn()
                .expect("xdotool paste command failed to start");
            // For some reason adding this additional paste of the emoji allows to keep the previous clipboard,
            // and paste the right emoji with xdotool
            app_handle.clipboard_manager().write_text(emoji).unwrap();

        } else {
            // Paste on wayland with ydotool
            println!("Emoji to paste: {}", emoji);
            // Type don't work with emojis https://github.com/ReimuNotMoe/ydotool/issues/22
            // ydotool key 29:1 42:1 47:1 47:0 42:0 29:0

            match Command::new("ydotool")
            .arg("key")
            .arg("29:1") // ctrl
            .arg("42:1") // shift
            .arg("47:1") // v
            .arg("47:0")
            .arg("42:0")
            .arg("29:0")
            .spawn()
            {
                Ok(_child) => {
                    // if keep.unwrap_or(false) == true && previous.is_some() {
                    //     // Put back the previous item in the clipboard
                    //     println!("TRY TO KEEP: {}", previous.unwrap().to_string());
                    //     app_handle.clipboard_manager().write_text(previous.unwrap().to_string()).unwrap();
                    // }
                }
                Err(_error) => {}
            }

            if keep.unwrap_or(false) == true && previous.is_some() {
                // Put back the previous item in the clipboard
                println!("TRY TO KEEP: {}", previous.unwrap().to_string());
                app_handle.clipboard_manager().write_text(previous.unwrap().to_string()).unwrap();
            }
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
