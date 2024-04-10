// UI functions relating to creating a menubar with dropdowns
// I don't know if it's bad practice or not, but this takes ownership of the ui and gives it back when it's done.

use super::*;

// Holds information about the menubar
#[derive(Default)]
pub struct Menubar {
    // TODO: Originally I was gonna use lifetimes but they're a whole can of worms...
    ui: Option<UIState>,
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

    pub fn ui_mut(&mut self) -> &mut UIState {
        self.ui.as_mut().unwrap()
    }
    pub fn ui(&mut self) -> &UIState {
        self.ui.as_ref().unwrap()
    }

    // Take ownership of the ui state
    pub fn begin(&mut self, ui: UIState) {
        self.ui = Some(ui);
        self.next_pos = 0.0;
        // Deselect the menubar if elsewhere is clicked
        let mouse_pos = self.ui().mouse_pos;
        if self.current.is_some()
        && self.ui().mouse_pressed
        && !self.prev_dropdown_rect.contains(mouse_pos)
        {
            self.ui_mut().active_item = SelectedItem::Unavailable;
            self.current = None;
        } 
    }
    // Gives ownership of the ui_state back
    pub fn finish(&mut self) -> UIState {
        let b = DrawShape::Rect {
            x: 0.0,
            y: 0.0,
            w: self.ui().screen_size.x,
            h: self.height,
            color: Color::from_hex(0xC0CBDC)
        };
        self.ui_mut().draw_queue.push(b);
        self.ui.take().unwrap()
    }


    pub fn item(&mut self, text: impl AsRef<str>, dropdown_width: f32) -> bool {
        self.current_pos = self.next_pos;
        self.dropdown_item_width = dropdown_width;

        let size = self.ui().text_renderer.text_size(&text, None) + vec2(4.0, 2.0);
        self.next_pos += size.x;
        self.height = size.y;
        
        self.dropdown_height = self.height;
        self.dropdown_next = vec2(self.current_pos, self.height) + 1.0;
        self.dropdown_current = self.dropdown_next;

        // Button logic
        let rect = Rect::new(
            self.current_pos,
            0.0,
            size.x,
            size.y
        );
        let id = hash_string(&format!("menubar{:?}", text.as_ref()));

        let mut state = ButtonState::Idle;

        if self.ui().hot_item == SelectedItem::None && self.ui().mouse_in_rect(rect) {
            state = ButtonState::Hovered;
            self.ui_mut().hot_item.assign(id);
            if self.ui().active_item == SelectedItem::None && self.ui().mouse_down {
                state = ButtonState::Clicked;
                self.ui_mut().active_item.assign(id)
            }
        }
        if (state == ButtonState::Hovered && self.current.is_some()) || state == ButtonState::Clicked {
            self.current = Some(id);
        }

        let colors = match self.current == Some(id) || state != ButtonState::Idle {
            true  => self.ui().style.menubar_hovered,
            false => self.ui().style.menubar_idle,
        };
        // Rendering
        self.ui_mut().draw_queue.push(DrawShape::label(rect.x + 2.0, rect.y + 1.0, text.as_ref().to_string(), colors.1));
        self.ui_mut().draw_queue.push(DrawShape::rect(rect, colors.0));

        self.current == Some(id)
    }

    pub fn finish_item(&mut self) {
        // Draw the box and it's shadow
        let dropdown_rect = self.dropdown_rect();
        let dropdown_bg_source = self.ui().style.dropdown_bg_source;
        let shadow_color = self.ui().style.shadow_color;
        self.ui_mut().draw_queue.push(DrawShape::nineslice(dropdown_rect, dropdown_bg_source));
        self.ui_mut().draw_queue.push(DrawShape::rect(dropdown_rect.offset(Vec2::splat(3.0)), shadow_color));

        // Make it so we can't hover over things through the dropdown
        if dropdown_rect.contains(self.ui().mouse_pos) && self.ui().hot_item == SelectedItem::None {
            self.ui_mut().hot_item = SelectedItem::Unavailable;
        }
        self.prev_dropdown_rect = dropdown_rect;
    }


    fn dropdown_move_down_by(&mut self, amount: f32) {
        self.dropdown_next.y += amount;
        self.dropdown_height = f32::max(self.dropdown_height, self.dropdown_next.y);
    }

    pub fn dropdown_rect(&self) -> Rect {
        Rect {
            x: self.current_pos,
            y: self.height,
            w: 1.0 + self.dropdown_current.x - self.current_pos + self.dropdown_item_width,
            h: 1.0 + self.dropdown_height - self.height,
        }
    }

    fn dropdown_item(&mut self, text: impl AsRef<str>, draw_mark: bool) -> bool {
        self.dropdown_current = self.dropdown_next;

        // Button logic
        let rect = Rect::new(
            self.dropdown_current.x,
            self.dropdown_current.y,
            self.dropdown_item_width,
            self.ui().text_renderer.text_size(&text, None).y + 3.0
        );
        let id = hash_string(&format!("{:?}{}", self.current.unwrap_or(0), text.as_ref()));
        
        if self.ui().hot_item == SelectedItem::None && self.ui().mouse_in_rect(rect) {
            self.ui_mut().hot_item.assign(id);
            if self.ui().mouse_down {
                self.ui_mut().active_item.assign(id)
            }
        }
        let released = self.ui().hot_item == id && self.ui().active_item == id && !self.ui().mouse_down;
        if released {
            self.current = None;
        }

        // Rendering
        let colors = match self.ui().hot_item {
            SelectedItem::Some(hot_item) if id == hot_item => self.ui().style.menubar_hovered,
            _ => self.ui().style.menubar_idle
        };

        if draw_mark {
            // TODO: The same shape (A square to the left of the text) is used for both radios and checkboxes.
            // Is this bad? I don't think so, but I might have to think about it more.
            let mark = DrawShape::Rect {
                x: self.dropdown_current.x + 2.0, 
                y: self.dropdown_current.y + 3.0,
                w: 3.0, 
                h: 3.0, 
                color: colors.1,
            };
            self.ui_mut().draw_queue.push(mark);
        }
        self.ui_mut().draw_queue.push(DrawShape::label(rect.x + 7.0, rect.y + 2.0, text.as_ref().to_owned(), colors.1));
        self.ui_mut().draw_queue.push(DrawShape::rect(rect, colors.0));

        self.dropdown_move_down_by(rect.h);
        released
    }

    pub fn dropdown(&mut self, text: impl AsRef<str>) -> bool {
        self.dropdown_item(text, false)
    }

    // TODO: Not specific to this bit of code
    // If this is called and the radio variable is changed to something else, and that thing is used as a qualifier AFTER the previous one
    // You'll see it flash for a split second before the menu closes. Maybe I need some kind of buffer or idk what.
    pub fn dropdown_radio(&mut self, text: impl AsRef<str>, qualifier: bool) -> bool {
        self.dropdown_item(text, qualifier)
    }

    pub fn dropdown_checkbox(&mut self, text: impl AsRef<str>, value: &mut bool) -> bool {
        let pressed = self.dropdown_item(text, *value);
        if pressed { *value = !*value; }
        pressed
    }

    pub fn dropdown_separator(&mut self) {
        let source = self.ui().style.separator_source;
        let dest = Rect::new(
            self.dropdown_next.x + 1.0,
            self.dropdown_next.y,
            self.dropdown_item_width - 2.0,
            source.h,
        );
        self.ui_mut().draw_queue.push(DrawShape::ImageRect { dest, source });
        self.dropdown_move_down_by(source.h);
    }

    pub fn dropdown_new_column(&mut self) {
        self.dropdown_next.x += self.dropdown_item_width;
        self.dropdown_next.y = self.height + 1.0;
        self.dropdown_current = self.dropdown_next;
    }
}