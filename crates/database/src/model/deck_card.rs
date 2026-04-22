use crate::model::card::Db_Card;
use crate::model::deck::Db_Deck;
use diesel::prelude::*;

#[derive(Associations, Identifiable, Insertable, Selectable, Queryable, Debug)]
#[diesel(table_name = crate::schema::deck_card)]
#[diesel(belongs_to(Db_Deck, foreign_key = deck_id))]
#[diesel(belongs_to(Db_Card, foreign_key = card_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(deck_id, card_id))]
pub struct Db_DeckCard {
  pub deck_id: i32,
  pub card_id: i32,
  pub quantity: i32,
}
