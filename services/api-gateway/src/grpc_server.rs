use tonic::{Request, Response, Status};
use sqlx::PgPool;
use proto_gen::api_gateway::{IncrementRequest, IncrementResponse};
use proto_gen::api_gateway::api_gateway_server::ApiGateway;
use crate::database::PgPoolExt;

pub struct MyApiGatewayServer {
  pub db: PgPool,
}

#[tonic::async_trait]
impl ApiGateway for MyApiGatewayServer {
  async fn increment_counter(
    &self,
    _request: tonic::Request<IncrementRequest>,
  ) -> Result<tonic::Response<IncrementResponse>, tonic::Status> {
    // Connect to db, increment counter, fetch new value
    let new_value = self.db.increment_counter()
      .await
      .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;

    Ok(tonic::Response::new(IncrementResponse { new_value }))
  }
}

