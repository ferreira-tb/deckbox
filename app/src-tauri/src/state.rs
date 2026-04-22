use crate::manager::ManagerExt;
use anyhow::{Result, anyhow};
use deckbox_database::{BlockingDatabase, Database};
use std::fs;
use tauri::AppHandle;

#[derive(Clone, Debug)]
pub struct Deckbox {
  database: Database,
}

impl Deckbox {
  pub fn new(app: &AppHandle) -> Result<Self> {
    let url = database_url(app)?;
    let database = BlockingDatabase::new(&url)?;
    Ok(Self { database: Database::from(database) })
  }

  pub fn database(&self) -> Database {
    self.database.clone()
  }
}

fn database_url(app: &AppHandle) -> Result<String> {
  let dir = app.deckbox_dir()?;
  fs::create_dir_all(&dir)?;

  #[cfg(debug_assertions)]
  let file_name = "deckbox.dev.db";
  #[cfg(not(debug_assertions))]
  let file_name = "deckbox.db";

  dir
    .join(file_name)
    .to_str()
    .map(ToOwned::to_owned)
    .ok_or_else(|| anyhow!("failed to convert database path to string"))
}
