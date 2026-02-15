use std::process::{Command};


pub fn list_sessions() -> Vec<String> {
  let mut open_sessions: Vec<String> = Vec::<String>::new();
  let list_sessions_output = Command::new("tmux").arg("list-session").output();
  match list_sessions_output {
    Ok(val) => {
      let sessions = String::from_utf8_lossy(&val.stdout).to_string();
      let mut sessions_as_collection: Vec<String> = sessions.lines().filter_map(|line| line.split(":").next())
      .map(|session| session.to_string()).collect();
      open_sessions.append(&mut sessions_as_collection);
    }
    Err(_e) => return open_sessions,
  }
  return open_sessions
}