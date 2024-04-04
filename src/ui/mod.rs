use macroquad::prelude::*;
use macroquad::math::Vec2;

pub enum DrawShape {
    Rect{x: f32, y: f32, w: f32, h: f32, color: Color},
}

#[derive(PartialEq)]
enum SelectedItem {
    None,
    Some(usize),
    Unavailable,
}

impl PartialEq<usize> for SelectedItem {
    fn eq(&self, other: &usize) -> bool {
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

    drawqueue: Vec<DrawShape>,
}

impl UIState {
    pub fn new() -> UIState {
        UIState {
            mouse_pos: Vec2::ZERO,
            mouse_down: false,
            hot_item: SelectedItem::None,
            active_item: SelectedItem::None,

            drawqueue: vec![],
        }
    }

    pub fn prepare(&mut self) {
        self.mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        self.mouse_down =  is_mouse_button_down(MouseButton::Left);
        self.hot_item = SelectedItem::None;
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
            match *d {
                DrawShape::Rect { x, y, w, h, color } => draw_rectangle(x, y, w, h, color), 
            }
        }
    }

    pub fn mouse_in_rect(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.mouse_pos.x >= x     &&
        self.mouse_pos.x <  x + w &&
        self.mouse_pos.y >= y     &&
        self.mouse_pos.y <  y + h
    }


    pub fn button(&mut self, id: usize, x: f32, y: f32, w: f32, h: f32) -> bool { 
        // Detect whether the button should be hot / active
        if self.hot_item == SelectedItem::None && self.mouse_in_rect(x, y, w, h) {
            self.hot_item = SelectedItem::Some(id);
            if self.active_item == SelectedItem::None && self.mouse_down {
                self.active_item = SelectedItem::Some(id);
            }
        }

        // Rendering
        let (rect_x, rect_y, color) = match (&self.hot_item, &self.active_item) {
            // Hot and active (Pressed)
            (hot_item, active_item) if *hot_item == id && *active_item == id => (x + 2.0, y + 2.0, Color::from_hex(0xAAAAAA)),
            // Hot, but not active (Hovered)
            (hot_item, _) if *hot_item == id => (x, y, Color::from_hex(0xFFFFFF)),
            // Otherwise
            _  => (x, y, Color::from_hex(0xDDDDDD)),
        };
        self.drawqueue.push(DrawShape::Rect { x: rect_x, y: rect_y, w, h, color });
        // Draw the shadow
        self.drawqueue.push(DrawShape::Rect { x: x + 4.0, y: y + 4.0, w, h, color: Color::from_hex(0x000000) });

        // If the button is hot and active, but the mouse isn't down, the user must've clicked the button
        !self.mouse_down && self.hot_item == id && self.active_item == id
    }

    pub fn checkbox(&mut self, id: usize, value: &mut bool, x: f32, y: f32, w: f32, h: f32) {
        if *value {
            self.drawqueue.push(DrawShape::Rect { x: x+w/4.0, y: y+h/4.0, w: w*(2.0/4.0), h: h*(2.0/4.0), color: Color::from_hex(0x333333) });
        }
        if self.button(id, x, y, w, h) {
            *value = !*value;
        }
    }
}