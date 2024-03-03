use ggez::{glam::Vec2, graphics::{self, Canvas, Color, DrawParam, InstanceArray}};

use super::{button::{self, LabeledButton}, Dropdown, TextRenderer};

pub struct MenuItem {
    pub button: LabeledButton,
    pub dropdown: Dropdown,
}
pub struct MenuBar {
    // pub items: Vec<(String, Button, Dropdown)>,
    // pub items_: Vec<(String, Button)>,
    pub items: Vec<MenuItem>,
    current_item: Option<usize>,
}

impl MenuBar {
    // n_item_names: Vec<(String, Vec<String>)>, 
    pub fn new(tr: &TextRenderer, item_names: Vec<(String, f32, Vec<String>)>) -> MenuBar {
        // Generate a vector of all of the buttons and position them
        let mut items: Vec<MenuItem> = Vec::with_capacity(item_names.len());
        let mut x_pos = 0.0;
        for (s, min_w, d) in item_names {
            let button = LabeledButton::new(tr, s, Some((1.0, 1.0, 2.0, 2.0)), Vec2::new(x_pos, 0.0), button::PressMode::Immediate, false);
            let dropdown = Dropdown::new(tr, d, Vec2::new(button.b.rect.x, button.b.rect.y + button.b.rect.h), min_w);
            x_pos += button.b.rect.w;
            items.push(MenuItem { button, dropdown });
        }
        MenuBar { items, current_item: None }
    }

    pub fn menu_button_pressed(&mut self, menu_index: usize, dropdown_index: usize) -> bool {
        if let Some(menu_item) = self.items.get_mut(menu_index) {
            if let Some(dropdown_item) = menu_item.dropdown.items.get_mut(dropdown_index) {
                dropdown_item.pressed()
            } else { println!("Warning! Tried to get invalid dropdown in menu_button_pressed({:?}, {:?})!!", menu_index, dropdown_index); false }
        } else { println!("Warning! Tried to get invalid menu in menu_button_pressed({:?}, {:?})!!", menu_index, dropdown_index); false }
    }

    // TODO: This function isn't very good.
    // Update each of the buttons in the menu bar
    pub fn update(&mut self, mouse_pos: Vec2, mouse_mode: super::button::MouseMode) {
        // If we've just clicked and we're not overlapping a menu item or dropdown, close the menus 
        if mouse_mode == super::button::MouseMode::Down {
            if let Some(current_item_index) = self.current_item {
                if self.items[current_item_index].button.b.state == super::button::State::Idle && !self.items[current_item_index].dropdown.mouse_over(mouse_pos) {
                    self.current_item = None;
                }
            } else {
                self.current_item = None;
            }
        }

        for (i, menu_item) in &mut self.items.iter_mut().enumerate() {
            menu_item.button.update(mouse_pos, mouse_mode);
            if menu_item.button.pressed() {
                // If we're already on this menu item, clicking it should close it
                if self.current_item.is_some_and(|c| c == i) {
                    self.current_item = None;
                } else {
                    self.current_item = Some(i);
                }
            }
        }

        if let Some(current_item_index) = self.current_item {
            // If the current menu or it's dropdown aren't being hovered over, we want to see if another one is and switch to that!
            if self.items[current_item_index].button.b.state == super::button::State::Idle && !self.items[current_item_index].dropdown.mouse_over(mouse_pos) {
                // If another menuitem is being hovered over or clicked on, make THAT the current one!
                let other_hovered_menu: Vec<usize> = self.items.iter().enumerate()
                    .filter_map(|(i, items)| match items.button.b.state {
                        button::State::Hovered | button::State::Depressed => Some(i), _ => None,
                    }).collect();
                if let Some(other) = other_hovered_menu.get(0) {
                    self.current_item = Some(*other);
                }
            }
        }
        // TODO: Hide the menus when a dropdown item is pressed
        if let Some(current_item_index) = self.current_item {
            // Update the menu
            if self.items[current_item_index].dropdown.update(mouse_pos, mouse_mode) {
                // self.current_item = None;
            }
        }
    }
    // Renders the menu bar as well as dropdowns
    pub fn render(&self, canvas: &mut Canvas, tr: &mut TextRenderer, spritesheet: &mut InstanceArray) {
        for (i, menu_item) in self.items.iter().enumerate() {
            // If the current button isn't idle or is the selected one, make it shaded
            let (bg_col, text_col) = if menu_item.button.b.state != button::State::Idle || self.current_item.is_some_and(|c| c == i) {
                (Color::from_rgb(38, 43, 68), Color::from_rgb(255, 255, 255))
            } else {
                (Color::from_rgb(192, 203, 220), Color::from_rgb(24, 20, 37))
            };

            let _ = match (menu_item.button.b.state, self.current_item.is_some_and(|c| c == i)) {
                (button::State::Idle, false) => (Color::from_rgb(192, 203, 220), Color::from_rgb(24, 20, 37)),
                (button::State::Disabled, _) => (Color::RED, Color::BLACK),
                _ => (Color::from_rgb(192, 203, 220), Color::from_rgb(24, 20, 37)),
            };

            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(menu_item.button.b.rect.point())
                    .scale(menu_item.button.b.rect.size())
                    .color(bg_col),
            );
            tr.draw_text(canvas, &menu_item.button.label, DrawParam::new().color(text_col).dest(menu_item.button.text_pos()));
        }

        if let Some(current_item_index) = self.current_item {
            let current_item = &self.items[current_item_index];
            current_item.dropdown.render(canvas, tr, spritesheet);
        }
    }
}