use derive_more::{Deref, Display, From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(
  FromSqlRow,
  AsExpression,
  Clone,
  Copy,
  Debug,
  Default,
  Deref,
  Display,
  From,
  Into,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Deserialize,
  Serialize,
  Type,
)]
#[diesel(sql_type = Integer)]
pub struct Db_DeckCardAmount(u8);

impl FromSql<Integer, Sqlite> for Db_DeckCardAmount {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <i32 as FromSql<Integer, Sqlite>>::from_sql(bytes)?;
    Ok(Db_DeckCardAmount(u8::try_from(value)?))
  }
}

impl ToSql<Integer, Sqlite> for Db_DeckCardAmount
where
  i32: ToSql<Integer, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(i32::from(self.0));
    Ok(IsNull::No)
  }
}
