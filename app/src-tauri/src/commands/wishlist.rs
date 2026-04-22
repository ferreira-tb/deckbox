use crate::error::CmdResult;
use crate::manager::ManagerExt;
use deckbox_database::model::wishlist::{Db_NewWish, Db_Wish};
use deckbox_database::sql_types::card_id::Db_CardId;
use tauri::AppHandle;

#[tauri::command]
#[specta::specta]
pub async fn create_wish(app: AppHandle, card_id: Db_CardId) -> CmdResult<u32> {
  let new = Db_NewWish::builder(card_id).build();
  app
    .database()
    .create_wish(new)
    .await?
    .try_into()
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn get_wish_by_card_id(app: AppHandle, card_id: Db_CardId) -> CmdResult<Db_Wish> {
  app
    .database()
    .get_wish_by_card_id(card_id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn get_wishlist(app: AppHandle) -> CmdResult<Vec<Db_Wish>> {
  app
    .database()
    .get_wishlist()
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn remove_wish(app: AppHandle, card_id: Db_CardId) -> CmdResult<u32> {
  app
    .database()
    .remove_wish(card_id)
    .await?
    .try_into()
    .map_err(Into::into)
}
