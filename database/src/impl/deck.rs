use crate::Database;
use crate::error::{Error, Result};
use crate::model::deck::{Db_Deck, Db_NewDeck};
use crate::sql_types::num::Db_DeckId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

impl Database {
  pub async fn create_deck(&self, new_deck: &Db_NewDeck) -> Result<Db_DeckId> {
    use crate::schema::deck;

    let mut conn = self.0.lock().await;
    diesel::insert_into(deck::table)
      .values(new_deck)
      .on_conflict(deck::id)
      .do_update()
      .set((
        deck::description.eq(&new_deck.description),
        deck::updated_at.eq(Db_Zoned::now()),
      ))
      .returning(deck::id)
      .get_result(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn get_deck(&self, id: Db_DeckId) -> Result<Db_Deck> {
    use crate::schema::deck;

    let mut conn = self.0.lock().await;
    deck::table
      .find(id)
      .select(Db_Deck::as_select())
      .first(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn get_decks(&self) -> Result<Vec<Db_Deck>> {
    use crate::schema::deck;

    let mut conn = self.0.lock().await;
    deck::table
      .order(deck::name.asc())
      .select(Db_Deck::as_select())
      .load(&mut *conn)
      .await
      .map_err(Error::from)
  }
}
