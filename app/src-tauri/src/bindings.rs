use crate::{commands, settings};
use deckbox_database::model::deck::Db_NewDeck;
use tauri::Wry;
use tauri_specta::{Builder, ErrorHandlingMode, collect_commands};

pub fn collect() -> Builder {
  let builder = Builder::<Wry>::new()
    .error_handling(ErrorHandlingMode::Throw)
    .constant("SETTINGS_BACKUP_DIR", settings::BACKUP_DIR)
    .constant("SETTINGS_BANLIST_DIR", settings::BANLIST_DIR)
    .constant("SETTINGS_CAN_EDIT", settings::CAN_EDIT)
    .constant("SETTINGS_CHECK_TRUNK", settings::CHECK_TRUNK)
    .constant("SETTINGS_DECK_DIR", settings::DECK_DIR)
    .constant("SETTINGS_STORE_ID", settings::STORE_ID)
    .constant("SETTINGS_TRUNK_DIR", settings::TRUNK_DIR)
    .commands(collect_commands![
      commands::export_database_file,
      commands::open_settings_file,
      commands::open_store_website,
      commands::show_window,
      commands::card::fetch_cards,
      commands::card::get_archetypes,
      commands::card::get_card,
      commands::card::get_card_by_card_id,
      commands::card::get_cards,
      commands::deck::create_deck,
      commands::deck::export_decks,
      commands::deck::get_deck,
      commands::deck::get_decks,
      commands::deck::remove_deck,
      commands::deck::rename_deck,
      commands::deck_card::get_deck_cards,
      commands::deck_card::set_deck_cards,
      commands::trunk::create_trunk_entry,
      commands::trunk::decrease_trunk_entry_amount,
      commands::trunk::export_trunk,
      commands::trunk::get_trunk,
      commands::trunk::get_trunk_cards,
      commands::trunk::get_trunk_entry_by_card_id,
      commands::trunk::has_trunk_entry,
      commands::trunk::increase_trunk_entry_amount,
      commands::wishlist::create_wish,
      commands::wishlist::get_wish_by_card_id,
      commands::wishlist::get_wishlist,
      commands::wishlist::remove_wish
    ])
    .typ::<Db_NewDeck>();

  #[cfg(debug_assertions)]
  export(&builder);

  builder
}

#[cfg(debug_assertions)]
fn export(specta: &Builder) {
  use specta_typescript::{BigIntExportBehavior, Typescript};

  let ts = Typescript::default()
    .bigint(BigIntExportBehavior::BigInt)
    .header("// @ts-nocheck");

  specta
    .export(ts, "../src/lib/bindings.ts")
    .expect("failed to export typescript bindings");
}
