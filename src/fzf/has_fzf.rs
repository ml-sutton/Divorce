use std::process::Command;

pub fn has_fzf() -> bool {
  let fzf_output = Command::new("fzf").arg("--version").output();
  match fzf_output {
    Ok(_val) => return true,
    Err(_e) => return false,
  };
}