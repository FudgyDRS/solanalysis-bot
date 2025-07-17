use tonic::{Request, Response, Status};
use sqlx::PgPool;
use proto_gen::solana_service::{TrackWalletRequest, TrackWalletResponse};
use proto_gen::solana_service::solana_service_server::SolanaService;
use crate::database::PgPoolExt;

pub struct MySolanaServiceServer {
  pub db: PgPool;
}

#[tonic::async_trait]
impl SolanaService for MySolanaServiceServer {
  async fn track_wallet(
    &self,
    request: Request<TrackWalletRequest>,
  ) -> Result<Response<TrackWalletResponse>, Status> {
    let req = request.into_inner();
    let wallet_address = req.wallet_address;

    // solana query
    let response = TrackWalletResponse {
      message: format!("Track wallet {}", wallet_address),
      balance: 0,
      transactions: None,
    };

    Ok(Response::new(response))
  }
}
