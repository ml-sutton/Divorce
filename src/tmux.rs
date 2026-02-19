use std::env;
use std::process::Command;
use std::os::unix::process::CommandExt;


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

pub fn has_tmux() -> bool {
  let tmux_output = Command::new("tmux").arg("-V").output();
  match tmux_output {
    Ok(_val) => return true,
    Err(_e) => return false,
  };
}

pub fn is_in_tmux() -> bool {
  match env::var("TMUX") {
      Ok(_val) => return true,
      Err(_e) => return false,
  }
}
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

pub fn new_session(session_name: String){
  let _ = Command::new("tmux").args(["new-session","-A","-s",session_name.as_str()]).exec();

}
pub fn switch_session(session_name: &String) {
  let stripped_session_name = match session_name.strip_suffix("\n") {
      Some(_session_name) => _session_name.to_string(),
      None => String::from("").to_string(),
  };
  let err = Command::new("tmux").arg("switch-client").arg("-t").arg(stripped_session_name.as_str()).exec();
  println!("{}", err);
}