use crate::error::CmdResult;
use crate::manager::ManagerExt;
use deckbox_database::model::deck_card::Db_DeckCard;
use deckbox_database::sql_types::num::Db_DeckId;
use tauri::AppHandle;

#[tauri::command]
#[specta::specta]
pub async fn get_deck_cards(app: AppHandle, deck_id: Db_DeckId) -> CmdResult<Vec<Db_DeckCard>> {
  app
    .database()
    .get_deck_cards(deck_id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn set_deck_cards(
  app: AppHandle,
  deck_id: Db_DeckId,
  cards: Vec<Db_DeckCard>,
) -> CmdResult<()> {
  app
    .database()
    .set_deck_cards(deck_id, &cards)
    .await
    .map_err(Into::into)
}
