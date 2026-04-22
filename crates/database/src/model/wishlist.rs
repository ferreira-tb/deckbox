use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::wishlist)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Db_Wish {
  pub card_id: Db_CardId,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::wishlist)]
pub struct Db_NewWish {
  pub(crate) card_id: Db_CardId,
  pub(crate) created_at: Db_Zoned,
}

impl Db_NewWish {
  pub fn new(card_id: Db_CardId) -> Self {
    Self { card_id, created_at: Db_Zoned::now() }
  }
}
