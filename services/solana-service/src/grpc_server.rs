use tonic::{Request, Response, Status};
use solana_client::rpc_client::RpcClient;
use proto_gen::solana_service::{TrackWalletRequest, TrackWalletResponse};
use proto_gen::solana_service::solana_service_server::SolanaService;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub struct MySolanaServiceServer {
  pub solana_client: RpcClient,
}

#[tonic::async_trait]
impl SolanaService for MySolanaServiceServer {
  async fn track_wallet(
    &self,
    request: Request<TrackWalletRequest>,
  ) -> Result<Response<TrackWalletResponse>, Status> {
    let req = request.into_inner();
    let pubkey = Pubkey::from_str(&req.wallet_address)
      .map_err(|e| Status::invalid_argument(format!("Invalid pubkey: {}", e)))?;

    let balance = self
      .solana_client
      .get_balance(&pubkey)
      .map_err(|e| Status::internal(format!("RPC error: {}", e)))?;

    // solana query
    let response = TrackWalletResponse {
      message: format!("Wallet {} balance: {}", req.wallet_address, balance),
      balance: balance as i64,
      transactions: None,
    };

    Ok(Response::new(response))
  }
}
