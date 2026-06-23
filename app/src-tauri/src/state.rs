use crate::manager::ManagerExt;
use anyhow::Result;
use deckbox_database::Database;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Clone)]
pub struct Deckbox {
  database: Database,
}

impl Deckbox {
  pub async fn new(app: &AppHandle) -> Result<Self> {
    let url = database_file(app)?
      .to_str()
      .expect("failed to convert database path to string")
      .to_owned();

    Ok(Self { database: Database::new(&url).await? })
  }

  pub fn database(&self) -> Database {
    self.database.clone()
  }
}

pub fn database_file(app: &AppHandle) -> Result<PathBuf> {
  let dir = app.deckbox_dir()?;
  fs::create_dir_all(&dir)?;

  #[cfg(debug_assertions)]
  let file_name = "deckbox.dev.db";
  #[cfg(not(debug_assertions))]
  let file_name = "deckbox.db";

  Ok(dir.join(file_name))
}
