use derive_more::{Deref, Display};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use specta::Type;

macro_rules! impl_u32 {
  ($($name:ident),+ $(,)?) => {
    $(
      #[derive(
        FromSqlRow,
        AsExpression,
        Clone,
        Copy,
        Debug,
        Deref,
        Display,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Deserialize,
        Serialize,
        Type,
      )]
      #[diesel(sql_type = Integer)]
      pub struct $name(u32);

      impl FromSql<Integer, Sqlite> for $name
      where
        i32: FromSql<Integer, Sqlite>,
      {
        fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
          let value = i32::from_sql(bytes)?;
          Ok($name(u32::try_from(value)?))
        }
      }

      impl ToSql<Integer, Sqlite> for $name
      where
        i32: ToSql<Integer, Sqlite>,
      {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
          out.set_value(i32::try_from(self.0)?);
          Ok(IsNull::No)
        }
      }
    )+
  }
}

impl_u32!(Db_CardLocalId, Db_DeckId);
