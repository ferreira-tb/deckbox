use serde::Serialize;
use specta::Type;
use std::fmt::Display;

pub use std::error::Error as StdError;
pub use std::result::Result as StdResult;

pub type CmdResult<T> = StdResult<T, Error>;
pub type BoxResult<T> = StdResult<T, Box<dyn StdError>>;

#[derive(Debug, Serialize, Type)]
pub struct Error(String);

impl<T: Display> From<T> for Error {
  fn from(value: T) -> Self {
    Self(value.to_string())
  }
}
