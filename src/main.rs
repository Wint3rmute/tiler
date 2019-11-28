mod wmctrl;
mod wmctrl_window;
use std::process::Command;
use std::vec::Vec;
use std::mem::swap;

static SCREEN_HEIGHT: i16 = 1080;
static SCREEN_WIDTH: i16 = 1920;

macro_rules! hp {
    ($height:expr) => {
        (SCREEN_HEIGHT as f32 * $height as f32) as i16;
    };
}

macro_rules! wp {
    ($width:expr) => {
        (SCREEN_WIDTH as f32 * $width as f32) as i16;
    };
}

fn get_windows_on_current_workspace() -> Vec<wmctrl_window::WmctrlWindow> {
    let current_workspace = wmctrl::raw_get_current_workspace();

    let mut result = Vec::<wmctrl_window::WmctrlWindow>::new();

    let windows_list_raw = wmctrl::raw_get_windows_list();

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

impl wmctrl_window::WmctrlWindow {
    fn get_field(&self) -> i32 {
        return self.width as i32 * self.height as i32;
    }

    fn place_center(&mut self, width_percent: f32, height_percent: f32) {
        self.set_center_horizontally(width_percent);
        self.set_center_vertically(height_percent);
        self.apply_position();
    }

    fn set_center_horizontally(&mut self, width_percent: f32) {
        self.left = wp!((1.0 - width_percent) / 2.0);
        self.width = wp!(width_percent);
    }

    fn set_center_vertically(&mut self, height_percent: f32) {
        self.top = hp!((1.0 - height_percent) / 2.0);
        self.height = hp!(height_percent);
    }
}

fn layout_1_window(window: &mut wmctrl_window::WmctrlWindow) {
    // let field = window.get_field();
    println!("{}", window.width);

    if window.width > 1200 {
        println!("{}", 0.6);
        window.place_center(0.5, 0.6);
    } else {
        println!("{}", 0.9);
        window.place_center(0.95, 0.93);
    }
}

fn layout_2_windows(windows: &mut Vec<wmctrl_window::WmctrlWindow>) {
    // let mut 

    let mut window_left = windows.pop().unwrap();
    let mut window_right = windows.pop().unwrap();

    if window_left.left < window_right.left {
        swap(&mut window_left, &mut window_right);
    }


    let width_perc = 0.42;
    let height_perc = 0.54;

    window_left.set_center_vertically(height_perc);
    window_right.set_center_vertically(height_perc);

    let left_margin = (1.0 - width_perc*2.0)/3.0;

    window_left.left = wp!(left_margin);
    window_right.left = wp!(left_margin * 2.0 + width_perc);

    window_left.width = wp!(width_perc);
    window_right.width = wp!(width_perc);

    window_left.apply_position();
    window_right.apply_position(); 
}

fn layout_3_windows(window: &Vec<wmctrl_window::WmctrlWindow>) {}

fn main() {
    let mut windows = get_windows_on_current_workspace();
    let num_of_windows = windows.len();

    if num_of_windows == 1 {
        layout_1_window(&mut windows[0]);
    } else if num_of_windows == 2 {
        layout_2_windows(&mut windows);
    } else if num_of_windows == 3 {
        layout_3_windows(&mut windows);
    }
}
