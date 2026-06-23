#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
#![feature(const_trait_impl, try_blocks)]

mod bindings;
mod commands;
mod error;
mod http;
mod manager;
mod plugin;
mod settings;
mod state;
mod window;

#[cfg(debug_assertions)]
mod log;

use crate::state::Deckbox;
use error::BoxResult;
use mimalloc::MiMalloc;
use tauri::async_runtime::block_on;
use tauri::{AppHandle, Manager};

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

fn main() {
  #[cfg(debug_assertions)]
  log::setup().unwrap();

  let specta = bindings::collect();
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_persisted_scope::init())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(plugin::prevent_default())
    .plugin(plugin::single_instance())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(specta.invoke_handler())
    .run(tauri::generate_context!())
    .expect("failed to start tauri app");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  app.plugin(plugin::pinia(app)?)?;

  let deckbox = block_on(Deckbox::new(app))?;
  app.manage(deckbox);

  window::open(app)?;

  Ok(())
}
