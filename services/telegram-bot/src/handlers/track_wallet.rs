use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use std::str::SplitWhitespace;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::services::track_wallet;
use crate::handlers::types::{Command, CommandHandler, UserState};

pub async fn handle(
  bot: Bot, 
  msg: Message,
  parts: &mut SplitWhitespace<'_>, 
  user_states: Arc<Mutex<HashMap<i64, UserState>>>,
) {
  if parts.next().is_none() {
    let keyboard = InlineKeyboardMarkup::new(vec![vec![
      InlineKeyboardButton::callback("Solana", "track_wallet:solana"),
      InlineKeyboardButton::callback("Eclipse", "track_wallet:eclipse"),
    ]]);

    bot.send_message(msg.chat.id, "Choose a network:")
      .reply_markup(keyboard)
      .await
      .unwrap();

    let mut states = user_states.lock().await;
    states.insert(msg.chat.id.0, UserState::AwaitingNetwork);

    return;
  }

  let network = parts.next().unwrap_or("solana");
  let wallet_address = parts.next().unwrap_or("unknown");

  match track_wallet::track_wallet(network, wallet_address).await {
    Ok(response) => {
      bot.send_message(msg.chat.id, response).await.unwrap();
    }
    Err(e) => {
      bot.send_message(msg.chat.id, format!("Error: {}", e)).await.unwrap();
    }
  }
}

pub fn command(user_states: Arc<Mutex<HashMap<i64, UserState>>>) -> Command {
  Command {
    description: "Track a wallet for trasactions, balances, and more",
    usage: "/track\\_wallet \\[network\\] \\[wallet address\\]",
    handler: Arc::new({
      let user_states = user_states.clone();
      move |bot, msg, parts| {
        let user_states = user_states.clone();
        Box::pin(handle(bot, msg, parts, user_states))
      }
    }),
  }
}
