use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::num::Db_WishId;
use crate::sql_types::zoned::Db_Zoned;
use bon::Builder;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::wishlist)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Db_Wish {
  pub id: Db_WishId,
  pub card_id: Db_CardId,
}

#[derive(Builder, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::wishlist)]
pub struct Db_NewWish {
  #[builder(start_fn)]
  pub(crate) card_id: Db_CardId,

  #[builder(skip = Db_Zoned::now())]
  pub(crate) created_at: Db_Zoned,
}
