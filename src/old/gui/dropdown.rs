use ggez::{glam::Vec2, graphics::Rect};

use super::{
    button::{self, LabeledButton},
    TextRenderer,
};

pub enum DropdownItem {
    Button(LabeledButton),
    Divider(f32),
}

// TODO: add checkboxes and shit to dropdowns, split them up into sections

pub struct Dropdown {
    pub items: Vec<DropdownItem>,
    pub rect: Rect,
}

impl Dropdown {
    // TODO: Maybe make defining dropdowns neater, instead of Option<String> some kind of enum for specifying creation.. or maybe "---" should turn into a divider...
    // This could be handy if I want more than just buttons and dividers in dropdowns.
    pub fn new(
        tr: &TextRenderer,
        item_names: Vec<Option<String>>,
        pos: Vec2,
        min_width: f32,
    ) -> Dropdown {
        let padding = (1.0, 1.0, 2.0, 2.0);
        // Make the width of the menu be the larger of the minimum width or the thickest button
        let width = min_width.max(
            item_names
                .iter()
                .flatten()
                .map(|s| tr.text_size_padded(&s, padding).x)
                .fold(f32::NEG_INFINITY, f32::max),
        );
        // Generate a vector of all of the buttons and position them
        let mut items: Vec<DropdownItem> = Vec::with_capacity(item_names.len());
        let mut y_pos = 1.0;
        for item in item_names {
            if let Some(s) = item {
                let mut i = LabeledButton::new(
                    tr,
                    s.clone(),
                    Some(padding),
                    Vec2::new(1.0, y_pos) + pos,
                    button::PressMode::Immediate,
                    false,
                );
                i.b.rect.w = width;
                y_pos += i.b.rect.h;
                items.push(DropdownItem::Button(i));
            } else {
                items.push(DropdownItem::Divider(y_pos));
                y_pos += 4.0;
            }
        }
        Dropdown {
            items,
            rect: Rect::new(pos.x, pos.y, width + 2.0, y_pos + 1.0),
        }
    }

    // Updates all of the buttons and returns if any were pressed
    pub fn update(&mut self, mouse_pos: Vec2, mouse_mode: super::MousePressMode) -> bool {
        let mut button_was_pressed = false;
        for dropdown_item in &mut self.items.iter_mut() {
            if let DropdownItem::Button(button) = dropdown_item {
                button.update(mouse_pos, mouse_mode);
                if button.b.pressed {
                    button_was_pressed = true;
                }
            }
        }
        button_was_pressed
    }

    // Resets all of the buttons for the dropdown
    pub fn init(&mut self) {
        for dropdown_item in &mut self.items.iter_mut() {
            if let DropdownItem::Button(button) = dropdown_item {
                button.b.pressed = false;
                if button.b.state != button::State::Disabled {
                    button.b.state = button::State::Idle;
                }
            }
        }
    }

    pub fn mouse_over(&self, mouse_pos: Vec2) -> bool {
        self.rect.contains(mouse_pos)
    }
}
