use std::process::Command;

pub fn get_git_details() -> String{
  let empty_str = String::new();
  let branch = Command::new("git").arg("branch").output();
  let project_dir_name = Command::new("git").args(["rev-parse","--show-toplevel"]).output();
  let branch_name = match branch {
    Ok(value) => match String::from_utf8_lossy(&value.stdout).to_string().strip_suffix("\n") {
        Some(val) => val.to_string(),
        None => empty_str,
    },
    Err(_) => empty_str,
  };
  let project_dir = match project_dir_name {
    Ok(value) => match String::from_utf8_lossy(&value.stdout).to_string().strip_suffix("\n") {
        Some(val) => match val.to_string().split("/").last() {
            Some(val) => val.to_string(),
            None => String::new(),
        },
        None => String::new(),
    },
    Err(_) => String::new(),
  };
  return format!("{}{}",project_dir,branch_name)
}