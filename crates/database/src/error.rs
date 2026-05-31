use serde::Serialize;
use serde::ser::Serializer;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::io;
use std::result::Result as StdResult;
use tokio::task::JoinError;

pub use diesel::result::Error as DieselError;

pub type Result<T, E = Error> = StdResult<T, E>;
pub type AnyResult<T> = anyhow::Result<T>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Migration failed: {0}")]
  MigrationFailed(Box<dyn StdError + Send + Sync>),

  #[error(transparent)]
  Diesel(#[from] diesel::result::Error),
  #[error(transparent)]
  DieselConnection(#[from] diesel::ConnectionError),
  #[error(transparent)]
  Io(#[from] io::Error),
  #[error(transparent)]
  Jiff(#[from] jiff::Error),
  #[error(transparent)]
  Unknown(#[from] anyhow::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

impl<E> From<Result<Infallible, E>> for Error
where
  E: Into<Error>,
{
  fn from(value: Result<Infallible, E>) -> Self {
    value.unwrap_err().into()
  }
}

impl From<JoinError> for Error {
  fn from(err: JoinError) -> Self {
    Self::Io(io::Error::from(err))
  }
}
