use std::env::args;

pub enum SizeSettings {
    Big,
    Small,
}

pub fn get_size_settings() -> SizeSettings {
    // for arg in args() {
    // println!("{}", arg);
    // }

    let mut result = SizeSettings::Small;

    match args().nth(1) {
        Some(value) => {
            if value == "big" {
                result = SizeSettings::Big;
            }
        },

        _ => {}
    };
    

    result
}
