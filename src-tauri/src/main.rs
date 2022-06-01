#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
fn toggle_popout(window: tauri::Window) {
  println!("toggling");
  if let Some(popout_window) = window.get_window("popout") {
    println!("found window");
    match popout_window.is_visible() {
      Ok(visible) => {
        if visible {
          match popout_window.hide() {
            Ok(()) => {
              println!("hidden");
            },
            Err(err) => {
              println!("error hiding: {}", err);
            }
          }
        } else {
          match popout_window.show() {
            Ok(()) => {
              println!("shown");
            },
            Err(err) => {
              println!("error showing: {}", err);
            }
          }
        }
      },
      Err(err) => {
        println!("error: {}", err);
      }
    }
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      toggle_popout,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
