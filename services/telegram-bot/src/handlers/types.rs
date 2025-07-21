use teloxide::prelude::*;
use std::str::SplitWhitespace;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type CommandHandler =  dyn for<'a> Fn(
    Bot, 
    Message, 
    &'a mut SplitWhitespace<'a>
  ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
  + Send
  + Sync;

#[derive(Clone)]
pub struct Command {
  pub description: &'static str,
  pub usage: &'static str,
  pub handler: Arc<CommandHandler>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum UserState {
  Idle,
  AwaitingNetwork { user_id: u64 },
  AwaitingWallet { user_id: u64, network: String },
}
