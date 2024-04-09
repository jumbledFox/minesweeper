// UI functions relating to creating a menubar with dropdowns

use macroquad::miniquad::window::screen_size;

use super::*;

// Holds information about the menubar
#[derive(Default)]
pub struct Menubar {
    // The currently opened menubar
    current: Option<u64>,
    // How tall the menubar is, used for aligining elements bar at the top of the screen is
    height: f32,
    current_pos: f32,
    next_pos: f32,
    // How wide the items in the dropdown will be
    dropdown_item_width: f32,
    // How tall the dropdown is currently
    dropdown_height: f32,
    // Where the current dropdown item is and where the next one should be
    dropdown_current: Vec2,
    dropdown_next: Vec2,
    // The rect of the dropdown on the last frame
    prev_dropdown_rect: Rect,
}

impl Menubar {
    pub fn reset(&mut self) {
        self.height = 0.0;
        self.current_pos = 0.0;
        self.next_pos = 0.0;
        self.dropdown_item_width = 0.0;
        self.dropdown_height = 0.0;
        self.dropdown_current = Vec2::ZERO;
        self.dropdown_next = Vec2::ZERO;
    }
}

impl UIState {
    pub fn begin_menubar(&mut self) {
        self.menubar.next_pos = 0.0;
    }

    pub fn finish_menubar(&mut self) {
        self.drawqueue.push(DrawShape::Rect {
            x: 0.0,
            y: 0.0,
            w: screen_size().0,
            h: self.menubar.height,
            color: Color::from_hex(0xC0CBDC)
        })
    }

    pub fn deselect_menubar(&mut self) {
        if self.menubar.current.is_some()
        && self.mouse_pressed
        && !self.menubar.prev_dropdown_rect.contains(self.mouse_pos) {
            self.menubar.current = None;
            self.active_item = SelectedItem::Unavailable;
        }
    }

    pub fn menu_item(&mut self, text: String, dropdown_width: f32) -> bool {
        let size = self.text_renderer.text_size(&text, None) + Vec2::new(4.0, 2.0);
        self.menubar.current_pos = self.menubar.next_pos;
        self.menubar.next_pos += size.x;
        self.menubar.height = size.y;

        self.menubar.dropdown_item_width = dropdown_width;
        self.menubar.dropdown_height = 0.0;
        self.menubar.dropdown_next = Vec2::new(self.menubar.current_pos, self.menubar.height) + 1.0;

        let rect = Rect::new(
            self.menubar.current_pos,
            0.0,
            size.x,
            size.y
        );
        let id = hash_string(&format!("menubar{}", text));

        let mut state = ButtonState::Idle;
        if self.hot_item == SelectedItem::None && self.mouse_in_rect(rect) {
            state = ButtonState::Hovered;
            self.hot_item.assign(id);
            if self.active_item == SelectedItem::None && self.mouse_down {
                state = ButtonState::Clicked;
                self.active_item.assign(id)
            }
        }
        if (state == ButtonState::Hovered && self.menubar.current.is_some()) || state == ButtonState::Clicked {
            self.menubar.current = Some(id);
        }

        let colors = match self.menubar.current == Some(id) || state != ButtonState::Idle {
            true  => self.style.menubar_hovered,
            false => self.style.menubar_idle,
        };

        self.drawqueue.push(DrawShape::label(rect.x + 2.0, rect.y + 1.0, text, colors.1));
        self.drawqueue.push(DrawShape::rect(rect, colors.0));

        self.menubar.current == Some(id)
    }

    pub fn finish_menu_item(&mut self) {
        // Draw the box and it's shadow
        self.drawqueue.push(DrawShape::nineslice(self.dropdown_rect(), self.style.dropdown_bg_source));
        self.drawqueue.push(DrawShape::rect(self.dropdown_rect().offset(Vec2::splat(3.0)), self.style.shadow_col));
        // Make it so we can't hover over things through the dropdown
        if self.dropdown_rect().contains(self.mouse_pos) && self.hot_item == SelectedItem::None {
            self.hot_item = SelectedItem::Unavailable;
        }
        self.menubar.prev_dropdown_rect = self.dropdown_rect();
    }

    pub fn dropdown_rect(&self) -> Rect {
        Rect {
            x: self.menubar.current_pos,
            y: self.menubar.height,
            w: 1.0 + self.menubar.dropdown_current.x - self.menubar.current_pos + self.menubar.dropdown_item_width,
            h: 1.0 + self.menubar.dropdown_height - self.menubar.height,
        }
    }

    // Move the position of the next dropdown item down by an amount
    fn dropdown_move_down_by(&mut self, amount: f32) {
        self.menubar.dropdown_next.y += amount;
        self.menubar.dropdown_height = self.menubar.dropdown_height.max(self.menubar.dropdown_next.y);
    }

    // A nice little separator
    pub fn dropdown_separator(&mut self) {
        self.drawqueue.push(DrawShape::image_rect(
            Rect::new(
                self.menubar.dropdown_next.x + 1.0,
                self.menubar.dropdown_next.y + 1.0,
                self.menubar.dropdown_item_width - 2.0,
                self.style.separator_source.h,
            ),
            self.style.separator_source,
        ));
        self.dropdown_move_down_by(3.0);
    }

    // Makes the dropdown menu have a new column
    pub fn dropdown_new_column(&mut self) {
        self.menubar.dropdown_next.x += self.menubar.dropdown_item_width;
        self.menubar.dropdown_next.y = 1.0 + self.menubar.height;
    }

    pub fn dropdown_item(&mut self, text: String) -> ButtonState {
        self.menubar.dropdown_current = self.menubar.dropdown_next;
        let rect = Rect::new(
            self.menubar.dropdown_current.x,
            self.menubar.dropdown_current.y,
            self.menubar.dropdown_item_width,
            self.text_renderer.text_size(&text, None).y + 3.0
        );

        // The id of the dropdown item, calculated from its text and the id of the menu item it belongs to
        let id = hash_string(&format!("{:?}{text}", self.menubar.current.unwrap_or(0)));
        
        let mut state = ButtonState::Idle;
        if self.hot_item == SelectedItem::None && self.mouse_in_rect(rect) {
            state = ButtonState::Hovered;
            self.hot_item.assign(id);
            if self.mouse_down {
                state = ButtonState::Clicked;
                self.active_item.assign(id)
            }
        }
        if self.hot_item == id && self.active_item == id && !self.mouse_down {
            state = ButtonState::Released;
            self.menubar.current = None;
        }
        let colors = match self.hot_item {
            SelectedItem::Some(d) if id == d => self.style.menubar_hovered,
            _ => self.style.menubar_idle
        };

        self.drawqueue.push(DrawShape::label(rect.x + 7.0, rect.y + 2.0, text, colors.1));
        self.drawqueue.push(DrawShape::rect(rect, colors.0));

        self.dropdown_move_down_by(rect.h);
        state
    }

    pub fn dropdown_item_radio(&mut self, text: String, qualifier: bool) -> ButtonState {
        let prev_index = self.drawqueue.len();
        let state = self.dropdown_item(text);
        if qualifier {
            self.drawqueue.insert(prev_index, DrawShape::Rect {
                x: self.menubar.dropdown_current.x + 2.0, 
                y: self.menubar.dropdown_current.y + 3.0,
                w: 3.0, 
                h: 3.0, 
                color: if state != ButtonState::Idle {self.style.menubar_hovered} else {self.style.menubar_idle}.1,
            });
        }
        state
    }

    pub fn dropdown_item_checkbox(&mut self, text: String, value: &mut bool) -> ButtonState {
        // Personally, I think using a square for a checkbox property isn't confusing
        // TODO: Maybe change this
        let prev_index = self.drawqueue.len();
        let state = self.dropdown_item(text);
        // Draw the 'check mark' if the value is true
        if *value {
            self.drawqueue.insert(prev_index, DrawShape::Rect {
                x: self.menubar.dropdown_current.x + 2.0, 
                y: self.menubar.dropdown_current.y + 3.0,
                w: 3.0, 
                h: 3.0, 
                color: if state != ButtonState::Idle {self.style.menubar_hovered} else {self.style.menubar_idle}.1,
            });
        }
        if state == ButtonState::Released {
            *value = !*value;
        }
        state
    }
}