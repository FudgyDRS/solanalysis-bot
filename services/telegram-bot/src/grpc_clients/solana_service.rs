use tonic::transport::Channel;
use tonic::Request;

use proto_gen::solana_service::{
  solana_service_client::SolanaServiceClient,
  TrackWalletRequest,
};

pub struct SolanaGrpcClient {
  client: SolanaServiceClient<Channel>,
}

impl SolanaGrpcClient {
  pub async fn connect(addr: &str) -> Result<Self, tonic::transport::Error> {
    let client = SolanaServiceClient::connect(addr.to_string()).await?;
    Ok(Self { client })
  }

  pub async fn track_wallet(&mut self, wallet_address: &str) -> Result<String, String> {
    let request = Request::new(TrackWalletRequest {
      wallet_address: wallet_address.to_string(),
      filter: None, 
    });

    let response = self.client.track_wallet(request).await
      .map_err(|e| format!("GRPC call failed: {}", e))?;

    Ok(response.into_inner().message)
  }
}
