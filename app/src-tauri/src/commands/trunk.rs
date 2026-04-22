use crate::error::CmdResult;
use crate::manager::ManagerExt;
use deckbox_database::model::trunk::{Db_NewTrunkEntry, Db_TrunkEntry};
use deckbox_database::sql_types::card_id::Db_CardId;
use deckbox_database::sql_types::trunk_entry_amount::Db_TrunkEntryAmount;
use tauri::AppHandle;

#[tauri::command]
#[specta::specta]
pub async fn create_trunk_entry(app: AppHandle, card_id: Db_CardId) -> CmdResult<u32> {
  let new = Db_NewTrunkEntry::builder(card_id).build();
  app
    .database()
    .create_trunk_entry(new)
    .await?
    .try_into()
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn decrease_trunk_entry_amount(app: AppHandle, card_id: Db_CardId) -> CmdResult<u16> {
  app
    .database()
    .decrease_trunk_entry_amount(card_id)
    .await
    .map_err(Into::into)
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
pub async fn get_trunk_entry_by_card_id(
  app: AppHandle,
  card_id: Db_CardId,
) -> CmdResult<Db_TrunkEntry> {
  app
    .database()
    .get_trunk_entry_by_card_id(card_id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn has_trunk_entry(app: AppHandle, card_id: Db_CardId) -> CmdResult<bool> {
  app
    .database()
    .has_trunk_entry(card_id)
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
    .increase_trunk_entry_amount(card_id)
    .await
    .map_err(Into::into)
}
