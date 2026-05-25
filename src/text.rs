use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::{TextParams, draw_text_ex, measure_text};
use macroquad::text::Font;

/// Self-explanatory enums for horizontal text alignment.
pub enum AlignX {
    Left,
    Center,
    Right,
}

/// Self-explanatory enums for vertical text alignment.
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

/// Struct for all properties of a text field
pub struct Text {
    pos: Vec2,
    max_w: f32,
    text: String,
    font: Font,
    alignment: Alignment,
    size: u16,
    color: Color,
    lines: Vec<Line>,
}

/// Impl for text fields
impl Text {
    /// Function to create a new text field
    pub fn new(pos: Vec2, max_w: f32, text: String, font: Font, alignment: Alignment, size: u16, color: Color) -> Text {
        let mut t = Text {
            pos,
            max_w,
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

    /// Draws the text with the current settings
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

    /// Changes the top left position of the text field
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    /// Changes the width of the text field
    pub fn set_max_w(&mut self, max_w: f32) {
        self.max_w = max_w.max(0.0);
        self.update_all();
    }

    /// Changes the text of the text field.
    /// Only plain text strings are currently supported.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.update_all();
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
        self.update_all();
    }

    pub fn set_align_x(&mut self, align_x: AlignX) {
        self.alignment.x = align_x;
        self.update_alignment();
    }

    pub fn set_align_y(&mut self, align_y: AlignY) {
        self.alignment.y = align_y;
        self.update_alignment();
    }

    pub fn set_size(&mut self, size: u16) {
        self.size = size;
        self.update_all();
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn update_all(&mut self) {
        self.lines.clear();
        let mut raw_str = self.text.as_str();
        let mut dimensions = measure_text(raw_str, Some(&self.font), self.size, 1.0);
        while dimensions.width > self.max_w {
            //TODO: test
            let mut i = (raw_str.len() as f32 * dimensions.width / self.max_w) as usize;
            while raw_str.chars().nth(i) != Option::from(' ') && i != 0 {
                i -= 1;
            }
            let add: &str;
            if i == 0 {
                (add, raw_str) = raw_str.split_at((raw_str.len() as f32 * dimensions.width / self.max_w) as usize);
            } else {
                (add, raw_str) = raw_str.split_at(i);
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

    fn update_alignment(&mut self) {
        let lines_len = self.lines.len();
        for (i, line) in self.lines.iter_mut().enumerate() {
            let dimensions = measure_text(&line.text, Some(&self.font), self.size, 1.0);
            line.x_offset = match self.alignment.x {
                AlignX::Center => -dimensions.width / 2.0,
                AlignX::Right => -dimensions.width,
                AlignX::Left => 0.0,
            };
            line.y_offset = match self.alignment.y {
                //TODO: don't align based of the baseline
                AlignY::Center => self.size as f32 * (i as f32 - lines_len as f32 / 2.0 + 0.5),
                AlignY::Top => self.size as f32 * (i + 1) as f32,
                AlignY::Bottom => self.size as f32 * (i as f32 - lines_len as f32 + 1.0),
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
        let pos = vec2(100.0, 100.0);
        let content = "A quick brown fox jumps over the lazy dog.".to_string();
            let mut w: f32 = 100.0;
        
        let mut text = Text::new(
            pos,
            w,
            content,
            load_ttf_font("JetBrainsMono-VariableFont_wght.ttf").await.unwrap(),
                Alignment { x: AlignX::Left, y: AlignY::Bottom },
            20,
            WHITE,
        );
        loop {
            clear_background(BLACK);
                if is_mouse_button_pressed(MouseButton::Left) {
                    w = mouse_position().0 - pos.x;
                text.set_max_w(w);
            }
                draw_rectangle(pos.x, pos.y, w, screen_height() - pos.y, RED);
            text.draw();
            next_frame().await;
        }
    });
}
}
