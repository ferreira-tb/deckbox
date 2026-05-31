use crate::Database;
use crate::error::{Error, Result};
use crate::model::card::{Db_Card, Db_NewCard};
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::card_race::Db_CardRace;
use crate::sql_types::card_type::Db_CardType;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use ygo::{CardRace, CardType};

impl Database {
  pub async fn create_card(&self, new_card: &Db_NewCard) -> Result<usize> {
    use crate::schema::card;

    let mut conn = self.0.lock().await;
    diesel::insert_into(card::table)
      .values(new_card)
      .on_conflict(card::card_id)
      .do_update()
      .set((
        card::archetype.eq(&new_card.archetype),
        card::banlist_status.eq(&new_card.banlist_status),
        card::description_pt.eq(&new_card.description_pt),
        card::image_url.eq(&new_card.image_url),
        card::image_url_cropped.eq(&new_card.image_url_cropped),
        card::image_url_small.eq(&new_card.image_url_small),
        card::name_pt.eq(&new_card.name_pt),
        card::ocg_date.eq(&new_card.ocg_date),
        card::price.eq(&new_card.price),
        card::tcg_date.eq(&new_card.tcg_date),
        card::updated_at.eq(Db_Zoned::now()),
      ))
      .execute(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn get_archetypes(&self) -> Result<Vec<String>> {
    use crate::schema::card;

    let mut conn = self.0.lock().await;
    let archetypes: Vec<Option<String>> = card::table
      .filter(card::tcg_date.is_not_null())
      .filter(card::archetype.is_not_null())
      .order(card::archetype.asc())
      .select(card::archetype)
      .distinct()
      .load(&mut *conn)
      .await?;

    Ok(archetypes.into_iter().flatten().collect())
  }

  pub async fn get_card_by_card_id(&self, card_id: &Db_CardId) -> Result<Db_Card> {
    use crate::schema::card;

    let mut conn = self.0.lock().await;
    card::table
      .filter(card::card_id.eq(card_id))
      .select(Db_Card::as_select())
      .first(&mut *conn)
      .await
      .map_err(Error::from)
  }

  pub async fn get_cards(&self) -> Result<Vec<Db_Card>> {
    use crate::schema::card;

    let mut conn = self.0.lock().await;
    card::table
      .filter(card::tcg_date.is_not_null())
      .filter(card::card_race.ne(Db_CardRace::from(CardRace::None)))
      .filter(card::card_type.ne_all([
        Db_CardType::from(CardType::SkillCard),
        Db_CardType::from(CardType::Token),
      ]))
      .order(card::name.asc())
      .select(Db_Card::as_select())
      .load(&mut *conn)
      .await
      .map_err(Error::from)
  }
}
