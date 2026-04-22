use super::BlockingDatabase;
use crate::error::{Error, Result};
use crate::model::wishlist::{NewWish, Wish};
use crate::sql_types::card_id::Db_CardId;
use diesel::prelude::*;

impl BlockingDatabase {
  pub fn create_wish(&self, new_wish: &NewWish) -> Result<usize> {
    use crate::schema::wishlist;
    if self.has_trunk_entry(&new_wish.card_id)? {
      Ok(0)
    } else {
      diesel::insert_into(wishlist::table)
        .values(new_wish)
        .on_conflict(wishlist::card_id)
        .do_nothing()
        .execute(&mut *self.conn())
        .map_err(Error::from)
    }
  }

  pub fn get_wish_by_card_id(&self, card_id: &Db_CardId) -> Result<Wish> {
    use crate::schema::wishlist;
    wishlist::table
      .filter(wishlist::card_id.eq(card_id))
      .select(Wish::as_select())
      .first(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn get_wishlist(&self) -> Result<Vec<Wish>> {
    use crate::schema::wishlist;
    wishlist::table
      .select(Wish::as_select())
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
