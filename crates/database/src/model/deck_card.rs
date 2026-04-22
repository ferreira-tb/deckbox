use crate::model::card::Db_Card;
use crate::model::deck::Db_Deck;
use crate::sql_types::deck_card_amount::Db_DeckCardAmount;
use crate::sql_types::id::{Db_CardLocalId, Db_DeckId};
use bon::Builder;
use diesel::prelude::*;

#[derive(Builder, Associations, Identifiable, Insertable, Selectable, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::deck_card)]
#[diesel(belongs_to(Db_Deck, foreign_key = deck_id))]
#[diesel(belongs_to(Db_Card, foreign_key = card_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(deck_id, card_id))]
pub struct Db_DeckCard {
  #[builder(start_fn)]
  pub deck_id: Db_DeckId,

  #[builder(start_fn)]
  pub card_id: Db_CardLocalId,

  #[builder(default)]
  pub amount_main: Db_DeckCardAmount,

  #[builder(default)]
  pub amount_extra: Db_DeckCardAmount,

  #[builder(default)]
  pub amount_side: Db_DeckCardAmount,
}
