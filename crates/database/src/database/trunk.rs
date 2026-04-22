use super::BlockingDatabase;
use crate::error::{Error, Result};
use crate::model::trunk::{NewTrunkEntry, TrunkEntry};
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::trunk_entry_amount::Db_TrunkEntryAmount;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;

impl BlockingDatabase {
  /// Creates a new trunk entry with an initial amount of 1.
  ///
  /// If the card is in the wishlist, it will be removed from there.
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

  /// Decreases the amount of a trunk entry by 1.
  /// If the amount is already 1, deletes the trunk entry instead.
  ///
  /// Returns the new amount of the trunk entry after the decrease, or 0 if the trunk entry was deleted.
  pub fn decrease_trunk_entry_amount(&self, card_id: &Db_CardId) -> Result<u16> {
    use crate::schema::trunk;
    let new_amount: Option<Db_TrunkEntryAmount> = diesel::update(
      trunk::table
        .filter(trunk::card_id.eq(card_id))
        .filter(trunk::amount.gt(1)),
    )
    .set((
      trunk::amount.eq(trunk::amount - 1),
      trunk::updated_at.eq(Db_Zoned::now()),
    ))
    .returning(trunk::amount)
    .get_result(&mut *self.conn())
    .optional()?;

    if let Some(new_amount) = new_amount {
      Ok(u16::from(new_amount))
    } else {
      diesel::delete(
        trunk::table
          .filter(trunk::card_id.eq(card_id))
          .filter(trunk::amount.eq(1)),
      )
      .execute(&mut *self.conn())?;

      Ok(0)
    }
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

  /// Increases the amount of a trunk entry by 1.
  ///
  /// Returns the new amount of the trunk entry after the increase.
  pub fn increase_trunk_entry_amount(&self, card_id: &Db_CardId) -> Result<Db_TrunkEntryAmount> {
    use crate::schema::trunk;
    diesel::update(trunk::table.filter(trunk::card_id.eq(card_id)))
      .set((
        trunk::amount.eq(trunk::amount + 1),
        trunk::updated_at.eq(Db_Zoned::now()),
      ))
      .returning(trunk::amount)
      .get_result(&mut *self.conn())
      .map_err(Error::from)
  }
}
