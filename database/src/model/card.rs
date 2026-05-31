use crate::sql_types::banlist_status::Db_BanlistStatus;
use crate::sql_types::card_attribute::Db_CardAttribute;
use crate::sql_types::card_id::Db_CardId;
use crate::sql_types::card_race::Db_CardRace;
use crate::sql_types::card_type::Db_CardType;
use crate::sql_types::date::Db_Date;
use crate::sql_types::num::Db_CardLocalId;
use crate::sql_types::url::Db_Url;
use crate::sql_types::zoned::Db_Zoned;
use diesel::prelude::*;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::str::FromStr;

#[derive(Queryable, Selectable, Clone, Debug, Serialize, Deserialize, Type)]
#[diesel(table_name = crate::schema::card)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Db_Card {
  pub id: Db_CardLocalId,
  pub name: String,
  pub description: String,
  pub card_id: Db_CardId,
  pub card_type: Db_CardType,
  pub card_type_human: Option<String>,
  pub card_race: Db_CardRace,
  pub attack: Option<i32>,
  pub defense: Option<i32>,
  pub level: Option<i32>,
  pub linkval: Option<i32>,
  pub attribute: Option<Db_CardAttribute>,
  pub archetype: Option<String>,
  pub banlist_status: Option<Db_BanlistStatus>,
  pub image_url: Db_Url,
  pub image_url_cropped: Db_Url,
  pub image_url_small: Db_Url,
  pub price: Option<String>,
  pub name_pt: Option<String>,
  pub description_pt: Option<String>,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::card)]
pub struct Db_NewCard {
  pub(crate) name: String,
  pub(crate) description: String,
  pub(crate) card_id: Db_CardId,
  pub(crate) card_type: Db_CardType,
  pub(crate) card_type_human: Option<String>,
  pub(crate) card_race: Db_CardRace,
  pub(crate) attack: Option<i32>,
  pub(crate) defense: Option<i32>,
  pub(crate) level: Option<i32>,
  pub(crate) linkval: Option<i32>,
  pub(crate) attribute: Option<Db_CardAttribute>,
  pub(crate) archetype: Option<String>,
  pub(crate) banlist_status: Option<Db_BanlistStatus>,
  pub(crate) image_url: Db_Url,
  pub(crate) image_url_cropped: Db_Url,
  pub(crate) image_url_small: Db_Url,
  pub(crate) price: Option<String>,
  pub(crate) created_at: Db_Zoned,
  pub(crate) updated_at: Db_Zoned,
  pub(crate) ocg_date: Option<Db_Date>,
  pub(crate) tcg_date: Option<Db_Date>,
  pub name_pt: Option<String>,
  pub description_pt: Option<String>,
}

impl Db_NewCard {
  pub fn from_ygo_card(mut card: ygo::Card) -> Option<Self> {
    let card_id = card.id?;
    let now = Db_Zoned::now();

    let image = card
      .card_images
      .iter_mut()
      .find(|it| it.id == Some(u32::from(card_id)))?;

    let banlist_status = card
      .banlist_info
      .and_then(|it| it.ban_tcg)
      .map(Db_BanlistStatus::from);

    let price = card
      .card_prices
      .iter()
      .filter_map(|it| it.tcgplayer_price.as_deref())
      .filter_map(to_price_f64)
      .filter(|it| *it > 0.0)
      .min_by(f64::total_cmp)
      .map(|it| it.to_string());

    let ocg_date = card
      .misc_info
      .iter()
      .find_map(|it| it.ocg_date.as_deref().and_then(to_date));

    let tcg_date = card
      .misc_info
      .iter()
      .find_map(|it| it.tcg_date.as_deref().and_then(to_date));

    Some(Self {
      name: card.name?,
      description: card.desc?,
      card_id: Db_CardId::from(card_id),
      card_type: Db_CardType::from(card.r#type?),
      card_type_human: card.human_readable_card_type,
      card_race: Db_CardRace::from(card.race?),
      attack: card.atk.and_then(|it| it.to_i32()),
      defense: card.def.and_then(|it| it.to_i32()),
      level: card.level.and_then(|it| it.to_i32()),
      linkval: card.linkval.and_then(|it| it.to_i32()),
      attribute: card.attribute.map(Db_CardAttribute::from),
      archetype: card.archetype,
      banlist_status,
      image_url: image.image_url.take()?.into(),
      image_url_cropped: image.image_url_cropped.take()?.into(),
      image_url_small: image.image_url_small.take()?.into(),
      price,
      created_at: now.clone(),
      updated_at: now,
      ocg_date,
      tcg_date,
      name_pt: None,
      description_pt: None,
    })
  }

  pub fn card_id(&self) -> &Db_CardId {
    &self.card_id
  }

  pub fn image_url(&self) -> &Db_Url {
    &self.image_url
  }

  pub fn image_url_cropped(&self) -> &Db_Url {
    &self.image_url_cropped
  }

  pub fn image_url_small(&self) -> &Db_Url {
    &self.image_url_small
  }
}

fn to_date(date: &str) -> Option<Db_Date> {
  match jiff::civil::Date::from_str(date) {
    Ok(date) => Some(Db_Date::from(date)),
    #[cfg(debug_assertions)]
    Err(err) => {
      tracing::warn!("failed to parse date: {date}, error: {err}");
      None
    }
    #[cfg(not(debug_assertions))]
    Err(_) => None,
  }
}

fn to_price_f64(price: &str) -> Option<f64> {
  match price.parse::<f64>() {
    Ok(price) => Some(price),
    #[cfg(debug_assertions)]
    Err(err) => {
      tracing::warn!("failed to parse price: {price}, error: {err}");
      None
    }
    #[cfg(not(debug_assertions))]
    Err(_) => None,
  }
}
