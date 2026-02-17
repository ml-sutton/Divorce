use std::process::{Command, Stdio};


pub fn fzf_sessions(sessions: Vec<String>) -> String {
  let sessions_output = sessions.join("\n");
  let mut echo_output = Command::new("echo").arg(sessions_output).stdout(Stdio::piped()).spawn().unwrap();
  let echo_stdin = echo_output.stdout.take().unwrap();
  let fzf_command = Command:: new("fzf").stdin(Stdio::from(echo_stdin)).stdout(Stdio::piped()).spawn().unwrap();
  let output = fzf_command.wait_with_output().unwrap();

  return String::from_utf8_lossy(&output.stdout).to_string()


}