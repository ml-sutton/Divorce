

use std::process::Command;

pub fn has_git() -> bool{
  let fzf_output = Command::new("git").arg("-v").output();
  match fzf_output {
    Ok(_val) => return true,
    Err(_e) => return false,
  };
}

pub fn get_git_branch() -> String{
  let empty_str = String::new();
  let branch = Command::new("git").arg("branch").output();
  let branch_name = match branch {
    Ok(value) => match String::from_utf8_lossy(&value.stdout).to_string().strip_suffix("\n") {
        Some(val) => val.to_string(),
        None => empty_str,
    },
    Err(_) => empty_str,
  };
  
  return branch_name
}