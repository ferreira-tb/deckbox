use crate::error::CmdResult;
use crate::manager::ManagerExt;
use deckbox_database::model::deck::{Db_Deck, Db_NewDeck};
use deckbox_database::sql_types::num::Db_DeckId;
use tauri::AppHandle;

#[tauri::command]
#[specta::specta]
pub async fn create_deck(app: AppHandle, deck: Db_NewDeck) -> CmdResult<Db_DeckId> {
  app
    .database()
    .create_deck(&deck)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn get_deck(app: AppHandle, id: Db_DeckId) -> CmdResult<Db_Deck> {
  app
    .database()
    .get_deck(id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn get_decks(app: AppHandle) -> CmdResult<Vec<Db_Deck>> {
  app
    .database()
    .get_decks()
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn remove_deck(app: AppHandle, id: Db_DeckId) -> CmdResult<()> {
  app
    .database()
    .remove_deck(id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn rename_deck(app: AppHandle, id: Db_DeckId, name: String) -> CmdResult<()> {
  app
    .database()
    .rename_deck(id, &name)
    .await
    .map_err(Into::into)
}
