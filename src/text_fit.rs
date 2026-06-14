use crate::{AlignX, AlignY, Alignment};
use macroquad::color::Color;
use macroquad::math::Vec2;
use macroquad::prelude::Font;
use macroquad::text::{TextParams, draw_text_ex, measure_text};

pub struct TextFit {
    pos: Vec2,
    width: f32,
    text: String,
    font: Font,
    alignment: Alignment,
    color: Color,
    text_size: u16,
    text_height: f32,
}

/// This text field scales the text to the desired width.
impl TextFit {
    pub fn new(pos: Vec2, width: f32, text: String, font: Font, alignment: Alignment, color: Color) -> Self {
        let mut text_fit = TextFit {
            pos,
            width,
            text,
            font,
            alignment,
            color,
            text_size: 0,
            text_height: 0.0,
        };
        text_fit.update_size();
        text_fit
    }

    pub fn draw(&self) {
        let pos_x: f32 = match self.alignment.x {
            AlignX::Left => self.pos.x,
            AlignX::Center => self.pos.x - self.width / 2.0,
            AlignX::Right => self.pos.x - self.width,
        };
        let pos_y: f32 = match self.alignment.y {
            AlignY::Bottom => self.pos.y,
            AlignY::Center => self.pos.y + self.text_height / 2.0,
            AlignY::Top => self.pos.y + self.text_height,
        };
        draw_text_ex(
            &self.text,
            pos_x,
            pos_y,
            TextParams {
                font: Option::from(&self.font),
                font_size: self.text_size,
                color: self.color,
                ..Default::default()
            },
        );
    }

    fn update_size(&mut self) {
        let text_dimensions = measure_text(self.text.as_str(), Option::from(&self.font), 1, 1.0);
        let size: f32 = self.width / text_dimensions.width;
        self.text_size = size as u16;
        self.text_height = text_dimensions.offset_y * size;
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn set_width(&mut self, width: f32) {
        if self.width != width {
            self.width = width;
            self.update_size();
        }
    }

    pub fn set_text(&mut self, text: String) {
        if self.text != text {
            self.text = text;
            self.update_size();
        }
    }

    pub fn set_font(&mut self, font: Font) {
        self.font = font;
        self.update_size();
    }

    pub fn set_alignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use macroquad::prelude::*;

    #[ignore = "Requires a graphical window, skip in CI"]
    #[test]
    fn test_text_fitting_and_drawing() {
        macroquad::Window::new("Integration Test", async {
            let content = "Jump!".to_string();
            let mut w: f32 = 100.0;

            let mut text_center = TextFit::new(vec2(200.0, 200.0), w, content.clone(), get_default_font(), Alignment { x: AlignX::Left, y: AlignY::Top }, WHITE);
            let mut text_top = TextFit::new(vec2(200.0, 200.0), w, content.clone(), get_default_font(), Alignment { x: AlignX::Center, y: AlignY::Center }, WHITE);
            let mut text_bottom = TextFit::new(vec2(200.0, 200.0), w, content.clone(), get_default_font(), Alignment { x: AlignX::Right, y: AlignY::Bottom }, WHITE);
            loop {
                clear_background(BLACK);
                if is_mouse_button_pressed(MouseButton::Left) {
                    w = mouse_position().0 - 200.0;
                    text_center.set_width(w);
                    text_top.set_width(w);
                    text_bottom.set_width(w);
                }
                draw_rectangle(100.0, 100.0, w * 3.0, w * 3.0, GRAY);
                draw_rectangle(200.0, 200.0, w, w, RED);
                text_center.draw();
                text_top.draw();
                text_bottom.draw();
                next_frame().await;
            }
        });
    }
}
