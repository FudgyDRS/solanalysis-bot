use proto_gen::ApiGatewayServer;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
mod config;
mod database;
mod grpc_server;
use grpc_server::MyApiGatewayServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = config::Config::from_env().await?;

  println!("Starting api-gateway at {}", config.api_gateway_addr);

  let db_pool = init_db_pool(&config.database_url).await?;
  let api_gateway_service = MyApiGatewayServer { db: db_pool };

  tonic::transport::Server::builder()
    .add_service(ApiGatewayServer::new(api_gateway_service))
    .serve(config.api_gateway_addr.parse()?)
    .await?;

  Ok(())
}

async fn init_db_pool(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
  PgPoolOptions::new()
    .max_connections(5)
    .connect(database_url)
    .await
}

async fn increment_counter(db: &PgPool) -> Result<i64, sqlx::Error> {
  let rec = sqlx::query_scalar!("update counters set value = value + 1 returning value")
    .fetch_one(db)
    .await?;

  Ok(rec)
}
