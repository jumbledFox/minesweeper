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
}