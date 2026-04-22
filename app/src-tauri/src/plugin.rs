use crate::error::BoxResult;
use crate::manager::ManagerExt;
use crate::window::WindowExt;
use tap::Pipe;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Wry};
use tauri_plugin_pinia::PrettyTomlMarshaler;

pub fn pinia(app: &AppHandle) -> BoxResult<TauriPlugin<Wry>> {
  tauri_plugin_pinia::Builder::new()
    .path(app.deckbox_dir()?)
    .marshaler(Box::new(PrettyTomlMarshaler))
    .build()
    .pipe(Ok)
}

pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::{Builder, Flags, PlatformOptions};
  Builder::new()
    .with_flags(Flags::debug())
    .platform(
      PlatformOptions::new()
        .browser_accelerator_keys(cfg!(debug_assertions))
        .default_context_menus(cfg!(debug_assertions))
        .default_script_dialogs(cfg!(debug_assertions))
        .general_autofill(false)
        .password_autosave(false)
        .pinch_zoom(false)
        .swipe_navigation(false)
        .zoom_control(false),
    )
    .build()
}

pub fn single_instance() -> TauriPlugin<Wry> {
  tauri_plugin_single_instance::init(|app, _, _| {
    let window = app.main_window();
    let _ = try {
      window.show()?;
      window.unminimize()?;
      window.set_focus()?;
    };
  })
}
