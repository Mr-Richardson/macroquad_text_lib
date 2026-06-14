pub mod text_fit;
pub mod text_wrap;

#[derive(PartialEq)]
pub enum AlignX {
    Left,
    Center,
    Right,
}

#[derive(PartialEq)]
pub enum AlignY {
    Top,
    Center,
    Bottom,
}

/// Struct to hold both horizontal and vertical alignment settings.

#[derive(PartialEq)]
pub struct Alignment {
    pub x: AlignX,
    pub y: AlignY,
}
