use crate::sql_types::num::Db_DeckId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::deck)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Db_Deck {
  pub id: Db_DeckId,
  pub name: String,
  pub description: Option<String>,
}

#[derive(Insertable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::deck)]
pub struct Db_NewDeck {
  pub(crate) name: String,

  #[serde(default)]
  pub(crate) description: Option<String>,

  #[serde(skip, default = "Db_Zoned::now")]
  pub(crate) created_at: Db_Zoned,

  #[serde(skip, default = "Db_Zoned::now")]
  pub(crate) updated_at: Db_Zoned,
}
