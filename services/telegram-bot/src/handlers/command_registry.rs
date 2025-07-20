use std::collections::HashMap;
use std::str::SplitWhitespace;
use teloxide::prelude::*;
use teloxide::types::Message;
use std::future::Future;
use std::pin::Pin;

pub type CommandHandler = Box<
  dyn Fn(Bot, Message, &mut SplitWhitespace<'_>) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>
  + Send
  + Sync,
>;

pub struct Command {
  pub descriptionL &'static str,
  pub useage: &'static str,
  pub handler: CommandHandler,
}
