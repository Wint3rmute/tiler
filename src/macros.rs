/*
    TODO: Read about macro hygiene.

    Can those variables be moved to
    some another module? They are
    constant values after all.
*/

#[macro_export]
macro_rules! hp {
    ($height:expr) => {
        (1080 as f32 * $height as f32) as i16;
    };
}

#[macro_export]
macro_rules! wp {
    ($width:expr) => {
        (1920 as f32 * $width as f32) as i16;
    };
}
