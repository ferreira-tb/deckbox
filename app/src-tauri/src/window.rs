use crate::manager::ManagerExt;
use anyhow::Result;
use serde_json::json;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Wry};

pub trait WindowExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl<T: Manager<Wry>> WindowExt for T {}

pub fn open(app: &AppHandle) -> Result<()> {
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, "main", url)
    .title("Deckbox")
    .initialization_script(script(app)?)
    .inner_size(1280.0, 768.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .maximized(true)
    .prevent_overflow()
    .build()?;

  Ok(())
}

fn script(app: &AppHandle) -> Result<String> {
  let mut script = String::new();
  macro_rules! define {
    ($name:literal, $value:expr) => {{
      let name = $name;
      let value = json!($value);
      let snippet = format! {"
        Object.defineProperty(window, '{name}', {{
          configurable: false,
          enumerable: true,
          writable: false,
          value: {value},
        }});
      "};

      script.push_str(&snippet);
    }};
  }

  define!("__DEBUG_ASSERTIONS__", cfg!(debug_assertions));
  define!("__DESKTOP__", cfg!(desktop));
  define!("__MOBILE__", cfg!(mobile));
  define!("__VERSION__", env!("CARGO_PKG_VERSION"));

  define!("__DECKBOX_DIR__", app.deckbox_dir()?);
  define!("__DECKBOX_IMG_DIR__", app.img_dir()?);
  define!("__DECKBOX_IMG_DIR_CROPPED__", app.img_dir_cropped()?);
  define!("__DECKBOX_IMG_DIR_SMALL__", app.img_dir_small()?);

  Ok(script)
}
