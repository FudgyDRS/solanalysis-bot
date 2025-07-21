use crate::grpc_clients::solana_service::SolanaGrpcClient;

pub async fn track_wallet(network: &str, wallet_address: &str) -> Result<String, String> {
  // Ok(format!("Tracking wallet `{}` on network `{}`", wallet_address, network))

  let mut grpc_client = SolanaGrpcClient::connect("http://[::1]:50052")
    .await
    .map_err(|e| format!("Failed to connect to Solana gRPC service: {}", e))?;

  grpc_client.track_wallet(wallet_address).await
}
