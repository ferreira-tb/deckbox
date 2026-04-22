use super::BlockingDatabase;
use crate::error::{Error, Result};
use crate::model::deck::{Db_Deck, Db_NewDeck};
use crate::sql_types::id::Db_DeckId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;

impl BlockingDatabase {
  pub fn create_deck(&self, new_deck: &Db_NewDeck) -> Result<Db_DeckId> {
    use crate::schema::deck;
    diesel::insert_into(deck::table)
      .values(new_deck)
      .on_conflict(deck::id)
      .do_update()
      .set((
        deck::description.eq(&new_deck.description),
        deck::updated_at.eq(Db_Zoned::now()),
      ))
      .returning(deck::id)
      .get_result(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn get_decks(&self) -> Result<Vec<Db_Deck>> {
    use crate::schema::deck;
    deck::table
      .select(Db_Deck::as_select())
      .load(&mut *self.conn())
      .map_err(Error::from)
  }
}
