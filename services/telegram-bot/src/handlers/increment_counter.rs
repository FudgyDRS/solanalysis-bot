use teloxide::prelude::*;
use std::str::SplitWhitespace;
use crate::handlers::type::{Command, CommandHandler};
use crate::services;

pub async dn handle(bot: Bot, msg: Message, _pars: &mut SplitMessage<'_>) {
  match services::increment_counter::increment_counter().await {
    Ok(count) => {
      if let Err(e) = bot.send_message(msg.chat.id, format!("Counter is now: {}", count)).await {
        log::error!("Failed to send message: {}", e);
      }
    }
    Err(e) => {
      if let Err(e) = bot.send_message(chat.msg.id, format!("Error: {}", e)).await {
        log::error!("Failed to send message: {}", e);
      }
    }
  }
}

pub fn command() -> Command {
  Command {
    description: "Increment counter on database",
    usage: "/increment_counter",
    handler: Box::new(|bot, msg, parts| Box::pin(handle(bot, msg, parts))),
  }
}
