pub mod track_wallet;
pub mod help;
pub mod increment_counter;
pub mod types;

use std::collections::HashMap;
pub use types::{Command, CommandHandler, UserState};

pub use increment_counter::command as increment_counter_command;
pub use track_wallet::command as track_wallet_command;
pub use help::command as help_command;

use std::sync::Arc;
use tokio::sync::Mutex;

pub fn get_commands() -> HashMap<&'static str, Command> {
  let mut cmds = HashMap::new();

  cmds.insert("/increment_counter", increment_counter_command());
  cmds.insert("/track_wallet", track_wallet_command(std::sync::Arc::new(tokio::sync::Mutex::new(HashMap::new()))));
  cmds.insert("/help", help_command());

  cmds
}
