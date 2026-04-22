use super::BlockingDatabase;
use crate::error::{Error, Result};
use crate::model::card::{Card, NewCard};
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;

impl BlockingDatabase {
  pub fn create_card(&self, new_card: &NewCard) -> Result<usize> {
    use crate::schema::card;
    diesel::insert_into(card::table)
      .values(new_card)
      .on_conflict(card::card_id)
      .do_update()
      .set((
        card::archetype.eq(&new_card.archetype),
        card::banlist_status.eq(&new_card.banlist_status),
        card::image_url.eq(&new_card.image_url),
        card::image_url_cropped.eq(&new_card.image_url_cropped),
        card::image_url_small.eq(&new_card.image_url_small),
        card::price.eq(&new_card.price),
        card::updated_at.eq(Db_Zoned::now()),
      ))
      .execute(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn get_archetypes(&self) -> Result<Vec<String>> {
    use crate::schema::card;
    let archetypes: Vec<Option<String>> = card::table
      .filter(card::archetype.is_not_null())
      .select(card::archetype)
      .distinct()
      .load(&mut *self.conn())?;

    Ok(archetypes.into_iter().flatten().collect())
  }

  pub fn get_card_by_card_id(&self, card_id: &Db_CardId) -> Result<Card> {
    use crate::schema::card;
    card::table
      .filter(card::card_id.eq(card_id))
      .select(Card::as_select())
      .first(&mut *self.conn())
      .map_err(Error::from)
  }

  pub fn get_cards(&self) -> Result<Vec<Card>> {
    use crate::schema::card;
    card::table
      .select(Card::as_select())
      .load(&mut *self.conn())
      .map_err(Error::from)
  }
}
