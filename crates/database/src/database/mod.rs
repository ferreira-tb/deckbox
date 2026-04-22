mod card;
mod deck;
mod trunk;
mod wishlist;

use crate::error::Result;
use crate::migration::run_pending_migrations;
use crate::model::card::{Db_Card, Db_NewCard};
use crate::model::trunk::{Db_NewTrunkEntry, Db_TrunkEntry};
use crate::model::wishlist::{Db_NewWish, Db_Wish};
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::trunk_entry_amount::Db_TrunkEntryAmount;
use diesel::prelude::*;
use std::fmt;
use std::sync::Arc;
use std::sync::nonpoison::{Mutex, MutexGuard};
use tokio::task::spawn_blocking;

#[must_use]
#[derive(Clone)]
pub struct BlockingDatabase(Arc<Mutex<SqliteConnection>>);

impl BlockingDatabase {
  pub fn new(url: &str) -> Result<Self> {
    let mut conn = SqliteConnection::establish(url)?;
    run_pending_migrations(&mut conn)?;
    Ok(Self(Arc::new(Mutex::new(conn))))
  }

  fn conn(&self) -> MutexGuard<'_, SqliteConnection> {
    self.0.lock()
  }
}

impl fmt::Debug for BlockingDatabase {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("BlockingDatabase")
      .finish_non_exhaustive()
  }
}

#[must_use]
#[derive(Clone, Debug)]
pub struct Database(BlockingDatabase);

impl Database {
  pub async fn new(url: &str) -> Result<Self> {
    let url = url.to_owned();
    spawn_blocking(move || BlockingDatabase::new(&url).map(Self)).await?
  }

  pub fn blocking(&self) -> BlockingDatabase {
    self.0.clone()
  }

  pub async fn with_blocking<F, T>(&self, f: F) -> Result<T>
  where
    F: FnOnce(BlockingDatabase) -> Result<T> + Send + 'static,
    T: Send + 'static,
  {
    let blocking = self.blocking();
    spawn_blocking(move || f(blocking)).await?
  }

  pub async fn create_card(&self, new_card: Db_NewCard) -> Result<usize> {
    self
      .with_blocking(move |db| db.create_card(&new_card))
      .await
  }

  pub async fn create_trunk_entry(&self, new_entry: Db_NewTrunkEntry) -> Result<usize> {
    self
      .with_blocking(move |db| db.create_trunk_entry(&new_entry))
      .await
  }

  pub async fn create_wish(&self, new_wish: Db_NewWish) -> Result<usize> {
    self
      .with_blocking(move |db| db.create_wish(&new_wish))
      .await
  }

  pub async fn decrease_trunk_entry_amount(&self, card_id: Db_CardId) -> Result<u16> {
    self
      .with_blocking(move |db| db.decrease_trunk_entry_amount(&card_id))
      .await
  }

  pub async fn get_archetypes(&self) -> Result<Vec<String>> {
    self
      .with_blocking(|db| db.get_archetypes())
      .await
  }

  pub async fn get_card_by_card_id(&self, card_id: Db_CardId) -> Result<Db_Card> {
    self
      .with_blocking(move |db| db.get_card_by_card_id(&card_id))
      .await
  }

  pub async fn get_cards(&self) -> Result<Vec<Db_Card>> {
    self.with_blocking(|db| db.get_cards()).await
  }

  pub async fn get_trunk(&self) -> Result<Vec<Db_TrunkEntry>> {
    self.with_blocking(|db| db.get_trunk()).await
  }

  pub async fn get_trunk_entry_by_card_id(&self, card_id: Db_CardId) -> Result<Db_TrunkEntry> {
    self
      .with_blocking(move |db| db.get_trunk_entry_by_card_id(&card_id))
      .await
  }

  pub async fn get_wish_by_card_id(&self, card_id: Db_CardId) -> Result<Db_Wish> {
    self
      .with_blocking(move |db| db.get_wish_by_card_id(&card_id))
      .await
  }

  pub async fn get_wishlist(&self) -> Result<Vec<Db_Wish>> {
    self
      .with_blocking(|db| db.get_wishlist())
      .await
  }

  pub async fn has_trunk_entry(&self, card_id: Db_CardId) -> Result<bool> {
    self
      .with_blocking(move |db| db.has_trunk_entry(&card_id))
      .await
  }

  pub async fn increase_trunk_entry_amount(
    &self,
    card_id: Db_CardId,
  ) -> Result<Db_TrunkEntryAmount> {
    self
      .with_blocking(move |db| db.increase_trunk_entry_amount(&card_id))
      .await
  }

  pub async fn remove_wish(&self, card_id: Db_CardId) -> Result<usize> {
    self
      .with_blocking(move |db| db.remove_wish(&card_id))
      .await
  }
}

impl From<BlockingDatabase> for Database {
  fn from(blocking: BlockingDatabase) -> Self {
    Self(blocking)
  }
}
