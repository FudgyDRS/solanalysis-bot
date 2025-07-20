use teloxide::prelude::*;
use std::str::SplitWhitespace;
use std::sync::Arc;
use crate::handlers::types::{Command, CommandHandler};
use crate::services;

pub async fn handle(bot: Bot, msg: Message, _parts: &mut SplitWhitespace<'_>) {
  match services::increment_counter::increment_counter().await {
    Ok(count) => {
      if let Err(e) = bot.send_message(msg.chat.id, format!("Counter is now: {}", count)).await {
        log::error!("Failed to send message: {}", e);
      }
    }
    Err(e) => {
      if let Err(e) = bot.send_message(msg.chat.id, format!("Error: {}", e)).await {
        log::error!("Failed to send message: {}", e);
      }
    }
  }
}

pub fn command() -> Command {
  Command {
    description: "Increment counter on database",
    usage: "/increment\\_counter",
    handler: Arc::new(|bot, msg, parts| Box::pin(handle(bot, msg, parts))),
  }
}
