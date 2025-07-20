use crate::grpc_clients::api_gateway;

pub async fn increment_counter() -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
  api_gateway::increment_counter().await
}
