use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::trunk_entry_amount::Db_TrunkEntryAmount;
use crate::sql_types::zoned::Db_Zoned;
use bon::Builder;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::trunk)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Db_TrunkEntry {
  pub card_id: Db_CardId,
  pub amount: Db_TrunkEntryAmount,
}

#[derive(Builder, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::trunk)]
pub struct Db_NewTrunkEntry {
  #[builder(start_fn)]
  pub(crate) card_id: Db_CardId,

  #[builder(skip = Db_Zoned::now())]
  pub(crate) created_at: Db_Zoned,

  #[builder(skip = Db_Zoned::now())]
  pub(crate) updated_at: Db_Zoned,
}
