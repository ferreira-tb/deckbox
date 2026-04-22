use crate::error::CmdResult;
use crate::http::get_bytes;
use crate::manager::ManagerExt;
use deckbox_database::model::card::{Db_Card, Db_NewCard};
use deckbox_database::sql_types::card_id::Db_CardId;
use deckbox_database::sql_types::url::Db_Url;
use futures::try_join;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use tap::TapFallible;
use tauri::AppHandle;
use tokio::fs;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::sleep;

#[tauri::command]
#[specta::specta]
pub async fn fetch_cards(app: AppHandle) -> CmdResult<()> {
  let database = app.database();
  let img_dir = app.img_dir()?;
  let img_dir_cropped = app.img_dir_cropped()?;
  let img_dir_small = app.img_dir_small()?;

  fs::create_dir_all(&img_dir).await?;
  fs::create_dir_all(&img_dir_cropped).await?;
  fs::create_dir_all(&img_dir_small).await?;

  let img_dir: Arc<Path> = Arc::from(img_dir);
  let img_dir_cropped: Arc<Path> = Arc::from(img_dir_cropped);
  let img_dir_small: Arc<Path> = Arc::from(img_dir_small);

  let mut set = JoinSet::new();
  let semaphore = Arc::new(Semaphore::new(5));

  for card in ygo::all()
    .await?
    .into_iter()
    .filter_map(Db_NewCard::from_ygo_card)
  {
    database.create_card(card.clone()).await?;
    set.spawn({
      let semaphore = Arc::clone(&semaphore);
      let img_dir = Arc::clone(&img_dir);
      let img_dir_cropped = Arc::clone(&img_dir_cropped);
      let img_dir_small = Arc::clone(&img_dir_small);

      async move {
        let file_name = format!("{}.jpg", card.card_id());
        let path = img_dir.join(&file_name);
        let path_cropped = img_dir_cropped.join(&file_name);
        let path_small = img_dir_small.join(&file_name);

        let permit = semaphore.acquire_owned().await?;
        let sent_request = AtomicBool::new(false);

        let download = async |url: &Db_Url, path: &Path| {
          if !fs::try_exists(&path).await? {
            sent_request.store(true, Relaxed);
            match get_bytes(url.as_str()).await {
              Ok(bytes) => {
                fs::write(path, bytes).await?;

                #[cfg(debug_assertions)]
                tracing::info!("image saved to {}", path.display());
              }
              #[cfg(debug_assertions)]
              Err(error) => tracing::warn!(%error),
              #[cfg(not(debug_assertions))]
              Err(_) => {}
            }
          }

          anyhow::Ok(())
        };

        try_join!(
          download(card.image_url(), &path),
          download(card.image_url_cropped(), &path_cropped),
          download(card.image_url_small(), &path_small)
        )?;

        if sent_request.load(Relaxed) {
          sleep(Duration::from_secs(1)).await;
        }

        drop(permit);

        anyhow::Ok(())
      }
    });
  }

  while let Some(result) = set.join_next().await {
    let _ = result?.tap_err_dbg(|error| tracing::warn!(%error));
  }

  Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_archetypes(app: AppHandle) -> CmdResult<Vec<String>> {
  app
    .database()
    .get_archetypes()
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn get_card_by_card_id(app: AppHandle, card_id: Db_CardId) -> CmdResult<Db_Card> {
  app
    .database()
    .get_card_by_card_id(card_id)
    .await
    .map_err(Into::into)
}

#[tauri::command]
#[specta::specta]
pub async fn get_cards(app: AppHandle) -> CmdResult<Vec<Db_Card>> {
  app
    .database()
    .get_cards()
    .await
    .map_err(Into::into)
}
