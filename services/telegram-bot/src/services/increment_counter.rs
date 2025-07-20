use crate::grpc_client::api_gateway;

pub async fn increment_counter() -> Result<i64, Box<std::error::Error + Send + Sync>> {
  api_gateway::increment_counter().await
}
