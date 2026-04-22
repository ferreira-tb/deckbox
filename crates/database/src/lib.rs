#![feature(nonpoison_mutex, sync_nonpoison)]

mod database;
pub mod error;
mod migration;
pub mod model;
mod schema;
pub mod sql_types;

pub use database::{BlockingDatabase, Database};
