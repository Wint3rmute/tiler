use std::process::Command;
// mod wmctrl;

pub fn raw_get_current_workspace() -> i8 {
    let mut wmctrl_command = Command::new("wmctrl");
    wmctrl_command.arg("-d");

    let output_vec = wmctrl_command.output().unwrap();
    let output_str = String::from_utf8(output_vec.stdout).unwrap();

    let mut counter = 0;
    for line in output_str.lines() {
        if line.contains("*")
            { break; }

        counter += 1;
    }

    counter
}

pub fn raw_get_windows_list() -> String {
    let mut wmctrl_command = Command::new("wmctrl");
    wmctrl_command.arg("-lG");
    let output = wmctrl_command.output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}


