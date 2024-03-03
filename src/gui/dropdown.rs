use ggez::{glam::Vec2, graphics::{self, Canvas, Color, DrawParam, InstanceArray, Rect}};

use super::{button::{self, LabeledButton, MouseMode}, draw_nineslice, TextRenderer};

pub struct Dropdown {
    pub items: Vec<LabeledButton>,
    pub rect: Rect,
}

impl Dropdown {
    pub fn new(tr: &TextRenderer, item_names: Vec<String>, pos: Vec2, min_width: f32) -> Dropdown {
        let padding = (1.0, 1.0, 2.0, 2.0);
        // Make the width of the menu be the larger of the minimum width or the thickest button
        let width = min_width.max(item_names.iter().map(|s| tr.text_size(&s).x + padding.2 + padding.3).fold(f32::NEG_INFINITY, f32::max));
        
        // Generate a vector of all of the buttons and position them
        let mut items: Vec<LabeledButton> = Vec::with_capacity(item_names.len());
        let mut y_pos = 1.0;
        for s in item_names {
            let mut i = LabeledButton::new(tr, s.clone(), Some(padding), Vec2::new(1.0, y_pos) + pos, button::PressMode::Immediate, false);
            i.b.rect.w = width;
            y_pos += i.b.rect.h;
            items.push(i);
        }
        Dropdown { items, rect: Rect::new(pos.x, pos.y, width+2.0, y_pos + 1.0) }
    }

    pub fn update(&mut self, mouse_pos: Vec2, mouse_mode: MouseMode) -> bool {
        let mut button_was_pressed = false;
        for button in &mut self.items.iter_mut() {
            button.update(mouse_pos, mouse_mode);
            if button.b.pressed { button_was_pressed = true; }
        }
        button_was_pressed
    }

    pub fn render(&self, canvas: &mut Canvas, tr: &mut TextRenderer, spritesheet: &mut InstanceArray) {
        draw_nineslice(canvas, spritesheet, Rect::new(55.0, 39.0, 3.0, 3.0), 1.0, self.rect);
        for button in self.items.iter() {
            let (bg_col, text_col) = match button.b.state {
                super::button::State::Idle => (Color::from_rgb(192, 203, 220), Color::from_rgb(24, 20, 37)),
                _  => (Color::from_rgb(38, 43, 68), Color::from_rgb(255, 255, 255)),
            };
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(button.b.rect.point())
                    .scale(button.b.rect.size())
                    .color(bg_col),
            );
            tr.draw_text(canvas, &button.label, DrawParam::new().color(text_col).dest(button.text_pos()));
        }
    }

    pub fn mouse_over(&self, mouse_pos: Vec2) -> bool {
        self.rect.contains(mouse_pos)
    }
}