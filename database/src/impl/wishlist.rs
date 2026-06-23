use crate::Database;
use crate::error::{Error, Result};
use crate::model::wishlist::{Db_NewWish, Db_Wish};
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::num::Db_WishId;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

impl Database {
  pub async fn create_wish(&self, new_wish: &Db_NewWish) -> Result<Option<Db_WishId>> {
    use crate::schema::wishlist;
    if self
      .has_trunk_entry(&new_wish.card_id)
      .await?
    {
      Ok(None)
    } else {
      let mut conn = self.0.lock().await;
      diesel::insert_into(wishlist::table)
        .values(new_wish)
        .returning(wishlist::id)
        .get_result(&mut *conn)
        .await
        .map(Some)
        .map_err(Error::from)
    }
  }

  pub async fn get_wish_by_card_id(&self, card_id: &Db_CardId) -> Result<Db_Wish> {
    use crate::schema::wishlist;

    let mut conn = self.0.lock().await;
    wishlist::table
      .filter(wishlist::card_id.eq(card_id))
      .select(Db_Wish::as_select())
      .first(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn get_wishlist(&self) -> Result<Vec<Db_Wish>> {
    use crate::schema::{card, wishlist};

    let mut conn = self.0.lock().await;
    wishlist::table
      .inner_join(card::table.on(card::card_id.eq(wishlist::card_id)))
      .order(card::name.asc())
      .select(Db_Wish::as_select())
      .load(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn remove_wish(&self, card_id: &Db_CardId) -> Result<usize> {
    use crate::schema::wishlist;

    let mut conn = self.0.lock().await;
    diesel::delete(wishlist::table.filter(wishlist::card_id.eq(card_id)))
      .execute(&mut *conn)
      .await
      .map_err(Error::from)
  }
}
