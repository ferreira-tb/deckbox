use crate::model::card::Db_Card;
use crate::model::deck::Db_Deck;
use crate::sql_types::deck_card_amount::Db_DeckCardAmount;
use crate::sql_types::num::{Db_CardLocalId, Db_DeckId};
use bon::Builder;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(
  Builder,
  Associations,
  Identifiable,
  Insertable,
  Selectable,
  Queryable,
  Clone,
  Debug,
  Serialize,
  Deserialize,
  Type,
)]
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

  #[builder(default, into)]
  pub main: Db_DeckCardAmount,

  #[builder(default, into)]
  pub extra: Db_DeckCardAmount,

  #[builder(default, into)]
  pub side: Db_DeckCardAmount,
}

impl Db_DeckCard {
  #[inline]
  pub fn id(&self) -> Db_DeckCardId {
    Db_DeckCardId {
      deck_id: self.deck_id,
      card_id: self.card_id,
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub struct Db_DeckCardId {
  pub deck_id: Db_DeckId,
  pub card_id: Db_CardLocalId,
}
