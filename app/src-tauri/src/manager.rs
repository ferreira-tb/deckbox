use crate::state::Deckbox;
use anyhow::Result;
use deckbox_database::Database;
use std::env;
use std::path::PathBuf;
use tauri::{Manager, State, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn deckbox(&self) -> State<'_, Deckbox> {
    self.app_handle().state::<Deckbox>()
  }

  fn database(&self) -> Database {
    self.deckbox().database()
  }

  fn deckbox_dir(&self) -> Result<PathBuf> {
    if let Some(home) = env::home_dir() {
      Ok(home.join(".tsukilabs/deckbox"))
    } else {
      self
        .path()
        .app_local_data_dir()
        .map_err(Into::into)
    }
  }

  fn img_dir(&self) -> Result<PathBuf> {
    Ok(self.deckbox_dir()?.join("images"))
  }

  fn img_dir_cropped(&self) -> Result<PathBuf> {
    Ok(self.img_dir()?.join("cropped"))
  }

  fn img_dir_small(&self) -> Result<PathBuf> {
    Ok(self.img_dir()?.join("small"))
  }
}

impl<T: Manager<Wry>> ManagerExt for T {}
