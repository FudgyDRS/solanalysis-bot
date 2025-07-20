use teloxide::prelude::*;
use teloxide::types::Message;
use std::sync::Arc;
use crate::handlers::{get_commands, Command};

pub async fn handle(bot: Bot, msg: Message, _parts: &mut std::str::SplitWhitespace<'_>) {
  let cmds = get_commands();

  let mut help_text = String::from("*Available commands:*\n");

  for (name, cmd) in cmds.iter() {
    use std::fmt::Write;
    writeln!(
      &mut help_text,
      "`{}`\n _Usage_: {}\n _Description_: {}\n",
      name, cmd.usage, cmd.description
    ).unwrap();
  }

  bot.send_message(msg.chat.id, help_text)
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await
    .unwrap();
}

pub fn command() -> Command {
  Command {
    description: "List available commands",
    usage: "/help",
    handler: Arc::new(|bot, msg, parts| Box::pin(handle(bot, msg, parts))),
  }
}
