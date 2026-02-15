#![warn(missing_docs)]
//! divorce
//!
//! tmux extension

mod tmux;
mod git_cli;
mod fzf;
fn main() {
    let _has_git: bool = git_cli::has_git::has_git();
    let _has_fzf: bool = fzf::has_fzf::has_fzf();
    let _has_tmux: bool = tmux::has_tmux::has_tmux();
    if !_has_fzf {
        println!("NO FZF");
    }
    if !_has_git {
        println!("NO GIT");
    }
    if !_has_tmux {
        println!("NO TMUX");
    }
    println!("Hello, world!");
}
