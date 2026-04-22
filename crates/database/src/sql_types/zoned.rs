use derive_more::{Deref, Display, From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::str::FromStr;

#[derive(
  FromSqlRow, AsExpression, Clone, Debug, Deref, Display, From, Into, Deserialize, Serialize, Type,
)]
#[diesel(sql_type = Text)]
pub struct Db_Zoned(#[specta(type = String)] jiff::Zoned);

impl Db_Zoned {
  pub fn now() -> Self {
    Self(jiff::Zoned::now())
  }
}

impl FromSql<Text, Sqlite> for Db_Zoned {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(Db_Zoned(jiff::Zoned::from_str(value.as_str())?))
  }
}

impl ToSql<Text, Sqlite> for Db_Zoned
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.to_string());
    Ok(IsNull::No)
  }
}
