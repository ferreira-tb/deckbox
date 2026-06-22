use crate::Database;
use crate::error::{Error, Result};
use crate::model::deck_card::Db_DeckCard;
use crate::sql_types::num::Db_DeckId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use itertools::Itertools;

impl Database {
  pub async fn get_deck_cards(&self, deck_id: Db_DeckId) -> Result<Vec<Db_DeckCard>> {
    use crate::schema::deck_card;

    let mut conn = self.0.lock().await;
    deck_card::table
      .filter(deck_card::deck_id.eq(deck_id))
      .select(Db_DeckCard::as_select())
      .load(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn set_deck_cards(&self, deck_id: Db_DeckId, cards: &[Db_DeckCard]) -> Result<()> {
    use crate::schema::{deck, deck_card};

    let cards = cards
      .iter()
      .filter(|card| card.deck_id == deck_id)
      .unique_by(|card| card.card_id)
      .collect_vec();

    let mut conn = self.0.lock().await;
    let mut rows = diesel::delete(deck_card::table)
      .filter(deck_card::deck_id.eq(deck_id))
      .execute(&mut *conn)
      .await?;

    for card in cards {
      rows += diesel::insert_into(deck_card::table)
        .values(card)
        .execute(&mut *conn)
        .await?;
    }

    if rows > 0 {
      diesel::update(deck::table.find(deck_id))
        .set(deck::updated_at.eq(Db_Zoned::now()))
        .execute(&mut *conn)
        .await?;
    }

    Ok(())
  }
}
