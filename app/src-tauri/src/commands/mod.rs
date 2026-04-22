pub mod card;
pub mod deck;
pub mod trunk;
pub mod wishlist;

use crate::error::CmdResult;
use crate::state::database_file;
use jiff::Zoned;
use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::FilePath;
use tokio::fs;
use tokio::sync::oneshot;

#[tauri::command]
#[specta::specta]
pub async fn export_database_file(app: AppHandle) -> CmdResult<()> {
  let (tx, rx) = oneshot::channel();
  app
    .dialog()
    .file()
    .set_title("Export Deckbox Database")
    .pick_folder(move |response| {
      let _ = tx.send(response);
    });

  if let Some(dir) = rx
    .await?
    .map(FilePath::into_path)
    .transpose()?
  {
    let version = env!("CARGO_PKG_VERSION");
    let now = Zoned::now().strftime("%Y%m%d%H%M%S");
    let name = format!("deckbox-{version}.{now}.db");
    let path = dir.join(name);

    let database = database_file(&app)?;
    fs::copy(database, &path).await?;

    #[cfg(debug_assertions)]
    tracing::info!("Exported database to {}", path.display());
  }

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn show_window(window: WebviewWindow) -> CmdResult<()> {
  window.show()?;
  window.set_focus()?;
  Ok(())
}
