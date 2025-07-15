use teloxide::prelude::*;
use teloxide::types::Message;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  log::info!("Starting Solana Wallet Bot...");

  let bot = Bot::from_env(); // TELEGRAM_BOT_TOKEN

  teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    let text = msg.text().unwrap_or("send a wallet address");

    bot.send_message(msg.chat.id, format!("You sent: {}", text))
      .await?;

    ResponseResult::<()>::Ok(())
  })
  .await;
}
