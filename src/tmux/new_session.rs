use std::process::Command;
use std::os::unix::process::{self, CommandExt};

pub fn new_session(session_name: String){
  let _ = Command::new("tmux").args(["new-session","-A","-s",session_name.as_str()]).exec();

}