use teloxide::prelude::*;
use std::str::SplitWhitespace;
use crate::services::wallet_tracker;
use crate::handlers::types::{Command, CommandHandler};

pub asunc fn handle(bot: Bot, msg: Message, parts: &mut SplitWhitespace<'_>) {
  let network = parts.next().unwrap_or("solana");
  let wallet_address = parts.next().unwrap_or("unknown");

  match wallet_tracker::track_wallet(network, wallet_address).await {
    Ok(response) => {
      bot.send_message(msg.chat.id, response).await.unwrap();
    }
    Err(e) => {
      bot.send_message(msg.chat.id, format!("Error: {}", e)).await.unwrap();
    }
  }
}

pub fn command() -> Command {
  Command {
    description: "Track a wallet for trasactions, balances, and more",
    usage: "/track_wallet [network] [wallet address]",
    handler: Box::new(|bot, msg, parts| Box::pin(handle(bot, msg, parts))),
  }
}
