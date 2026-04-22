use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::deck)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Deck {
  pub id: i32,
  pub name: String,
  pub description: Option<String>,
}

#[derive(Insertable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::deck)]
#[serde(rename_all = "camelCase")]
pub struct NewDeck {
  name: String,
  description: Option<String>,
  created_at: Db_Zoned,
  updated_at: Db_Zoned,
}
