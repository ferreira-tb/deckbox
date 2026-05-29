use crate::error::CmdResult;
use crate::manager::ManagerExt;
use deckbox_database::model::deck::{Db_Deck, Db_NewDeck};
use deckbox_database::sql_types::num::Db_DeckId;
use tauri::AppHandle;

#[tauri::command]
#[specta::specta]
pub async fn create_deck(
  app: AppHandle,
  name: String,
  description: Option<String>,
) -> CmdResult<Db_DeckId> {
  let new = Db_NewDeck::builder(name)
    .maybe_description(description)
    .build();

  app
    .database()
    .create_deck(&new)
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
