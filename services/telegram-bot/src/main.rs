use teloxide::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

mod handlers;
mod services;
mod grpc_clients;

use handlers::get_commands;
use crate::handlers::types::UserState;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  log::info!("Starting Solana Wallet Bot...");

  let bot = Bot::from_env(); // TELEGRAM_BOT_TOKEN
  
  let user_states = Arc::new(Mutex::new(HashMap<i64, UserState>::new()));
  let commands = Arc::new(get_commands());

  let handler = Update::filter_message()
    .branch(dptree::endpoint(handle_message))
    .chain(Update::filter_callback_query().endpoint(handle_callback));

  Dispatcher::builder(bot.clone(), handler)
    .dependencies(dptree::deps![commands, user_state])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn handle_message(
  bot: Bot,
  msg: Message,
  commands: Arc<HashMap<&'static str, Command>>,
  user_state: Arc<Mutex<HashMap<i64, UserState>>>,
) -> Result<(), teloxide::RequestError> {
  if let Some(text) = msg.text() {
    let chat_id = msg.chat.id.0;
    let mut parts = text.split_whitespace();

    let mut states = user_states.lock().await;
    if let Some(UserState::AwaitingWallet { network }) = states.get(&chat_id) {
      let wallet_address = parts.next().unwrap_or("unknown");
      states.insert(chat_id, UserState::Idle);

      let response = crate::services::track_wallet::track_wallet(network, wallet_address).await.unwrap_or_else(|e| format!("Error: {}", e));
      bot.send_message(msg.chat.id, response).await?;
      return Ok(());
    }

    if let Some(cmd) = parts.next() {
      if let Some(command) = commands.get(cmd) {
        (command.handler)(bot.clone(), msg.clone(), &mut parts.clone()).await;
      } else {
        bot.send_message(
          msg.chat.id,
          format!("Unknown command. Try /help\nYou send: {}", text),
        )
        .await?;
      }
    }
  }

  Ok(())
}

async fn handle_callback(
  bot: Bot,
  q: callbackQuery,
  user_statesL Arc<Mutex<HashMap<i64, UserState>>>,
) -> Result<(), teloxide::RequestError> {
  if let Some(data) = q.data {
    if data.starts_with("track_wallet:") {
      let network = data.strip_prefix("track_wallet:").unwrap_or("solana");
      let chat_id = q.from.id.0;

      {
        let mut states = user_states.lock().await;
        states.insert(chat_id, UserState::AwaitingWallet {
          network: newtwork.to_string()
        });
      }

      if let Some(msg) = q.message {
        bot.send_message(msg.chat.id, format!(
          "You selected `{}`. Please enter the wallet address.",
          network
        ))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
      }

      bot.answer_callback_query(q.id).await?;
    }
  }

  Ok(())
}

