#![warn(missing_docs)]
//! divorce
//!
//! tmux extension
use std::{process::exit};
use args::arg_parse;

mod args;
mod tmux;
mod git;
mod fzf;

fn main() {
    let arguments = arg_parse().get_matches();
    

    if !check_command() {
        println!("missing program");
        exit(1);
    }
    match arguments.subcommand() {
        Some(("fzf",_sub_matches)) => switch_with_fzf(),
        Some(("new",sub_matches)) => new_session(sub_matches.get_one::<String>("name").map(|s|s.as_str())),
        _ => unreachable!()

    }
    // let command = arguments[1].to_lowercase();
    
    // match command.as_str() {
    //     "fzf" => switch_with_fzf(),
    //     "new" => {
    //         if arguments.len() >= 3 {
    //             new_session(Some(arguments[2].clone()));
    //         } else {
    //             new_session(None);
    //         }
    //     },
    //     _ => println!("idk mang")
    // }
}
fn check_command() -> bool{
    let _has_git: bool = git::has_git();
    let _has_fzf: bool = fzf::has_fzf();
    let _has_tmux: bool = tmux::has_tmux();
    if !_has_fzf || !_has_git || !_has_tmux {
        return false;
    }
    return true
}

fn switch_with_fzf() {
    let open_sessions = tmux::list_sessions();
    let mut current_session = String::new();
    if open_sessions.len() <= 0 {
        return
    }
    let is_in_tmux = tmux::is_in_tmux();
    if is_in_tmux {
        current_session = tmux::get_current_session(is_in_tmux);
    }
    if current_session != "" {
        let sessions: Vec<String> = open_sessions.iter().filter(|s| **s != current_session).cloned().collect();
        let selected = fzf::fzf_sessions(sessions);
        tmux::switch_session(&selected);
        return
    }
    let selected = fzf::fzf_sessions(open_sessions);
    tmux::switch_session(&selected);
    return
}
fn new_session(new_session_name: Option<&str>) {
    


}