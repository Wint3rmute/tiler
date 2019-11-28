use std::process::Command;


pub fn run_command(mut command: &Command) -> String {
    let output = command.output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}