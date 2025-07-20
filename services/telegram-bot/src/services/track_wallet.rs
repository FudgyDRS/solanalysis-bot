pub async fn track_wallet(network: &str, wallet_address: &str) -> Result<String, String> {
  Ok(format!("Tracking wallet `{}` on network `{}`", wallet_address, network))
}
