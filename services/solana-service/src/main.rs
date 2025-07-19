use tonic::transport::Server;
use solana_client::rpc_client::RpcClient;
mod grpc_server;
use grpc_server::MySolanaServiceServer;
use proto_gen::solana_service::solana_service_server::SolanaServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Starting solana-service...");

  let addr = "[::1]:50052".parse()?;
  let solana_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

  let service = MySolanaServiceServer {
    solana_client,
  };

  println!("SolanaService listening on {}", addr);

  Server::builder()
    .add_service(SolanaServiceServer::new(service))
    .serve(addr)
    .await?;

  Ok(())
}
