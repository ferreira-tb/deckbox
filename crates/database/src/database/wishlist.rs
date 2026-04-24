use super::BlockingDatabase;
use crate::error::{Error, Result};
use crate::model::wishlist::{Db_NewWish, Db_Wish};
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::num::Db_WishId;
use diesel::prelude::*;

impl BlockingDatabase {
  pub fn create_wish(&self, new_wish: &Db_NewWish) -> Result<Option<Db_WishId>> {
    use crate::schema::wishlist;
    if self.has_trunk_entry(&new_wish.card_id)? {
      Ok(None)
    } else {
      diesel::insert_into(wishlist::table)
        .values(new_wish)
        .returning(wishlist::id)
        .get_result(&mut *self.conn())
        .map(Some)
        .map_err(Error::from)
    }
  }

  pub fn get_wish_by_card_id(&self, card_id: &Db_CardId) -> Result<Db_Wish> {
    use crate::schema::wishlist;
    wishlist::table
      .filter(wishlist::card_id.eq(card_id))
      .select(Db_Wish::as_select())
      .first(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn get_wishlist(&self) -> Result<Vec<Db_Wish>> {
    use crate::schema::{card, wishlist};
    wishlist::table
      .inner_join(card::table.on(card::card_id.eq(wishlist::card_id)))
      .order(card::name.asc())
      .select(Db_Wish::as_select())
      .load(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn remove_wish(&self, card_id: &Db_CardId) -> Result<usize> {
    use crate::schema::wishlist;
    diesel::delete(wishlist::table.filter(wishlist::card_id.eq(card_id)))
      .execute(&mut *self.conn())
      .map_err(Error::from)
  }
}
