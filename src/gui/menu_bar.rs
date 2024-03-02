use ggez::{glam::Vec2, graphics::{self, Canvas, Color, DrawParam, Rect}};

use super::{button::{self, LabeledButton}, Button, Dropdown, TextRenderer};

pub struct MenuBar {
    // pub items: Vec<(String, Button, Dropdown)>,
    // pub items_: Vec<(String, Button)>,
    pub items: Vec<LabeledButton>,
    current_item: Option<usize>,
}

impl MenuBar {
    // n_item_names: Vec<(String, Vec<String>)>, 
    pub fn new(tr: &TextRenderer, item_names: Vec<String>) -> MenuBar {
        // Generate a vector of all of the buttons
        let mut items: Vec<LabeledButton> = item_names.iter()
            .map(|s| LabeledButton::new(tr, s.clone(), Some((1.0, 1.0, 2.0, 2.0)), Vec2::new(0.0, 0.0), button::PressMode::Immediate, false)).collect();
        // Position them all properly
        let mut x_pos = 0.0;
        for button in items.iter_mut() {
            button.b.rect.x = x_pos;
            x_pos += button.b.rect.w;
        }
        MenuBar { items, current_item: None }
    }
    // Update each of the buttons in the menu bar
    pub fn update(&mut self, mouse_pos: Vec2, mouse_mode: super::button::MouseMode) {
        for (i, button) in &mut self.items.iter_mut().enumerate() {
            button.update(mouse_pos, mouse_mode);
            match button.b.state {
                super::button::State::Depressed => { self.current_item = Some(i) }
                _ => {}
            }
        }

        if let Some(current_item) = self.current_item {
            // If the button or the menu isn't being hovered over, hide the menu!
            if self.items[current_item].b.state == super::button::State::Idle && true {
                self.current_item = None;
            }
        }
    }
    // Renders the menu bar as well as dropdowns
    pub fn render(&self, canvas: &mut Canvas, tr: &mut TextRenderer) {
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
            tr.draw_text(canvas, &button.label, DrawParam::new().color(text_col).dest(Vec2::new(button.b.rect.x + button.padding.2, button.b.rect.y + button.padding.0)));
        }

        if let Some(current_item) = self.current_item {
            tr.draw_text(canvas, &self.items[current_item].label, DrawParam::new().dest(Vec2::new(1.0, 10.0)));
        }
    }
}