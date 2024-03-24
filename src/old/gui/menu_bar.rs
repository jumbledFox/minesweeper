use ggez::glam::Vec2;

use super::{
    button::{self, LabeledButton},
    dropdown, Dropdown, TextRenderer,
};

pub struct MenuItem {
    pub button: LabeledButton,
    pub dropdown: Dropdown,
}
pub struct MenuBar {
    pub items: Vec<MenuItem>,
    pub hovering_over: bool,
    pub current_item: Option<usize>,
    pub height: f32,
}

impl MenuBar {
    pub fn new(tr: &TextRenderer, item_names: Vec<(String, f32, Vec<Option<String>>)>) -> MenuBar {
        let padding = (1.0, 1.0, 2.0, 2.0);
        // Make the height of each button be that of the tallest button
        let height = item_names
            .iter()
            .map(|(s, ..)| tr.text_size_padded(&s, padding).y)
            .fold(f32::NEG_INFINITY, f32::max);
        // Generate a vector of all of the buttons and position them
        let mut items: Vec<MenuItem> = Vec::with_capacity(item_names.len());
        let mut x_pos = 0.0;
        for (s, min_w, d) in item_names {
            let mut button = LabeledButton::new(
                tr,
                s,
                Some(padding),
                Vec2::new(x_pos, 1.0),
                button::PressMode::Immediate,
                false,
            );
            button.b.rect.h = height;
            x_pos += button.b.rect.w;
            let dropdown = Dropdown::new(
                tr,
                d,
                Vec2::new(button.b.rect.x, button.b.rect.y + button.b.rect.h),
                min_w,
            );
            items.push(MenuItem { button, dropdown });
        }
        MenuBar {
            items,
            hovering_over: false,
            current_item: None,
            height,
        }
    }

    pub fn menu_button_pressed(&mut self, menu_index: usize, dropdown_index: usize) -> bool {
        if let Some(menu_item) = self.items.get_mut(menu_index) {
            if let Some(dropdown_item) = menu_item.dropdown.items.get_mut(dropdown_index) {
                if let dropdown::DropdownItem::Button(b) = dropdown_item {
                    return b.pressed();
                }
            }
        }
        false
    }

    // TODO: This function isn't very good.
    // Update each of the buttons in the menu bar, returns if something on the menu was clicked
    pub fn update(&mut self, mouse_pos: Vec2, mouse_mode: super::MousePressMode) -> bool {
        self.hovering_over = false;

        // Update the dropdown
        if let Some(current_item_index) = self.current_item {
            // Hide the menus if we've clicked elsewhere
            if mouse_mode == super::MousePressMode::Down {
                if self.items[current_item_index].button.b.state == super::button::State::Idle
                    && !self.items[current_item_index]
                        .dropdown
                        .mouse_over(mouse_pos)
                {
                    self.current_item = None;
                }
            }
            // Update the menu & hide the menus when a dropdown item is pressed
            if self.items[current_item_index]
                .dropdown
                .update(mouse_pos, mouse_mode)
            {
                self.current_item = None;
                return true;
            }
            if self.items[current_item_index]
                .dropdown
                .mouse_over(mouse_pos)
            {
                self.hovering_over = true;
            }
        }

        // Update the menu items
        let mut menu_button_pressed = false;
        // Update all of the buttons
        for (i, menu_item) in &mut self.items.iter_mut().enumerate() {
            menu_item.button.update(mouse_pos, mouse_mode);
            // If a menu item's been pressed, make it the current one, unless it's already the current one in which case make the current one None
            if menu_item.button.pressed() {
                menu_button_pressed = true;
                self.hovering_over = true;
                // If we're already on this menu item, clicking it should close it
                if self.current_item.is_some_and(|c| c == i) {
                    self.current_item = None;
                } else {
                    self.current_item = Some(i);
                    menu_item.dropdown.init();
                }
                continue;
            }
            match menu_item.button.b.state {
                button::State::Hovered | button::State::Depressed => {
                    self.hovering_over = true;
                }
                _ => (),
            }
            // If the current one isn't this button but we're hovering over it, make it the current one,
            // maybe TODO, make hovering over the menu 'swallow' the hovering over a button
            if self.current_item.is_some_and(|c| c != i)
                && menu_item.button.b.state == button::State::Hovered
            {
                self.hovering_over = true;
                self.current_item = Some(i);
                menu_item.dropdown.init();
            }
        }
        menu_button_pressed
    }
}
