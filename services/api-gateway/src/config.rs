use dotenv::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub database_url: String,
  pub api_gateway_addr: String,
}

impl Config {
  pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let api_gateway_addr = env::var("API_GATEWAY_ADDR")?;
    Ok(Config {
      database_url,
      api_gateway_addr,
    })
  }
}
