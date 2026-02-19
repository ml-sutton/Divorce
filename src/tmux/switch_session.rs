use std::process::Command;
use std::os::unix::process::{self, CommandExt};
pub fn switch_session(session_name: String) {
  let err = Command::new("tmux").arg("switch-client").arg("-t").arg(session_name.as_str()).exec();
  println!("{}", err)
}