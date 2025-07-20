use teloxide::prelude::*
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

mod handlers;
mod services;
mod grpc_clients;

use handlers::get_commands;

#[derive(Clone, Debug)]
enum UserState {
  Idle,
  AwaitingWallet { network: String },
}

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  log::info!("Starting Solana Wallet Bot...");

  let bot = Bot::from_env(); // TELEGRAM_BOT_TOKEN
  let user_states = Arc::new(Mutex::new(Hashmap::<i64, UserState>::new()));
  let commands = get_commands();

  teloxide::repl(bot, |bot: Bot, msg: Message| {
    let commands = commands.clone();
    let user_states = user_states.clone();

    async move {
      if let Some(text) = msg.text() {
        let mut parts = text.split_whitespace();
        if let Some(cmd) = parts.next() {
          if let Some(command) = commands.get(cmd) {
            (command.handler)(bot.clone(), msg.clone(), &mut parts).await;
          } else {
            bot.send_message(msg.chat.id, "Unknown command. Try /help\nYou send: {}", msg.text()).await.unwrap();
          }
        }
      }
      Ok::<(), teloxide::RequestError>(())
    }
  })
  .await;
}
