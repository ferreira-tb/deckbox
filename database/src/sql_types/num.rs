use derive_more::{Deref, Display, From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use specta::Type;

macro_rules! impl_infallible {
  ($($name:ident => $kind:ident),+ $(,)?) => {
    $(
      #[derive(
        FromSqlRow,
        AsExpression,
        Clone,
        Copy,
        Debug,
        Deref,
        Display,
        From,
        Into,
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
      pub struct $name($kind);

      impl FromSql<Integer, Sqlite> for $name
      where
        i32: FromSql<Integer, Sqlite>,
      {
        fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
          let value = <i32 as FromSql<Integer, Sqlite>>::from_sql(bytes)?;
          Ok($name($kind::try_from(value)?))
        }
      }

      impl ToSql<Integer, Sqlite> for $name
      where
        i32: ToSql<Integer, Sqlite>,
      {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
          out.set_value(i32::from(self.0));
          Ok(IsNull::No)
        }
      }

      impl PartialEq<$kind> for $name {
        fn eq(&self, other: &$kind) -> bool {
          self.0 == *other
        }
      }

      impl PartialOrd<$kind> for $name {
        fn partial_cmp(&self, other: &$kind) -> Option<std::cmp::Ordering> {
          self.0.partial_cmp(other)
        }
      }

      impl From<$name> for i32 {
        fn from(value: $name) -> Self {
          i32::from(value.0)
        }
      }
    )+
  }
}

impl_infallible!(Db_TrunkEntryAmount => u16);

macro_rules! impl_fallible {
  ($($name:ident => $kind:ident),+ $(,)?) => {
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
      pub struct $name($kind);

      impl FromSql<Integer, Sqlite> for $name
      where
        i32: FromSql<Integer, Sqlite>,
      {
        fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
          let value = <i32 as FromSql<Integer, Sqlite>>::from_sql(bytes)?;
          Ok($name($kind::try_from(value)?))
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

      impl PartialEq<$kind> for $name {
        fn eq(&self, other: &$kind) -> bool {
          self.0 == *other
        }
      }

      impl PartialOrd<$kind> for $name {
        fn partial_cmp(&self, other: &$kind) -> Option<std::cmp::Ordering> {
          self.0.partial_cmp(other)
        }
      }
    )+
  }
}

impl_fallible!(
  Db_CardLocalId => u32,
  Db_DeckId => u32,
  Db_TrunkEntryId => u32,
  Db_WishId => u32
);
