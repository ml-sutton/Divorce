use std::process::Command;


pub fn has_tmux() -> bool {
  let tmux_output = Command::new("tmux").arg("-V").output();
  match tmux_output {
    Ok(_val) => return true,
    Err(_e) => return false,
  };
}