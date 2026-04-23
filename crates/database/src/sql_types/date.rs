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
pub struct Db_Date(#[specta(type = String)] jiff::civil::Date);

impl FromSql<Text, Sqlite> for Db_Date {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(Db_Date(jiff::civil::Date::from_str(value.as_str())?))
  }
}

impl ToSql<Text, Sqlite> for Db_Date
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.to_string());
    Ok(IsNull::No)
  }
}
