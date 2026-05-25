use macroquad::color::Color;
use macroquad::math::Vec2;
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

/// Struct to hold both horizontal and vertical alignment settings.
pub struct Alignment {
    pub x: AlignX,
    pub y: AlignY,
}

/// Internal struct to represent a single line of text, along with its offset from the main position.
struct Line {
    text: String,
    x_offset: f32,
    y_offset: f32,
}

pub struct Text {
    pos: Vec2,
    width: f32,
    text: String,
    font: Font,
    alignment: Alignment,
    size: u16,
    color: Color,
    lines: Vec<Line>,
}

/// Impl for text fields
impl Text {
    pub fn new(pos: Vec2, width: f32, text: String, font: Font, alignment: Alignment, size: u16, color: Color) -> Text {
        let mut t = Text {
            pos,
            width,
            text,
            font,
            alignment,
            size,
            color,
            lines: Vec::new(),
        };
        t.update_all();
        t
    }

    pub fn draw(&self) {
        for line in &self.lines {
            draw_text_ex(
                &line.text,
                self.pos.x + line.x_offset,
                self.pos.y + line.y_offset,
                TextParams {
                    font: Some(&self.font),
                    font_size: self.size,
                    color: self.color,
                    ..Default::default()
                },
            );
        }
    }

    /// Changes the top left **position** of the text field
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    /// Changes the **width** of the text field.
    /// Negative values will make the text expand to the **left**!
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
        self.update_all();
    }

    /// Changes the **text** of the text field and triggers an internal **recalculation** of the text arrangement.
    /// Only **plain text strings** are currently supported.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.update_all();
    }

    /// Changes the **font** of the text and triggers an internal **recalculation** of the text arrangement.
    pub fn set_font(&mut self, font: Font) {
        self.font = font;
        self.update_all();
    }

    /// Changes the **alignment** of the text in the field and triggers an internal **recalculation** of the text alignment.
    pub fn set_alignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
        self.update_alignment();
    }

    /// Changes the **size** of the text and triggers an internal **recalculation** of the text arrangement.
    pub fn set_size(&mut self, size: u16) {
        self.size = size;
        self.update_all();
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Triggers a **recalculation** of the text arrangement.
    fn update_all(&mut self) {
        self.lines.clear();
        let mut raw_str = self.text.as_str();
        let mut dimensions = measure_text(raw_str, Some(&self.font), self.size, 1.0);
        while dimensions.width > self.width.abs() {
            // TODO: Panic origin - The logic below uses byte-based slicing. If a split point or index falls inside a multi-byte UTF-8 character (e.g. an emoji), the program will panic.
            let split_index = (raw_str.len() as f32 * (self.width.abs() / dimensions.width)) as usize;
            let i = raw_str[..split_index.min(raw_str.len())].rfind(' ').unwrap_or(1); //TODO: understand this
            let add: &str;
            if i == 0 {
                (add, raw_str) = raw_str.split_at((raw_str.len() as f32 * self.width.abs() / dimensions.width) as usize);
            } else {
                (add, raw_str) = raw_str.split_at(i);
                raw_str = raw_str.trim_start(); // Remove the leading space for the next line
            }
            self.lines.push(Line {
                text: add.to_string(),
                x_offset: 0.0,
                y_offset: 0.0,
            });
            dimensions = measure_text(raw_str, Some(&self.font), self.size, 1.0);
        }
        self.lines.push(Line {
            text: raw_str.to_string(),
            x_offset: 0.0,
            y_offset: 0.0,
        });
        self.update_alignment();
    }

    /// Triggers an **recalculation** of the text alignment.
    fn update_alignment(&mut self) {
        let lines_len = self.lines.len();
        for (i, line) in self.lines.iter_mut().enumerate() {
            let dimensions = measure_text(&line.text, Some(&self.font), self.size, 1.0);
            line.x_offset = match self.alignment.x {
                AlignX::Center => -dimensions.width / 2.0 + self.width.min(0.0),
                AlignX::Right => -dimensions.width + self.width.min(0.0),
                AlignX::Left => self.width.min(0.0),
            };
            line.y_offset = match self.alignment.y {
                AlignY::Center => self.size as f32 * (i as f32 - lines_len as f32 / 2.0 + 0.8),
                AlignY::Top => self.size as f32 * (i as f32 + 0.8),
                AlignY::Bottom => self.size as f32 * (i as f32 - lines_len as f32 + 0.8),
            };
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use macroquad::prelude::*;

    #[test]
    fn test_text_wrapping_and_drawing() {
        macroquad::Window::new("Integration Test", async {
            let content = "A quick brown fox jumps.".to_string();
            let mut w: f32 = 100.0;
            let size = 70;

            let mut text_center = Text::new(
                vec2(100.0, 100.0),
                w,
                content.clone(),
                load_ttf_font("JetBrainsMono-VariableFont_wght.ttf").await.unwrap(),
                Alignment { x: AlignX::Left, y: AlignY::Center },
                size,
                WHITE,
            );
            let mut text_top = Text::new(
                vec2(100.0, 300.0),
                w,
                content.clone(),
                load_ttf_font("JetBrainsMono-VariableFont_wght.ttf").await.unwrap(),
                Alignment { x: AlignX::Left, y: AlignY::Top },
                size,
                WHITE,
            );
            let mut text_bottom = Text::new(
                vec2(100.0, 500.0),
                w,
                content.clone(),
                load_ttf_font("JetBrainsMono-VariableFont_wght.ttf").await.unwrap(),
                Alignment { x: AlignX::Left, y: AlignY::Bottom },
                size,
                WHITE,
            );
            loop {
                clear_background(BLACK);
                if is_mouse_button_pressed(MouseButton::Left) {
                    w = mouse_position().0 - 100.0;
                    text_center.set_width(w);
                    text_top.set_width(w);
                    text_bottom.set_width(w);
                }
                draw_rectangle(100.0, 100.0, w, 150.0, RED);
                draw_rectangle(100.0, 300.0, w, 150.0, RED);
                draw_rectangle(100.0, 500.0, w, 150.0, RED);
                text_center.draw();
                text_top.draw();
                text_bottom.draw();
                next_frame().await;
            }
        });
    }
}
