use std::process::Command;

pub fn get_current_session(in_tmux: bool) -> String { 
  if !in_tmux {
    return String::from("");
  }
  let current_name_output = Command::new("tmux").args(["display-message", "-p", "#S"]).output();
  match current_name_output {
    Ok(value) => return String::from_utf8_lossy(&value.stdout).strip_suffix("\n").map(|s| s.to_string()).unwrap_or_default(),
    Err(err) => return String::from(err.to_string()),
  }
}