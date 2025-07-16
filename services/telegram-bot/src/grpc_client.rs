use proto_gen::{ApiGatewayClient, IncrementRequest};

pub async fn increment_counter() -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
  let mut client = ApiGatewayClient::connect("http://127.0.0.1:50051").await?;

  let request = tonic::Request::new(IncrementRequest {});

  let response = client.increment_counter(request).await?;

  Ok(response.into_inner().new_value)
}
