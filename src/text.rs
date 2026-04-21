use macroquad::color::Color;
use macroquad::prelude::{TextParams, draw_text_ex, measure_text};
use macroquad::text::Font;

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
    pub fn new(font: Font, align_x: AlignX, align_y: AlignY, size: u16, color: Color) -> Text {
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
        let mut lines: Vec<String> = Vec::new();
        if dimensions.width > max_w {
            // TODO
        } else {
            lines.push(text.to_string());
        }
        let lines_len = lines.len();
        for (i, str) in lines.iter().enumerate() {
            let dimensions = measure_text(str, Some(&self.font), self.size, 1.0);
            let render_x: f32 = match self.align_x {
                AlignX::Center => x - dimensions.width / 2.0,
                AlignX::Right => x - dimensions.width,
                AlignX::Left => x,
            };
            let render_y: f32 = match self.align_y {
                AlignY::Center => {
                    y + dimensions.offset_y * (i as f32 - lines_len as f32 / 2.0 + 0.5)
                }
                AlignY::Top => y + dimensions.offset_y * (i as f32 + 1.0),
                AlignY::Bottom => y + dimensions.offset_y * (i - lines_len + 1) as f32,
            };
            draw_text_ex(
                str,
                render_x,
                render_y,
                TextParams {
                    font: Some(&self.font),
                    font_size: self.size,
                    color: self.color,
                    ..Default::default()
                },
            );
        }
    }
}
