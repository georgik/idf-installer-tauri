#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use idf_env_core::config::get_tools_path;
use idf_env_core::antivirus::get_antivirus_name;
use cmd_lib::*;
use tauri::{command, State};
use std::path::Path;

#[derive(Debug, serde::Serialize)]
enum MyError {
  FooError,
}

fn run_command(esp_idf_path: &str) -> String {
  // let esp_idf_path = "c:/g";
  if !Path::new(esp_idf_path).exists() {
    println!("Cloning to {}", esp_idf_path);
    run_cmd! (
      git clone "https://github.com/espressif/esp-idf.git" "--recursive" "--depth" "1" "--shallow-submodules" "$esp_idf_path";
    );
  }

  run_cmd! (
    cd "$esp_idf_path";
    $esp_idf_path/install.bat;
  );

  "Ahoj".to_owned()
  // get_antivirus_name().into_iter().nth(0).unwrap().to_string()
}

#[command(async)]
fn simple_command(argument: String) -> Result<String, MyError> {
  println!("{}", argument);
  (!argument.is_empty())
  //.then(|| get_tools_path().to_string())
  .then(|| run_command(argument.as_str()))
  .ok_or(MyError::FooError)
}

fn main() {
  // println!("ESP-IDF: {}", get_tools_path());
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      simple_command
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
