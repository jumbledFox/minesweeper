use std::hash::{DefaultHasher, Hash, Hasher};

// TODO:
// Make it so that there's a LayoutBuilder that aligns all of the elements to the right position.
// This will mean a frame of lag.. but ¯\_(ツ)_/¯
// I'd also like to have a stack of Windows, which are just an ID and a rect. It could be reordered which would be good,
// I can use the last frames window stack to see if I should interact with a window / elements or not
// Maybe I could give everything a stack.. idk

pub fn hash_string(input: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

use macroquad::prelude::*;
use macroquad::math::Vec2;

use self::text_renderer::TextRenderer;

pub mod text_renderer;

pub enum DrawShape {
    Label{x: f32, y: f32, text: String, color: Color},
    Rect{x: f32, y: f32, w: f32, h: f32, color: Color},
}

#[derive(PartialEq)]
enum SelectedItem {
    None,
    Some(u64),
    Unavailable,
}

impl PartialEq<u64> for SelectedItem {
    fn eq(&self, other: &u64) -> bool {
        match &self {
            SelectedItem::Some(s) if *s == *other => true,
            _ => false,
        }
    }
}

pub struct UIState {
    mouse_pos: Vec2,
    mouse_down: bool,
    hot_item: SelectedItem,
    active_item: SelectedItem,

    menubar_info: MenubarInfo,

    text_renderer: TextRenderer,
    drawqueue: Vec<DrawShape>,
}

impl UIState {
    pub fn new() -> UIState {
        UIState {
            mouse_pos: Vec2::ZERO,
            mouse_down: false,
            hot_item: SelectedItem::None,
            active_item: SelectedItem::None,

            menubar_info: MenubarInfo::new(),

            text_renderer: TextRenderer::new(),
            drawqueue: vec![],
        }
    }

    pub fn begin(&mut self) {
        self.mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        self.mouse_down =  is_mouse_button_down(MouseButton::Left);
        self.hot_item = SelectedItem::None;
        self.menubar_info.reset();
        self.drawqueue = Vec::new();
    }
    
    pub fn finish(&mut self) {
        if !self.mouse_down {
            self.active_item = SelectedItem::None;
        } else {
            if self.active_item == SelectedItem::None {
                self.active_item = SelectedItem::Unavailable;
            }
        }

        // Draw all of the elements so the first ones are drawn last and appear on top
        for d in self.drawqueue.iter().rev() {
            match d {
                DrawShape::Label { x, y, text, color } => self.text_renderer.draw_text(text, *x, *y, *color, None),
                &DrawShape::Rect { x, y, w, h, color } => draw_rectangle(x, y, w, h, color), 
            }
        }
    }

    pub fn mouse_in_rect(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.mouse_pos.x >= x     &&
        self.mouse_pos.x <  x + w &&
        self.mouse_pos.y >= y     &&
        self.mouse_pos.y <  y + h
    }

    pub fn label(&mut self, text: String, color: Color, x: f32, y: f32) {
        self.drawqueue.push(DrawShape::Label { x, y, text, color });
    }
}

#[derive(PartialEq)]
pub enum ButtonState {
    Clicked, Released, Held, Hovered, Idle,
}

impl UIState {
    pub fn button(&mut self, text: String, x: f32, y: f32, w: f32, h: f32) -> ButtonState {
        let id = hash_string(&text);

        let mut clicked = false;
        // Detect whether the button should be hot / active
        if self.hot_item == SelectedItem::None && self.mouse_in_rect(x, y, w, h) {
            self.hot_item = SelectedItem::Some(id);
            if self.active_item == SelectedItem::None && self.mouse_down {
                self.active_item = SelectedItem::Some(id);
                clicked = true;
            }
        }

        // Rendering
        let (rect_x, rect_y, color, state) = match (&self.hot_item, &self.active_item) {
            // Hot and active (Held)
            (hot_item, active_item) if *hot_item == id && *active_item == id => (x + 2.0, y + 2.0, Color::from_hex(0xAAAAAA), ButtonState::Held),
            // Hot, but not active (Hovered)
            (hot_item, _) if *hot_item == id => (x, y, Color::from_hex(0xFFFFFF), ButtonState::Hovered),
            // Otherwise
            _  => (x, y, Color::from_hex(0xDDDDDD), ButtonState::Idle),
        };
        let label_rel_pos = (Vec2::new(w, h)-self.text_renderer.text_size(&text, None))/2.0;

        self.drawqueue.push(DrawShape::Label { x: rect_x + label_rel_pos.x, y: rect_y + label_rel_pos.y, text, color: Color::from_hex(0x000000) });
        self.drawqueue.push(DrawShape::Rect { x: rect_x, y: rect_y, w, h, color });
        // Draw the shadow
        // self.drawqueue.push(DrawShape::Rect { x: x + 4.0, y: y + 4.0, w, h, color: Color::from_hex(0x000000) });

        // If the button is hot and active, but the mouse isn't down, the user must've released the button
        if !self.mouse_down && self.hot_item == id && self.active_item == id {
            return ButtonState::Released;
        } else {
            if clicked {
                return ButtonState::Clicked;
            } else {
                return state;
            }
        }
    }

    pub fn checkbox(&mut self, label: String, value: &mut bool, x: f32, y: f32, w: f32, h: f32) {
        let drawqueue_before_index = self.drawqueue.len();

        let offset = match self.button(label, x, y, w, h) {
            ButtonState::Held => 2.0,
            ButtonState::Released => {*value = !*value; 0.0}
            _ => 0.0
        };
        if *value {
            self.drawqueue.insert(drawqueue_before_index, DrawShape::Rect { x: x+w/4.0+offset, y: y+h/4.0+offset, w: w*(2.0/4.0), h: h*(2.0/4.0), color: Color::from_hex(0x333333) });
        }
    }
}

struct MenubarInfo {
    height: f32,
    current_pos: f32,
    next_pos: f32,
    dropdown_width: f32,
    dropdown_x: f32,
    dropdown_y: f32,
}

impl MenubarInfo {
    pub fn new() -> MenubarInfo {
        MenubarInfo {
            height: 0.0,
            current_pos: 0.0,
            next_pos: 0.0,
            dropdown_width: 0.0,
            dropdown_x: 1.0,
            dropdown_y: 1.0,
        }
    }
    pub fn reset(&mut self) {
        self.next_pos = 0.0;
    }
}

impl UIState {
    pub fn begin_menubar(&mut self) {

    }

    pub fn finish_menubar(&mut self) {
        self.drawqueue.push(DrawShape::Rect {
            x: 0.0,
            y: 0.0,
            w: 9999.0,
            h: self.menubar_info.height,
            color: Color::from_hex(0xC0CBDC) })
    }

    pub fn menu_item(&mut self, text: String, dropdown_width: f32) -> bool {
        self.menubar_info.dropdown_width = dropdown_width;
        self.menubar_info.dropdown_x = 1.0;
        self.menubar_info.dropdown_y = 1.0;

        self.menubar_info.current_pos = self.menubar_info.next_pos;
        
        let size = self.text_renderer.text_size(&text, None) + Vec2::new(4.0, 2.0);
        
        let b = self.button(text.clone(), self.menubar_info.current_pos, 0.0, size.x, size.y);

        self.menubar_info.next_pos += size.x;
        self.menubar_info.height = size.y;

        true
    }

    // TODO: Align all buttons text to the left
    pub fn dropdown(&mut self, text: String) -> bool {
        let size = self.text_renderer.text_size(&text, None) + Vec2::new(4.0, 2.0);
        
        let b = self.button(
            text.clone(),
            self.menubar_info.dropdown_x + self.menubar_info.current_pos,
            self.menubar_info.dropdown_y + self.menubar_info.height,
            self.menubar_info.dropdown_width, 
            size.y
        );
        self.menubar_info.dropdown_y += size.y;

        b == ButtonState::Released
    }
    // TODO: Add dropdown_radio and dropdown_checkbox types
    pub fn dropdown_radio<T>(&mut self, value: &mut T, assign: &mut T) {
        
    }

    pub fn dropdown_separator(&mut self) {
        self.menubar_info.dropdown_y += 4.0 
    }
    pub fn dropdown_new_column(&mut self) {
        self.menubar_info.dropdown_x += self.menubar_info.dropdown_width;
        self.menubar_info.dropdown_y = 1.0;
    }
}