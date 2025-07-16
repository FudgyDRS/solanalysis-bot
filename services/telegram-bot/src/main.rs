use teloxide::{prelude::*, types::Message};
use std::env;

mod grpc_client;
use grpc_client::increment_counter;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  log::info!("Starting Solana Wallet Bot...");

  let bot = Bot::from_env(); // TELEGRAM_BOT_TOKEN

  teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    if let Some(text) = msg.text() {
      match text {
        "/increment-counter" => {
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
