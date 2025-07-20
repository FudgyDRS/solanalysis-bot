use teloxide::prelude::*;
use std::str::SplitWhitespace;
use std::future::Future;
use str::pin::Pin;

pub type CommandHandler = Box <
  dyn Fn(Bot, Message, &mut SplitWhitespace<'_>) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>
  + Send
  + Sync,
>;

pub struct Command {
  pub description: &'static str,
  pub usage: &'static str,
  pub handler: CommandHandler,
}
