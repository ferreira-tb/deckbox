use derive_more::{Display, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(FromSqlRow, AsExpression, Clone, Debug, Display, Into, Deserialize, Serialize, Type)]
#[diesel(sql_type = Text)]
pub struct Db_CardId(String);

impl From<ygo::CardId> for Db_CardId {
  fn from(id: ygo::CardId) -> Self {
    Db_CardId(id.to_string())
  }
}

impl FromSql<Text, Sqlite> for Db_CardId {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    Ok(Db_CardId(<String as FromSql<Text, Sqlite>>::from_sql(
      bytes,
    )?))
  }
}

impl ToSql<Text, Sqlite> for Db_CardId
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.to_string());
    Ok(IsNull::No)
  }
}
