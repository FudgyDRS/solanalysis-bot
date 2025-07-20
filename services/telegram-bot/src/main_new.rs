use teloxide::prelude::*
use teloxide::types{InlinekeyboardButton, InlineKeyboardMarkup, CallbackQuery, Message};
use std::env;

mod grpc_client;
use grpc_client::increment_counter;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::str::SplitWhitespace;
use std::future::Future;
use std::pin::Pin;

type CommandHandler = Box<
  dyn Fn(Bot, Message, &mut SplitWhitespace<'_>) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>
    + Send
    + Sync,
>;

struct Command<'a> {
  description: &'a str,
  usage: &'a str,
  handler: CommandHandler,
}

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

  teloxide::repl(bot, |bot: Bot, msg: Message| {
    let user_states = user_states.clone();

    async move {
      if let Some(text) = msg.text() {
        let chat_id = msg.chat.id.0;
        let mut states = use_states.lock().unwrap();

        let mut parts = text.split_whitespace();
        match parts.next() {
          Some("/increment_counter") => {
            match increment_counter().await {
              Ok(count) => {
                bot.send_message(msg.chat.id, format!("Counter is now: {}", count))
                  .await?;
              }
              Err(e) => {
                bot.send_message(msg.chat.id, format!("Error: {}", e))
                  .await?;
              }
            }
          }
          Some("/track_wallet") {}
          Some("/help") {}
          _ => {
            bot.send_message(msg.chat.id, format!("Unknown command. Try /increment-counter\nYou sent: {}", text))
              .await?;
          }
        }
      }
    // let text = msg.text().unwrap_or("send a wallet address");

    ResponseResult::<()>::Ok(())
  })
  .await;
}
