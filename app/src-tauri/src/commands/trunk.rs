use crate::error::CmdResult;
use crate::manager::ManagerExt as _;
use crate::settings;
use deckbox_database::model::card::Db_Card;
use deckbox_database::model::trunk::{Db_NewTrunkEntry, Db_TrunkEntry};
use deckbox_database::sql_types::card_id::Db_CardId;
use deckbox_database::sql_types::num::{Db_TrunkEntryAmount, Db_TrunkEntryId};
use itertools::Itertools;
use serde_json::json;
use std::path::PathBuf;
use tap::Pipe;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::FilePath;
use tauri_plugin_pinia::ManagerExt as _;
use tokio::fs;
use tokio::sync::oneshot;

#[tauri::command]
#[specta::specta]
pub async fn create_trunk_entry(app: AppHandle, card_id: Db_CardId) -> CmdResult<Db_TrunkEntryId> {
  let new = Db_NewTrunkEntry::builder(card_id).build();
  app
    .database()
    .create_trunk_entry(&new)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn decrease_trunk_entry_amount(
  app: AppHandle,
  card_id: Db_CardId,
) -> CmdResult<Db_TrunkEntryAmount> {
  app
    .database()
    .decrease_trunk_entry_amount(&card_id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn export_trunk(app: AppHandle) -> CmdResult<()> {
  let mut dir = app
    .pinia()
    .get::<PathBuf>(settings::STORE_ID, settings::TRUNK_DIR)
    .ok();

  if dir.is_none() {
    let (tx, rx) = oneshot::channel();
    app
      .dialog()
      .file()
      .set_title("Export Trunk")
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
        .set(settings::STORE_ID, settings::TRUNK_DIR, path)?;
    }
  }

  if let Some(dir) = dir {
    let to_json = |(card, amount): (Db_Card, Db_TrunkEntryAmount)| {
      json!({
        "card_id": card.card_id,
        "name": card.name,
        "name_pt": card.name_pt,
        "description": card.description,
        "archetype": card.archetype,
        "ygoprodeck_url": card.ygoprodeck_url,
        "amount": amount,
        "banlist_status": card.banlist_status
      })
    };

    let cards = app
      .database()
      .get_trunk_cards()
      .await?
      .into_iter()
      .map(to_json)
      .collect_vec()
      .pipe_ref(serde_json::to_vec)?;

    let path = dir.join("trunk.json");
    fs::write(path, cards).await?;
  }

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_trunk(app: AppHandle) -> CmdResult<Vec<Db_TrunkEntry>> {
  app
    .database()
    .get_trunk()
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn get_trunk_cards(app: AppHandle) -> CmdResult<Vec<(Db_Card, Db_TrunkEntryAmount)>> {
  app
    .database()
    .get_trunk_cards()
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn get_trunk_entry_by_card_id(
  app: AppHandle,
  card_id: Db_CardId,
) -> CmdResult<Db_TrunkEntry> {
  app
    .database()
    .get_trunk_entry_by_card_id(&card_id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn has_trunk_entry(app: AppHandle, card_id: Db_CardId) -> CmdResult<bool> {
  app
    .database()
    .has_trunk_entry(&card_id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn increase_trunk_entry_amount(
  app: AppHandle,
  card_id: Db_CardId,
) -> CmdResult<Db_TrunkEntryAmount> {
  app
    .database()
    .increase_trunk_entry_amount(&card_id)
    .await
    .map_err(Into::into)
}
