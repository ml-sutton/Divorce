#![warn(missing_docs)]
//! divorce
//!
//! tmux extension
mod args;
mod tmux;
mod git;
mod fzf;

use git::{get_git_branch, has_git};
use std::process::exit;
use args::arg_parse;
use tmux::{new_session, has_tmux,attach_to_session,switch_session,is_in_tmux,list_sessions,get_current_session};
use fzf::{has_fzf,fzf_sessions};
use log::error;
#[derive(Debug, Clone)]
struct MissingProgramError {
    pub missing_program_vec : Vec<String>,
}   






fn main() {
    let has_programs = check_command();
    match has_programs {
        Ok(_) => print!(""),
        Err(missing_program) => {
            error!("{:#?}",missing_program);
            exit(1)
        }
    }
    let arguments = arg_parse().get_matches();
    match arguments.subcommand() {
        Some(("fzf",_sub_matches)) => switch_with_fzf(),
        Some(("new",sub_matches)) => create_new_session(sub_matches.get_one::<String>("name").map(|s|s.as_str())),
        _ => unreachable!()

    }
}
fn check_command() -> Result<bool, MissingProgramError>{
    let mut missing_programs: Vec<String> = vec!();
    let has_git = has_git();
    let has_fzf = has_fzf();
    let has_tmux = has_tmux();
    if !has_git {
        missing_programs.push(String::from("git"));
    }
    if !has_fzf {
        missing_programs.push(String::from("fzf"));
    }
    if !has_tmux {
        missing_programs.push(String::from("tmux"));
    }
    if missing_programs.len() != 0 {
        return Err(MissingProgramError { missing_program_vec: missing_programs })
    } else {
        return Ok(true)
    }

}   

fn switch_with_fzf() {
    let open_sessions = list_sessions();
    let mut current_session = String::new();
    if open_sessions.len() <= 0 {
        return
    }
    let is_in_tmux = is_in_tmux();
    if is_in_tmux {
        current_session = get_current_session(is_in_tmux);
    }
    if current_session != "" {
        let sessions: Vec<String> = open_sessions.iter().filter(|s| **s != current_session).cloned().collect();
        let selected = fzf_sessions(sessions);
        switch_session(&selected);
        return
    }
    let selected = fzf_sessions(open_sessions);
    attach_to_session(&selected);
    return
}
fn create_new_session(new_session_name: Option<&str>) {
    let session_name = match new_session_name {
        Some(name) => name,
        None => &generate_session_name()
    };
    new_session(session_name);
}

fn generate_session_name() -> String{
    let git_branch = get_git_branch();
    let current_file = String::new();
    let session_name = format!("{}{}",current_file,git_branch);
    return session_name;
}