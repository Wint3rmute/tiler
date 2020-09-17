use std::env::args;

pub enum SizeSettings {
    BigGaps,
    SmallGaps,
}

pub fn get_size_settings() -> SizeSettings {

    let mut result = SizeSettings::BigGaps;

    match args().nth(1) {
        Some(value) => {
            if value == "small" {
                result = SizeSettings::SmallGaps;
            }
        },

        _ => {}
    };
    

    return result;
}
