use super::BlockingDatabase;
use crate::error::{Error, Result};
use crate::model::trunk::{NewTrunkEntry, TrunkEntry};
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;

impl BlockingDatabase {
  pub fn create_trunk_entry(&self, new_entry: &NewTrunkEntry) -> Result<usize> {
    use crate::schema::trunk;
    let mut rows = diesel::insert_into(trunk::table)
      .values(new_entry)
      .execute(&mut *self.conn())?;

    if rows > 0 {
      rows += self.remove_wish(&new_entry.card_id)?;
    }

    Ok(rows)
  }

  pub fn get_trunk(&self) -> Result<Vec<TrunkEntry>> {
    use crate::schema::trunk;
    trunk::table
      .filter(trunk::amount.gt(0))
      .select(TrunkEntry::as_select())
      .load(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn get_trunk_entry_by_card_id(&self, card_id: &Db_CardId) -> Result<TrunkEntry> {
    use crate::schema::trunk;
    trunk::table
      .filter(trunk::card_id.eq(card_id))
      .select(TrunkEntry::as_select())
      .first(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn has_trunk_entry(&self, card_id: &Db_CardId) -> Result<bool> {
    use crate::schema::trunk;
    use diesel::dsl::{exists, select};
    select(exists(trunk::table.filter(trunk::card_id.eq(card_id))))
      .get_result(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn set_trunk_entry_amount(&self, card_id: &Db_CardId, amount: u16) -> Result<usize> {
    use crate::schema::trunk;
    if amount == 0 {
      diesel::delete(trunk::table.filter(trunk::card_id.eq(card_id)))
        .execute(&mut *self.conn())
        .map_err(Error::from)
    } else {
      diesel::update(trunk::table.filter(trunk::card_id.eq(card_id)))
        .set((
          trunk::amount.eq(i32::from(amount)),
          trunk::updated_at.eq(Db_Zoned::now()),
        ))
        .execute(&mut *self.conn())
        .map_err(Error::from)
    }
  }
}
