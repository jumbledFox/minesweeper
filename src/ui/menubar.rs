// UI functions relating to creating a menubar with dropdowns.
// Since a menubar is pretty big and has a lot of functions, as well as members that should persist across frames,
// I've made it it's own struct that stores a reference to the UI.

use std::{cell::RefCell, rc::Rc};

use super::*;

// Holds information about the menubar
pub struct Menubar {
    // A reference to the UI, much nicer than passing ui to the menubar every single time we need it.
    // Originally I was gonna use lifetimes but they're a whole can of worms...
    ui: Rc<RefCell<UIState>>,
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

/* TODO:
Either use try_borrow_mut rather than having possible panicking, or move away entirely from rc refcells.. they're bad i think!!!
let mut ui = match self.ui.try_borrow_mut() {
    Ok(ui) => ui,
    Err(_) => return,
};
 */

impl Menubar {
    pub fn new(ui: Rc<RefCell<UIState>>) -> Menubar {
        Menubar {
            ui,
            current: None, height: 0.0,
            current_pos: 0.0, next_pos: 0.0,
            dropdown_item_width: 0.0, dropdown_height: 0.0,
            dropdown_current: Vec2::ZERO, dropdown_next: Vec2::ZERO,
            prev_dropdown_rect: Rect::default()
        }
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn reset(&mut self) {
        self.height = 0.0;
        self.current_pos = 0.0;
        self.next_pos = 0.0;
        self.dropdown_item_width = 0.0;
        self.dropdown_height = 0.0;
        self.dropdown_current = Vec2::ZERO;
        self.dropdown_next = Vec2::ZERO;
    }

    pub fn begin(&mut self) {
        self.next_pos = 0.0;
        // Deselect the menubar if elsewhere is clicked
        let mouse_pos = self.ui.borrow().mouse_pos;
        if self.current.is_some()
        && self.ui.borrow().mouse_pressed
        && !self.prev_dropdown_rect.contains(mouse_pos)
        {
            self.ui.borrow_mut().active_item = SelectedItem::Unavailable;
            self.current = None;
        } 
    }

    pub fn finish(&mut self) {
        let b = DrawShape::Rect {
            x: 0.0,
            y: 0.0,
            w: self.ui.borrow().screen_size.x,
            h: self.height,
            color: Color::from_hex(0xC0CBDC)
        };
        self.ui.borrow_mut().draw_queue().push(b);
    }


    pub fn item(&mut self, text: impl AsRef<str>, dropdown_width: f32) -> bool {
        self.current_pos = self.next_pos;
        self.dropdown_item_width = dropdown_width;

        let size = self.ui.borrow().text_renderer.text_size(&text, None) + vec2(4.0, 2.0);
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

        let mut ui = self.ui.borrow_mut();
        let mut state = ButtonState::Idle;
        if ui.hot_item == SelectedItem::None && ui.mouse_in_rect(rect) {
            state = ButtonState::Hovered;
            ui.hot_item.assign(id);
            if ui.active_item == SelectedItem::None && ui.mouse_down {
                state = ButtonState::Clicked;
                ui.active_item.assign(id)
            }
        }
        if (state == ButtonState::Hovered && self.current.is_some()) || state == ButtonState::Clicked {
            self.current = Some(id);
        }

        let colors = match self.current == Some(id) || state != ButtonState::Idle {
            true  => spritesheet::menubar_hovered(),
            false => spritesheet::menubar_idle(),
        };
        // Rendering
        ui.draw_queue().push(DrawShape::label(rect.x + 2.0, rect.y + 1.0, text.as_ref().to_string(), colors.1));
        ui.draw_queue().push(DrawShape::rect(rect, colors.0));

        self.current == Some(id)
    }

    pub fn finish_item(&mut self) {
        let mut ui = self.ui.borrow_mut();
        // Draw the box and it's shadow
        let dropdown_rect = self.dropdown_rect();
        ui.draw_queue().push(DrawShape::nineslice(dropdown_rect, spritesheet::DROPDOWN_BACKGROUND));
        ui.draw_queue().push(DrawShape::rect(dropdown_rect.offset(Vec2::splat(3.0)), spritesheet::shadow()));

        // Make it so we can't hover over things through the dropdown
        if dropdown_rect.contains(ui.mouse_pos) && ui.hot_item == SelectedItem::None {
            ui.hot_item = SelectedItem::Unavailable;
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
        let mut ui = self.ui.borrow_mut();
        let rect = Rect::new(
            self.dropdown_current.x,
            self.dropdown_current.y,
            self.dropdown_item_width,
            ui.text_renderer.text_size(&text, None).y + 3.0
        );
        let id = hash_string(&format!("{:?}{}", self.current.unwrap_or(0), text.as_ref()));
        
        if ui.hot_item == SelectedItem::None && ui.mouse_in_rect(rect) {
            ui.hot_item.assign(id);
            if ui.mouse_down {
                ui.active_item.assign(id)
            }
        }
        let released = ui.hot_item == id && ui.active_item == id && !ui.mouse_down;
        if released {
            self.current = None;
        }

        // Rendering
        let colors = match ui.hot_item {
            SelectedItem::Some(hot_item) if id == hot_item => spritesheet::menubar_hovered(),
            _ => spritesheet::menubar_idle()
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
            ui.draw_queue().push(mark);
        }
        ui.draw_queue().push(DrawShape::label(rect.x + 7.0, rect.y + 2.0, text.as_ref().to_owned(), colors.1));
        ui.draw_queue().push(DrawShape::rect(rect, colors.0));
        drop(ui);

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
        let source = spritesheet::DROPDOWN_SEPARATOR;
        let dest = Rect::new(
            self.dropdown_next.x + 1.0,
            self.dropdown_next.y,
            self.dropdown_item_width - 2.0,
            source.h,
        );
        self.ui.borrow_mut().draw_queue().push(DrawShape::ImageRect { dest, source });
        self.dropdown_move_down_by(source.h);
    }

    pub fn dropdown_new_column(&mut self) {
        self.dropdown_next.x += self.dropdown_item_width;
        self.dropdown_next.y = self.height + 1.0;
        self.dropdown_current = self.dropdown_next;
    }
}