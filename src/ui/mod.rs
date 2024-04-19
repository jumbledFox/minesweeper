use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_string(input: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

pub trait RectFeatures {
    fn round(&self) -> Self;
    fn centered(center_x: f32, center_y: f32, w: f32, h: f32) -> Self;
}

impl RectFeatures for Rect {
    fn round(&self) -> Self {
        Rect { x: self.x.round(), y: self.y.round(), w: self.w.round(), h: self.h.round() }
    }
    fn centered(center_x: f32, center_y: f32, w: f32, h: f32) -> Self {
        Rect { x: center_x - w / 2.0, y: center_y - h / 2.0, w, h }
    }
}

use macroquad::prelude::*;
use macroquad::math::Vec2;

pub mod menubar;
pub mod minesweeper;
pub mod text_renderer;

pub mod spritesheet;


use self::spritesheet::Nineslice;
use self::text_renderer::TextRenderer;
pub enum DrawShape {
    Label{x: f32, y: f32, text: String, color: Color},
    Rect{x: f32, y: f32, w: f32, h: f32, color: Color},
    Nineslice{dest: Rect, source: Rect, padding: f32},
    Image{x: f32, y: f32, source: Rect},
    ImageRect{dest: Rect, source: Rect},
}

impl DrawShape {
    pub fn label(x: f32, y: f32, text: String, color: Color) -> Self {
        Self::Label { x, y, text, color }
    }
    pub fn rect(rect: Rect, color: Color) -> Self {
        Self::Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h, color }
    }
    pub fn nineslice(dest: Rect, source: Nineslice) -> Self {
        Self::Nineslice { dest, source: source.rect, padding: source.padding }
    }
    pub fn image(x: f32, y: f32, source: Rect) -> Self {
        Self::Image { x, y, source }
    }
    pub fn image_rect(dest: Rect, source: Rect) -> Self {
        Self::ImageRect { dest, source }
    }
}

#[derive(PartialEq, Debug)]
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

impl SelectedItem {
    pub fn is_none(&self) -> bool {
        matches!(self, SelectedItem::None)
    }
    pub fn assign(&mut self, id: u64) {
        *self = SelectedItem::Some(id);
    }
}

pub struct UIState {
    mouse_pos: Vec2,
    mouse_buttons:    HashMap<MouseButton, (bool, bool)>, // (Down, Pressed)
    screen_size: Vec2,
    hot_item: SelectedItem,
    active_item: SelectedItem,

    text_renderer: TextRenderer,
    draw_queue: Vec<DrawShape>,

    texture: Texture2D,
}

impl UIState {
    pub fn new(texture: Texture2D) -> UIState {
        UIState {
            mouse_pos: Vec2::ZERO,
            mouse_buttons: HashMap::from([
                (MouseButton::Left,   (false, false)),
                (MouseButton::Right,  (false, false)),
                (MouseButton::Middle, (false, false)),
            ]),
            screen_size: Vec2::ONE,
            hot_item: SelectedItem::None,
            active_item: SelectedItem::None,

            text_renderer: TextRenderer::new(),
            draw_queue: vec![],

            texture,
        }
    }

    pub fn draw_queue(&mut self) -> &mut Vec<DrawShape> {
        &mut self.draw_queue
    }
    pub fn screen_size(&self) -> Vec2 {
        self.screen_size
    }

    pub fn begin(&mut self, scale: f32) {
        self.mouse_pos = Vec2::new(mouse_position().0, mouse_position().1) / scale;

        for (&button, (down, pressed)) in &mut self.mouse_buttons.iter_mut() {
            let down_prev = *down;
            *down = is_mouse_button_down(button);
            *pressed = *down && !down_prev;
        }

        let window_size = Vec2::new(screen_width(), screen_height());
        self.screen_size = window_size / scale;
        set_camera(&Camera2D {
            zoom: (scale* 2.0) / window_size,
            target: self.screen_size / 2.0,
            ..Default::default()
        });

        self.hot_item = SelectedItem::None;
        self.draw_queue = Vec::new();
    }
    
    pub fn finish(&mut self) {
        if !self.mouse_down(MouseButton::Left) {
            self.active_item = SelectedItem::None;
        } else {
            if self.active_item == SelectedItem::None {
                self.active_item = SelectedItem::Unavailable;
            }
        }
        // Draw all of the elements so the first ones are drawn last and appear on top
        // Automatically rounds them, we don't want to draw subpixel things
        for d in self.draw_queue.iter().rev() {
            match d {
                // TODO: fix differing &s
                DrawShape::Label { x, y, text, color } => self.text_renderer.draw_text(text, x.round(), y.round(), *color, None),
                &DrawShape::Rect { x, y, w, h, color } => draw_rectangle(x.round(), y.round(), w.round(), h.round(), color),
                &DrawShape::Nineslice { dest, source, padding } => {
                    fn calculate_parts(rect: Rect, pad: f32) -> [Rect; 9] {
                        let rect = rect.round();
                        let pad = pad.round();
                        let middle_size = Vec2::new(rect.w, rect.h) - pad*2.0;
                        [
                            // Middle
                            Rect::new(rect.x + pad, rect.y + pad,  middle_size.x, middle_size.y),
                            // Edges
                            Rect::new(rect.x,                rect.y + pad,  pad, middle_size.y), // Left
                            Rect::new(rect.x + rect.w - pad, rect.y + pad,  pad, middle_size.y), // Right
                            Rect::new(rect.x + pad, rect.y,                 middle_size.x, pad), // Top
                            Rect::new(rect.x + pad, rect.y + rect.h - pad,  middle_size.x, pad), // Bottom
                            // Corners
                            Rect::new(rect.x,                rect.y,                pad, pad), // Top left
                            Rect::new(rect.x + rect.w - pad, rect.y,                pad, pad), // Top right
                            Rect::new(rect.x,                rect.y + rect.h - pad, pad, pad), // Bottom left
                            Rect::new(rect.x + rect.w - pad, rect.y + rect.h - pad, pad, pad), // Bottom right
                        ]
                    }

                    for (&d, &s) in calculate_parts(dest, padding)
                        .iter()
                        .zip(calculate_parts(source, padding).iter())
                    {
                        let params = DrawTextureParams {
                            dest_size: Some(d.size()),
                            source: Some(s),
                            ..Default::default()
                        };
                        draw_texture_ex(&self.texture, d.x, d.y, WHITE, params);
                    }
                },
                &DrawShape::ImageRect { dest, source } => {
                    let (dest, source) = (dest.round(), source.round());
                    let params = DrawTextureParams {
                        dest_size: Some(dest.size()),
                        source: Some(source),
                        ..Default::default()
                    };
                    draw_texture_ex(&self.texture, dest.x, dest.y, WHITE, params);
                },
                &DrawShape::Image { x, y, source } => {
                    let params = DrawTextureParams {
                        source: Some(source.round()),
                        ..Default::default()
                    };
                    draw_texture_ex(&self.texture, x.round(), y.round(), WHITE, params);
                }
            }
        }
    }

    fn mouse_button_values(&self, button: MouseButton) -> (bool, bool) {
        *self.mouse_buttons.get(&button).unwrap_or(&(false, false))
    }
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_button_values(button).0
    }
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_button_values(button).1
    }

    pub fn mouse_in_rect(&self, rect: Rect) -> bool {
        self.mouse_pos.x >= rect.x     &&
        self.mouse_pos.x <  rect.x + rect.w &&
        self.mouse_pos.y >= rect.y     &&
        self.mouse_pos.y <  rect.y + rect.h
    }

    pub fn button_state(&mut self, id: u64, hovered: bool, held_when_not_hovered: bool) -> ButtonState {
        let mut state = ButtonState::Idle;
        if self.hot_item.is_none() && hovered {
            self.hot_item.assign(id);
            if state == ButtonState::Idle {
                state = ButtonState::Hovered;
            }
            if self.active_item.is_none() && self.mouse_down(MouseButton::Left) {
                self.active_item.assign(id);
                state = ButtonState::Clicked;
            } 
        }
        if (self.hot_item == id || held_when_not_hovered) && self.active_item == id && state != ButtonState::Clicked {
            state = match self.mouse_down(MouseButton::Left) {
                false if hovered => ButtonState::Released,
                _     => ButtonState::Held,
            }
        }
        state
    }
}

#[derive(PartialEq, Debug)]
pub enum ButtonState {
    Clicked, Released, Held, Hovered, Idle,
}

impl Into<bool> for ButtonState {
    fn into(self) -> bool {
        self == ButtonState::Released
    }
}

// The popup window
// impl UIState {
//     pub fn new_popup(/* type */) {}
// }

// // Stores the state of the popup, mainly the fields in the 'custom' popup
// pub enum PopupState {
//     Exit{pos: Vec2}, // Are you sure you want to exit?
//     NewGame{pos: Vec2},
//     Custom{pos: Vec2, width: usize, height: usize, bomb_count: usize},
// }
// impl PopupState {
//     pub fn new()
// }