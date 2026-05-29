mod card;
mod deck;
mod trunk;
mod wishlist;

use crate::error::Result;
use crate::migration::run_pending_migrations;
use diesel::prelude::*;
use diesel_async::AsyncConnection;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use std::fmt;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) type Conn = SyncConnectionWrapper<SqliteConnection>;

#[must_use]
#[derive(Clone)]
pub struct Database(Arc<Mutex<Conn>>);

impl Database {
  pub async fn new(url: &str) -> Result<Self> {
    let conn = Conn::establish(url).await?;
    let conn = run_pending_migrations(conn)?;
    Ok(Self(Arc::new(Mutex::new(conn))))
  }
}

impl fmt::Debug for Database {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("Database")
      .finish_non_exhaustive()
  }
}
