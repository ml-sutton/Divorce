#![warn(missing_docs)]
//! divorce
//!
//! tmux extension
use std::{env, process::exit};

use crate::{fzf::fzf_session::fzf_sessions, tmux::in_tmux::{self, is_in_tmux}};

mod tmux;
mod git_cli;
mod fzf;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        println!("help");
        exit(1);
    }
    if !check_command() {
        println!("missing program");
        exit(1);
    }
    let command = arguments[1].to_lowercase();
    
    match command.as_str() {
        "fzf" => switch_with_fzf(),
        "new" => println!("new session"),
        _ => println!("idk mang")
    }
}
fn check_command() -> bool{
    let _has_git: bool = git_cli::has_git::has_git();
    let _has_fzf: bool = fzf::has_fzf::has_fzf();
    let _has_tmux: bool = tmux::has_tmux::has_tmux();
    if !_has_fzf || !_has_git || !_has_tmux {
        return false;
    }
    return true
}

fn switch_with_fzf() {
    let open_sessions = tmux::list_sessions::list_sessions();
    let mut current_session = String::new();
    if open_sessions.len() <= 0 {
        return
    }
    let is_in_tmux = tmux::in_tmux::is_in_tmux();
    if is_in_tmux {
        current_session = tmux::get_current_session::get_current_session(is_in_tmux);
    }
    if current_session != "" {
        let sessions: Vec<String> = open_sessions.iter().filter(|s| **s == current_session).cloned().collect();
        let selected = fzf_sessions(sessions);
        println!("{}",selected);
        return
    }
    let selected = fzf_sessions(open_sessions);
    println!("{}", selected)

}