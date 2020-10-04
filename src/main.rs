mod layouts;
mod macros;
mod size_settings;
mod wmctrl;
mod wmctrl_window;

use layouts::{layout_1_window, layout_2_windows, layout_3_windows};

fn main() {
    let mut windows = wmctrl::get_windows_on_current_workspace();
    let settings = size_settings::get_size_settings();
    
    match windows.len() {
        1 => layout_1_window(&mut windows[0]),
        2 => layout_2_windows(&mut windows, settings),
        3 => layout_3_windows(&mut windows, settings),
        _ => {}
    };
}
