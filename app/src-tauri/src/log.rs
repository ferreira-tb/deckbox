use anyhow::Result;
use nil_log::timer::Timer;
use std::io;
use tap::Pipe;
use tracing::subscriber::set_global_default;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn setup() -> Result<()> {
  let filter = EnvFilter::builder()
    .from_env()?
    .add_directive("deckbox=trace".parse()?)
    .add_directive("deckbox_database=trace".parse()?);

  let layer = Layer::default()
    .with_ansi(true)
    .with_timer(Timer)
    .with_writer(io::stderr)
    .pretty();

  Registry::default()
    .with(layer)
    .with(filter)
    .pipe(set_global_default)?;

  Ok(())
}
