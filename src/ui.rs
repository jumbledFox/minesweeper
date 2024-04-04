use macroquad::prelude::*;
use macroquad::math::Vec2;

enum WidgetRenderable {
    Button(usize), // TODO: maybe enums?
    Checkbox(usize),
}

#[derive(PartialEq)]
enum ActiveItem {
    None,
    Some(usize),
    Unavailable,
}

pub struct UIState {
    mouse_pos: Vec2,
    mouse_down: bool,
    hot_item: Option<usize>,
    active_item: ActiveItem,

    renderstack: Vec<WidgetRenderable>,
}

impl UIState {
    pub fn new() -> UIState {
        UIState {
            mouse_pos: Vec2::ZERO,
            mouse_down: false,
            hot_item: None,
            active_item: ActiveItem::None,

            renderstack: vec![],
        }
    }

    pub fn prepare(&mut self) {
        self.mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        self.mouse_down =  is_mouse_button_down(MouseButton::Left);
        self.hot_item = None;

        self.renderstack = Vec::new();
    }
    
    pub fn finish(&mut self) {
        if !self.mouse_down {
            self.active_item = ActiveItem::None;
        } else {
            if self.active_item == ActiveItem::None {
                self.active_item = ActiveItem::Unavailable;
            }
        }

        // render
        for r in &self.renderstack {
            
        }
    }

    pub fn mouse_in_rect(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.mouse_pos.x >= x     &&
        self.mouse_pos.x <  x + w &&
        self.mouse_pos.y >= y     &&
        self.mouse_pos.y <  y + h
    }

    // Simple button widget
    pub fn button(&mut self, id: usize, x: f32, y: f32, w: f32, h: f32) -> bool {
        // Detect whether the button should be hot / active
        if self.mouse_in_rect(x, y, w, h) {
            self.hot_item = Some(id);
            if self.active_item == ActiveItem::None && self.mouse_down {
                self.active_item = ActiveItem::Some(id);
            }
        }

        // // Render the button
        // draw_rectangle(x + 4.0, y + 4.0, w, h, Color::from_hex(0x000000));
        // if self.hot_item == Some(id) {
        //     if self.active_item == ActiveItem::Some(id) {
        //         // Button is both 'hot' and 'active'
        //         draw_rectangle(x + 2.0, y + 2.0, w, h, Color::from_hex(0xAAAAAA));
        //     } else {
        //         // Button is merely 'hot'
        //         draw_rectangle(x, y, w, h, Color::from_hex(0xFFFFFF));
        //     }
        // } else {
        //     // Button isn't 'hot', but could be active
        //     draw_rectangle(x, y, w, h, Color::from_hex(0xDDDDDD));
        // }

        let s = if self.hot_item == Some(id) {
            if self.active_item == ActiveItem::Some(id) { 2 } else { 1 }
        } else { 0 };
        self.renderstack.push(WidgetRenderable::Button(s));
        
        // If the button is hot and active, but the mouse isn't down, the user must've clicked the button
        !self.mouse_down && self.hot_item == Some(id) && self.active_item == ActiveItem::Some(id)
    }

    pub fn checkbox(&mut self, id: usize, value: &mut bool, x: f32, y: f32, w: f32, h: f32) {
        // Detect whether the button should be hot / active
        if self.mouse_in_rect(x, y, w, h) {
            self.hot_item = Some(id);
            if self.active_item == ActiveItem::None && self.mouse_down {
                self.active_item = ActiveItem::Some(id);
            }
        }

        // If the button is hot and active, but the mouse isn't down, the user must've clicked the checkbox, so toggle it
        if !self.mouse_down && self.hot_item == Some(id) && self.active_item == ActiveItem::Some(id) {
            *value = !*value;
        }

        // Rendering
        // if self.hot_item == Some(id) {
        //     if self.active_item == ActiveItem::Some(id) {
        //         // Button is both 'hot' and 'active'
        //         draw_rectangle(x, y, w, h, Color::from_hex(0xAAAAAA));
        //     } else {
        //         // Button is merely 'hot'
        //         draw_rectangle(x, y, w, h, Color::from_hex(0xFFFFFF));
        //     }
        // } else {
        //     // Button isn't 'hot', but could be active
        //     draw_rectangle(x, y, w, h, Color::from_hex(0xDDDDDD));
        // }
        if *value {
            draw_rectangle(x + w/5.0, y + h/5.0, w * (3.0/5.0), h * (3.0/5.0), Color::from_hex(0x333333));
        }
    }
}