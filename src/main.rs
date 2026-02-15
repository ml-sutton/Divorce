#![warn(missing_docs)]
//! divorce
//!
//! tmux extension
use std::{env, process::exit};

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
    
    let open_sessions = tmux::list_sessions::list_sessions();
    match command.as_str() {
        "fzf" => println!("fuzzy finding {:?}", open_sessions),
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