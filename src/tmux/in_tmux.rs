use std::env;

pub fn is_in_tmux() -> bool {
  match env::var("TMUX") {
      Ok(_val) => return true,
      Err(_e) => return false,
  }
}