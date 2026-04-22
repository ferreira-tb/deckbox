pub mod card;
pub mod deck;
pub mod trunk;
pub mod wishlist;

use crate::error::CmdResult;
use tauri::WebviewWindow;

#[tauri::command]
#[specta::specta]
pub async fn show_window(window: WebviewWindow) -> CmdResult<()> {
  window.show()?;
  window.set_focus()?;
  Ok(())
}
