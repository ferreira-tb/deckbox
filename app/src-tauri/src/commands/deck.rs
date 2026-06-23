use crate::error::CmdResult;
use crate::manager::ManagerExt as _;
use crate::settings;
use deckbox_database::model::deck::{Db_Deck, Db_NewDeck};
use deckbox_database::sql_types::num::Db_DeckId;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::FilePath;
use tauri_plugin_pinia::ManagerExt as _;
use tokio::fs;
use tokio::sync::oneshot;

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
pub async fn export_decks(app: AppHandle) -> CmdResult<()> {
  let mut dir = app
    .pinia()
    .get::<PathBuf>(settings::STORE_ID, settings::DECK_DIR)
    .ok();

  if dir.is_none() {
    let (tx, rx) = oneshot::channel();
    app
      .dialog()
      .file()
      .set_title("Export Decks")
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
        .set(settings::STORE_ID, settings::DECK_DIR, path)?;
    }
  }

  if let Some(dir) = dir {
    let mut decks = Vec::new();
    let mut card_cache = HashMap::new();

    let database = app.database();
    for deck in database.get_decks().await? {
      let mut cards = Vec::new();
      for deck_card in database.get_deck_cards(deck.id).await? {
        let card = match card_cache.get(&deck_card.card_id) {
          Some(card) => card,
          None => {
            let card = database.get_card(deck_card.card_id).await?;
            card_cache.insert(deck_card.card_id, card);
            &card_cache[&deck_card.card_id]
          }
        };

        cards.push(json!({
          "card_id": card.card_id,
          "name": card.name,
          "main": deck_card.main,
          "extra": deck_card.extra,
          "side": deck_card.side,
        }));
      }

      decks.push(json!({
        "name": deck.name,
        "cards": cards
      }));
    }

    let path = dir.join("decks.json");
    let decks = serde_json::to_vec(&decks)?;
    fs::write(path, decks).await?;
  }

  Ok(())
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
