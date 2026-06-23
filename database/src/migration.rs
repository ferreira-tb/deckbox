use crate::Conn;
use crate::error::{Error, Result};
use diesel_async::AsyncMigrationHarness;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub(super) fn run_pending_migrations(conn: Conn) -> Result<Conn> {
  let mut harness = AsyncMigrationHarness::new(conn);
  harness
    .run_pending_migrations(MIGRATIONS)
    .map_err(Error::MigrationFailed)?;

  Ok(harness.into_inner())
}
