use std::fmt;
use std::process::Command;
use cmd_lib::{Process, CmdResult};

#[derive(Debug)]
pub struct WmctrlWindow {
    pub id: String,
    pub name: String,
    pub desktop_num: i8,
    pub top: i16,
    pub left: i16,
    pub width: i16,
    pub height: i16,
}

impl WmctrlWindow {
    pub fn new() -> WmctrlWindow {
        WmctrlWindow {
            id: "as".to_string(),
            name: "asda".to_string(),
            desktop_num: 3,
            top: 10,
            left: 11,
            width: 12,
            height: 13,
        }
    }

    pub fn from_string(line: String) -> Result<WmctrlWindow, ()> {
        
        let mut iterator =  line.split_whitespace();

        let id = iterator.next().unwrap().to_string();
        let desktop_num =  iterator.next().unwrap().parse::<i8>().unwrap();

        if desktop_num == -1 {
            return Err(());
        }

        let left =  iterator.next().unwrap().parse::<i16>().unwrap();
        let top =  iterator.next().unwrap().parse::<i16>().unwrap();
        let width =  iterator.next().unwrap().parse::<i16>().unwrap();
        let height =  iterator.next().unwrap().parse::<i16>().unwrap();

        iterator.next(); // Skip the hostname

        let name = iterator.collect::<Vec<&str>>().join(" ");

        Ok(WmctrlWindow{
            id,
            name,
            desktop_num,
            top,
            left,
            width,
            height
        })
    }

    pub fn apply_position(&self) {

        let position = [String::from("0"), self.left.to_string(),  self.top.to_string(), self.width.to_string(), self.height.to_string()].join(",");       
        let change_position_comand = ["wmctrl -i -r ", self.id.as_str(), " -e ", position.as_str()].join("");
        let unmaximize_command = ["wmctrl -ir ", self.id.as_str(), " -b remove,maximized_vert,maximized_horz"].join("");

        Process::new(change_position_comand).wait::<CmdResult>();//.unwrap();
        Process::new(unmaximize_command).wait::<CmdResult>();//.unwrap();

    /*
        let mut command = Command::new("wmctrl");
        command.arg("-vvv");
        
        command.arg(
            ["-r", self.id.as_str()].join(" ")
        );
        
        command.arg(
            ["-e", position.as_str()].join(" ")
        );
        
        let output = command.output().unwrap();
    */  

        // run_cmd!("wmctrl hello, {}", name)?;
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_normal_window() {
        let line = "0x01c00003  0 10   20   1900 1055 arch553ve Unit testing - Rust By Example - Mozilla Firefox".to_string();

        let window = WmctrlWindow::from_string(line).unwrap();

        assert_eq!(window.id, "0x01c00003");
        assert_eq!(window.left, 10);
        assert_eq!(window.top, 20);
        assert_eq!(window.width, 1900);

        assert_eq!(window.height, 1055);
        assert_eq!(window.desktop_num, 0);
        assert_eq!(window.name, "Unit testing - Rust By Example - Mozilla Firefox");

        //  assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ()")] // this annotation is cooool <3q
    fn test_ignored_window() {
        let line = "0x00800004 -1 1884 6    30   67         N/A tint2".to_string();

        let window = WmctrlWindow::from_string(line).unwrap();

    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
