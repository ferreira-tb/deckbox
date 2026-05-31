use crate::Database;
use crate::error::{Error, Result};
use crate::model::card::Db_Card;
use crate::model::trunk::{Db_NewTrunkEntry, Db_TrunkEntry};
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::num::{Db_TrunkEntryAmount, Db_TrunkEntryId};
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

impl Database {
  /// Creates a new trunk entry with an initial amount of 1.
  ///
  /// If the card is in the wishlist, it will be removed from there.
  pub async fn create_trunk_entry(&self, new_entry: &Db_NewTrunkEntry) -> Result<Db_TrunkEntryId> {
    use crate::schema::trunk;

    let mut conn = self.0.lock().await;
    let id = diesel::insert_into(trunk::table)
      .values(new_entry)
      .returning(trunk::id)
      .get_result(&mut *conn)
      .await?;

    drop(conn);

    self.remove_wish(&new_entry.card_id).await?;

    Ok(id)
  }

  /// Decreases the amount of a trunk entry by 1.
  /// If the amount is already 1, deletes the trunk entry instead.
  ///
  /// Returns the new amount of the trunk entry after the decrease, or 0 if the trunk entry was deleted.
  pub async fn decrease_trunk_entry_amount(
    &self,
    card_id: &Db_CardId,
  ) -> Result<Db_TrunkEntryAmount> {
    use crate::schema::trunk;

    let mut conn = self.0.lock().await;
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
    .get_result(&mut *conn)
    .await
    .optional()?;

    if let Some(new_amount) = new_amount {
      Ok(new_amount)
    } else {
      diesel::delete(
        trunk::table
          .filter(trunk::card_id.eq(card_id))
          .filter(trunk::amount.eq(1)),
      )
      .execute(&mut *conn)
      .await?;

      Ok(Db_TrunkEntryAmount::from(0u16))
    }
  }

  pub async fn get_trunk(&self) -> Result<Vec<Db_TrunkEntry>> {
    use crate::schema::{card, trunk};

    let mut conn = self.0.lock().await;
    trunk::table
      .inner_join(card::table.on(card::card_id.eq(trunk::card_id)))
      .filter(trunk::amount.gt(0))
      .order(card::name.asc())
      .select(Db_TrunkEntry::as_select())
      .load(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn get_trunk_cards(&self) -> Result<Vec<(Db_Card, Db_TrunkEntryAmount)>> {
    use crate::schema::{card, trunk};

    let mut conn = self.0.lock().await;
    card::table
      .inner_join(trunk::table.on(card::card_id.eq(trunk::card_id)))
      .filter(trunk::amount.gt(0))
      .order(card::name.asc())
      .select((Db_Card::as_select(), trunk::amount))
      .load(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn get_trunk_entry_by_card_id(&self, card_id: &Db_CardId) -> Result<Db_TrunkEntry> {
    use crate::schema::trunk;

    let mut conn = self.0.lock().await;
    trunk::table
      .filter(trunk::card_id.eq(card_id))
      .select(Db_TrunkEntry::as_select())
      .first(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn has_trunk_entry(&self, card_id: &Db_CardId) -> Result<bool> {
    use crate::schema::trunk;
    use diesel::dsl::{exists, select};

    let mut conn = self.0.lock().await;
    select(exists(trunk::table.filter(trunk::card_id.eq(card_id))))
      .get_result(&mut *conn)
      .await
      .map_err(Error::from)
  }

  /// Increases the amount of a trunk entry by 1.
  ///
  /// Returns the new amount of the trunk entry after the increase.
  pub async fn increase_trunk_entry_amount(
    &self,
    card_id: &Db_CardId,
  ) -> Result<Db_TrunkEntryAmount> {
    use crate::schema::trunk;

    let mut conn = self.0.lock().await;
    diesel::update(trunk::table.filter(trunk::card_id.eq(card_id)))
      .set((
        trunk::amount.eq(trunk::amount + 1),
        trunk::updated_at.eq(Db_Zoned::now()),
      ))
      .returning(trunk::amount)
      .get_result(&mut *conn)
      .await
      .map_err(Error::from)
  }
}
