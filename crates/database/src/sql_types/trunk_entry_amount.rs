use derive_more::{Deref, Display, From, Into};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::num::NonZeroU16;

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
  Deserialize,
  Serialize,
  Type,
)]
#[diesel(sql_type = Integer)]
pub struct Db_TrunkEntryAmount(NonZeroU16);

impl Db_TrunkEntryAmount {
  pub fn new(amount: u16) -> Option<Self> {
    NonZeroU16::new(amount).map(Db_TrunkEntryAmount)
  }
}

impl FromSql<Integer, Sqlite> for Db_TrunkEntryAmount {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <i32 as FromSql<Integer, Sqlite>>::from_sql(bytes)?;
    let value = NonZeroU16::try_from(u16::try_from(value)?)?;
    Ok(Db_TrunkEntryAmount(value))
  }
}

impl ToSql<Integer, Sqlite> for Db_TrunkEntryAmount
where
  i32: ToSql<Integer, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(i32::from(*self));
    Ok(IsNull::No)
  }
}

impl PartialEq<u16> for Db_TrunkEntryAmount {
  fn eq(&self, other: &u16) -> bool {
    self.0.get() == *other
  }
}

impl PartialOrd<u16> for Db_TrunkEntryAmount {
  fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
    self.0.get().partial_cmp(other)
  }
}

impl From<Db_TrunkEntryAmount> for i32 {
  fn from(value: Db_TrunkEntryAmount) -> Self {
    i32::from(value.0.get())
  }
}

impl From<Db_TrunkEntryAmount> for u16 {
  fn from(value: Db_TrunkEntryAmount) -> Self {
    value.0.get()
  }
}
