use clap::{arg, Command};

pub fn arg_parse() -> Command {
  Command::new("divorce")
    .about("Tmux session management tool written in rust")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .allow_external_subcommands(true)
    .subcommand(Command::new("fzf")
      .about("fuzzy find through open tmux sessions")
    ).subcommand(Command::new("new")
      .about("Create a new tmux session. A name can be provided, if not it will default to the current folders name and git branch if available")
      .arg(arg!(name: [NAME]))
    )
}