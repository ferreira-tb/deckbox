use crate::manager::ManagerExt;
use anyhow::Result;
use deckbox_database::{BlockingDatabase, Database};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Clone, Debug)]
pub struct Deckbox {
  database: Database,
}

impl Deckbox {
  pub fn new(app: &AppHandle) -> Result<Self> {
    let url = database_file(app)?
      .to_str()
      .expect("failed to convert database path to string")
      .to_owned();

    let database = BlockingDatabase::new(&url)?;
    Ok(Self { database: Database::from(database) })
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
