use anyhow::Result;
use reqwest::Client;
use std::sync::LazyLock;
use tap::Pipe;

static HTTP: LazyLock<Client> = LazyLock::new(|| {
  let repository = "https://github.com/ferreira-tb/deckbox";
  let user_agent = format!("deckbox/{} ({})", env!("CARGO_PKG_VERSION"), repository);

  Client::builder()
    .use_rustls_tls()
    .https_only(true)
    .user_agent(user_agent)
    .build()
    .expect("failed to create http client")
});

pub async fn get_bytes(url: &str) -> Result<Vec<u8>> {
  HTTP
    .get(url)
    .send()
    .await?
    .error_for_status()?
    .bytes()
    .await?
    .to_vec()
    .pipe(Ok)
}
