use std::process::Command;

pub fn has_git() -> bool{
  let fzf_output = Command::new("git").arg("-v").output();
  match fzf_output {
    Ok(_val) => return true,
    Err(_e) => return false,
  };
}