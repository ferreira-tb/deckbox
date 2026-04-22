use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::wishlist)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Wish {
  pub card_id: Db_CardId,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::wishlist)]
pub struct NewWish {
  pub(crate) card_id: Db_CardId,
  pub(crate) created_at: Db_Zoned,
}

impl NewWish {
  pub fn new(card_id: Db_CardId) -> Self {
    NewWish { card_id, created_at: Db_Zoned::now() }
  }
}
