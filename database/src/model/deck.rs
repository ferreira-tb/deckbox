use crate::sql_types::num::Db_DeckId;
use crate::sql_types::zoned::Db_Zoned;
use bon::Builder;
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

#[derive(Builder, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::deck)]
pub struct Db_NewDeck {
  #[builder(start_fn, into)]
  pub(crate) name: String,

  #[builder(into)]
  pub(crate) description: Option<String>,

  #[builder(skip = Db_Zoned::now())]
  pub(crate) created_at: Db_Zoned,

  #[builder(skip = Db_Zoned::now())]
  pub(crate) updated_at: Db_Zoned,
}
