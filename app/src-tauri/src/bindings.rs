use crate::commands;
use tauri::Wry;
use tauri_specta::{Builder, ErrorHandlingMode, collect_commands};

pub fn collect() -> Builder {
  let builder = Builder::<Wry>::new()
    .error_handling(ErrorHandlingMode::Throw)
    .commands(collect_commands![
      commands::show_window,
      commands::card::fetch_cards,
      commands::card::get_archetypes,
      commands::card::get_card_by_card_id,
      commands::card::get_cards,
      commands::trunk::create_trunk_entry,
      commands::trunk::decrease_trunk_entry_amount,
      commands::trunk::get_trunk,
      commands::trunk::get_trunk_entry_by_card_id,
      commands::trunk::has_trunk_entry,
      commands::trunk::increase_trunk_entry_amount,
      commands::wishlist::create_wish,
      commands::wishlist::get_wish_by_card_id,
      commands::wishlist::get_wishlist,
      commands::wishlist::remove_wish
    ]);

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
