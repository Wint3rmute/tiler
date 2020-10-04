use crate::size_settings::SizeSettings;
use crate::wmctrl_window;
use crate::{hp, wp};
use std::mem::swap;

// Extra positioning helpers
impl wmctrl_window::WmctrlWindow {
    // Maybe use it for some size-specific rules some day?
    /*
    fn get_field(&self) -> i32 {
        return self.width as i32 * self.height as i32;
    }
    */

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

pub fn layout_1_window(window: &mut wmctrl_window::WmctrlWindow) {
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

pub fn layout_2_windows(windows: &mut Vec<wmctrl_window::WmctrlWindow>, settings: SizeSettings) {
    let mut window_left = windows.pop().unwrap();
    let mut window_right = windows.pop().unwrap();

    if window_left.left < window_right.left {
        swap(&mut window_left, &mut window_right);
    }

    let (width_perc, height_perc) = match settings {
        SizeSettings::BigGaps => (0.42, 0.60),
        _ => (0.455, 0.90),
    };

    window_left.set_center_vertically(height_perc);
    window_right.set_center_vertically(height_perc);

    let left_margin = (1.0 - width_perc * 2.0) / 3.0;

    window_left.left = wp!(left_margin);
    window_right.left = wp!(left_margin * 2.0 + width_perc);

    window_left.width = wp!(width_perc);
    window_right.width = wp!(width_perc);

    window_left.apply_position();
    window_right.apply_position();
}

pub fn layout_3_windows(windows: &mut Vec<wmctrl_window::WmctrlWindow>, settings: SizeSettings) {
    windows.sort_by(|a, b| b.left.cmp(&a.left));

    let mut left_window = windows.pop().unwrap();

    windows.sort_by(|a, b| b.top.cmp(&a.top)); // Pure laziness?

    let mut top_window = windows.pop().unwrap();

    let mut bottom_window = windows.pop().unwrap();

    // println!("{}", top_window.name );
    // println!("{}", bottom_window.name );

    // CONFIG

    /*
    let margin_horizontal_perc = 0.02;
    let margin_vertical_perc = 0.032;
    */

    let (margin_horizontal_perc, margin_vertical_perc) = match settings {
        SizeSettings::SmallGaps => (0.02, 0.032),
        _ => (0.06, 0.096),
    };
    // WINDOW CYCLING

    swap(&mut left_window, &mut bottom_window);
    swap(&mut left_window, &mut top_window);

    // LEFT WINDOW

    left_window.left = wp!(margin_horizontal_perc);
    left_window.width = wp!(0.5 - margin_horizontal_perc * 1.5);

    left_window.top = hp!(margin_vertical_perc);
    left_window.height = hp!(1.0 - 2.0 * margin_vertical_perc);

    // UPPER WINDOW

    top_window.left = wp!(0.5 + margin_horizontal_perc * 0.5);
    top_window.width = wp!(0.5 - margin_horizontal_perc * 1.5);

    top_window.top = hp!(margin_vertical_perc);
    top_window.height = hp!(0.5 - margin_vertical_perc * 1.5);

    // BOTTOM WINDOW
    bottom_window.left = wp!(0.5 + margin_horizontal_perc * 0.5);
    bottom_window.width = wp!(0.5 - margin_horizontal_perc * 1.5);

    bottom_window.top = hp!(0.5 + margin_vertical_perc * 0.5);
    bottom_window.height = hp!(0.5 - margin_vertical_perc * 1.5);

    left_window.apply_position();
    top_window.apply_position();
    bottom_window.apply_position();
}
