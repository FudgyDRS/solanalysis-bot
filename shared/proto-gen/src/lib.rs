pub mod api_gateway;
pub mod solana_service;

pub use api_gateway::api_gateway_client::ApiGatewayClient;
pub use api_gateway::api_gateway_server::ApiGatewayServer;
pub use api_gateway::{IncrementRequest, IncrementResponse};

pub use solana_service::solana_service_client::SolanaServiceClient;
pub use solana_service::solana_service_server::{SolanaService, SolanaServiceServer};
pub use solana_service::{TrackWalletRequest, TrackWalletResponse, SlotRange, TimeRange, Transactions, Transaction};
