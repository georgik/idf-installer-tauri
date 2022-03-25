#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use idf_env_core::config::get_tools_path;
// use idf_env_core::antivirus::get_antivirus_name;
use cmd_lib::*;
use tauri::{command, State};
use std::path::Path;

#[derive(Debug, serde::Serialize)]
enum MyError {
  FooError,
}

fn deploy_esp_idf(esp_idf_base_path: &str, branch: &str) -> String {
  // let esp_idf_path = "c:/g";
  let esp_idf_path_string = format!("{}/frameworks/esp-idf-{}", esp_idf_base_path, branch);
  let esp_idf_path = Path::new(esp_idf_path_string.as_str());
  let parent_path = esp_idf_path.parent().unwrap();
  if !parent_path.exists() {
    std::fs::create_dir_all(parent_path);
  }

  println!("Checking existence of {}", esp_idf_path_string);
  if !esp_idf_path.exists() {
    println!("Cloning to {}", esp_idf_path_string);
    run_cmd! (
      git clone "https://github.com/espressif/esp-idf.git" "--branch" "$branch" "--recursive" "--depth" "1" "--shallow-submodules" "$esp_idf_path_string";
    );
  }

  #[cfg(windows)]
  run_cmd! (
    cd "$esp_idf_path";
    $esp_idf_path/install.bat;
  );

  #[cfg(unix)]
  run_cmd! (
    cd "$esp_idf_path";
    $esp_idf_path/install.sh;
  );
  "Done".to_string()
}

fn deploy_esp_idf_branches(esp_idf_base_path: &str, branches: Vec<String>) -> String {
  for branch in branches.iter() {
    println!("Processing branch {}", branch);
    deploy_esp_idf(esp_idf_base_path, branch.as_str());
  }
  "Done".to_string()
}

#[command(async)]
fn simple_command(argument: String, branch: String) -> Result<String, MyError> {
  println!("{}, {}", argument, branch);
  (!argument.is_empty())
  //.then(|| get_tools_path().to_string())
  .then(|| deploy_esp_idf(argument.as_str(), branch.as_str()))
  .ok_or(MyError::FooError)
}

#[command(async)]
fn deploy_esp_idf_branches_command(base: String, branches: Vec<String>) -> Result<String, MyError> {
  println!("{}", base);
  (!base.is_empty())
  //.then(|| get_tools_path().to_string())
  .then(|| deploy_esp_idf_branches(base.as_str(), branches))
  .ok_or(MyError::FooError)
}

fn main() {
  // println!("ESP-IDF: {}", get_tools_path());
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      simple_command,
      deploy_esp_idf_branches_command
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
