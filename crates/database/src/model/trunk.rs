use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::trunk)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct TrunkEntry {
  pub card_id: Db_CardId,
  pub amount: i32,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::trunk)]
pub struct NewTrunkEntry {
  pub(crate) card_id: Db_CardId,
  pub(crate) created_at: Db_Zoned,
  pub(crate) updated_at: Db_Zoned,
}

impl NewTrunkEntry {
  pub fn new(card_id: Db_CardId) -> Self {
    let now = Db_Zoned::now();
    NewTrunkEntry {
      card_id,
      created_at: now.clone(),
      updated_at: now,
    }
  }
}
