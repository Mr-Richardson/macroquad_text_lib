use macroquad::color::Color;
use macroquad::prelude::measure_text;
use macroquad::text::{draw_text, Font};

pub enum AlignX {
    Left,
    Center,
    Right,
}

pub enum AlignY {
    Top,
    Center,
    Bottom,
}

pub struct Text {
    font: Font,
    align_x: AlignX,
    align_y: AlignY,
    size: u16,
    color: Color,
}

impl Text {
    fn new(font: Font, align_x: AlignX, align_y: AlignY, size: u16, color: Color) -> Text {
        Text {
            font,
            align_x,
            align_y,
            size,
            color,
        }
    }

    pub fn draw(&self, text: &str, x: f32, y: f32, max_w: f32) {
        let dimensions = measure_text(text, Some(&self.font), self.size, 1.0);
        match self.align_x {
            AlignX::Center => x -= dimensions.width / 2.0,
            AlignX::Right => x -= dimensions.width,
            _ => (),
        }
        match self.align_y {
            AlignY::Center => y -= dimensions.offset_y / 2.0,
            AlignY::Top => y -= dimensions.offset_y,
            _ => (),
        }
        draw_text(text, x, y, self.size as f32, self.color);
    }
}
