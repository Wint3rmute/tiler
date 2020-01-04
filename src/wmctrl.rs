use std::process::Command;
use crate::wmctrl_window;

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



pub fn get_windows_on_current_workspace() -> Vec<wmctrl_window::WmctrlWindow> {
    let current_workspace = raw_get_current_workspace();

    let mut result = Vec::<wmctrl_window::WmctrlWindow>::new();

    let windows_list_raw = raw_get_windows_list();

    for window_line in windows_list_raw.lines() {
        let window = wmctrl_window::WmctrlWindow::from_string(window_line.to_owned());

        match window {
            Ok(window) => {
                if window.desktop_num == current_workspace {
                    result.push(window);
                }
            }
            Err(_) => {}
        };
    }

    result
}


