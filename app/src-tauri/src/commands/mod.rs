pub mod card;
pub mod deck;
pub mod trunk;
pub mod wishlist;

use crate::error::CmdResult;
use crate::settings::SETTINGS_BACKUP_DIR;
use crate::state::database_file;
use deckbox_database::sql_types::card_id::Db_CardId;
use jiff::Zoned;
use std::path::PathBuf;
use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::FilePath;
use tauri_plugin_pinia::ManagerExt as _;
use tokio::fs;
use tokio::sync::oneshot;
use url::Url;

#[tauri::command]
#[specta::specta]
pub async fn export_database_file(app: AppHandle) -> CmdResult<()> {
  let mut dir = app
    .pinia()
    .get::<PathBuf>("settings", SETTINGS_BACKUP_DIR)
    .ok();

  if dir.is_none() {
    let (tx, rx) = oneshot::channel();
    app
      .dialog()
      .file()
      .set_title("Export Deckbox Database")
      .pick_folder(move |response| {
        let _ = tx.send(response);
      });

    if let Some(path) = rx
      .await?
      .map(FilePath::into_path)
      .transpose()?
      .and_then(|it| it.to_str().map(ToOwned::to_owned))
    {
      dir = Some(PathBuf::from(path.as_str()));
      app
        .pinia()
        .set("settings", SETTINGS_BACKUP_DIR, path)?;
    }
  }

  if let Some(dir) = dir {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let now = Zoned::now().strftime("%Y%m%d%H%M%S");
    let name = format!("deckbox-{VERSION}.{now}.db");
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
pub async fn open_store_website(app: AppHandle, card_id: Db_CardId) -> CmdResult<()> {
  let mut card = card::get_card_by_card_id(app, card_id).await?;
  card.name = card
    .name
    .chars()
    .filter(|it| !matches!(it, '<' | '>'))
    .collect();

  let mut url = Url::parse("https://www.ligayugioh.com.br")?;
  url
    .query_pairs_mut()
    .append_pair("view", "cards/search")
    .append_pair("card", &card.name)
    .append_pair("orderBy", "3");

  open::that_detached(url.as_str())?;

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn show_window(window: WebviewWindow) -> CmdResult<()> {
  window.show()?;
  window.set_focus()?;
  Ok(())
}
