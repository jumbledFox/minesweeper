use std::hash::{DefaultHasher, Hash, Hasher};

// TODO:
// I'd like to have a stack of Windows, which are just an ID and a rect. It could be reordered which would be good,
// I can use the last frames window stack to see if I should interact with a window / elements or not

pub fn hash_string(input: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

use macroquad::prelude::*;
use macroquad::math::Vec2;

pub mod menubar;
pub mod text_renderer;

use self::text_renderer::TextRenderer;
use self::menubar::*;

pub struct Style {
    pub button_idle_source: NinesliceSource,
    pub button_down_source: NinesliceSource,
    
    pub menubar_idle: (Color, Color),
    pub menubar_hovered: (Color, Color),
    pub dropdown_bg_source: NinesliceSource,
    pub separator_source: Rect,
    pub shadow_col: Color,
}


pub enum DrawShape {
    Label{x: f32, y: f32, text: String, color: Color},
    Rect{x: f32, y: f32, w: f32, h: f32, color: Color},
    Nineslice{dest: Rect, source: Rect, padding: f32},
    ImageRect{dest: Rect, source: Rect},
}

impl DrawShape {
    pub fn label(x: f32, y: f32, text: String, color: Color) -> Self {
        Self::Label { x, y, text, color }
    }
    pub fn rect(rect: Rect, color: Color) -> Self {
        Self::Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h, color }
    }
    pub fn nineslice(dest: Rect, source: (Rect, f32)) -> Self {
        Self::Nineslice { dest, source: source.0, padding: source.1 }
    }
    pub fn image_rect(dest: Rect, source: Rect) -> Self {
        Self::ImageRect { dest, source }
    }
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

impl SelectedItem {
    pub fn assign(&mut self, id: u64) {
        *self = SelectedItem::Some(id);
    }
}

type NinesliceSource = (Rect, f32);
pub struct UIState {
    style: Style,

    mouse_pos: Vec2,
    mouse_down: bool,
    mouse_pressed: bool,
    screen_size: Vec2,
    hot_item: SelectedItem,
    active_item: SelectedItem,

    menubar: Menubar,

    text_renderer: TextRenderer,
    drawqueue: Vec<DrawShape>,

    texture: Texture2D,
}

impl UIState {
    pub fn new(texture: Texture2D, style: Style) -> UIState {
        UIState {
            style,

            mouse_pos: Vec2::ZERO,
            mouse_down: false,
            mouse_pressed: false,
            screen_size: Vec2::ONE,
            hot_item: SelectedItem::None,
            active_item: SelectedItem::None,

            menubar: Menubar::default(),

            text_renderer: TextRenderer::new(),
            drawqueue: vec![],

            texture,
        }
    }

    pub fn begin(&mut self, scale: f32) {
        self.mouse_pos = Vec2::new(mouse_position().0, mouse_position().1) / scale;
        
        let mouse_down_prev = self.mouse_down;
        self.mouse_down = is_mouse_button_down(MouseButton::Left);
        self.mouse_pressed = self.mouse_down && !mouse_down_prev;

        self.screen_size = Vec2::new(screen_width(), screen_height()) / scale;

        self.hot_item = SelectedItem::None;
        self.menubar.reset();
        self.drawqueue = Vec::new();

        self.deselect_menubar();
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
                // TODO: fix differing &s
                DrawShape::Label { x, y, text, color } => self.text_renderer.draw_text(text, *x, *y, *color, None),
                &DrawShape::Rect { x, y, w, h, color } => draw_rectangle(x, y, w, h, color),
                &DrawShape::ImageRect { dest, source } => {
                    let params = DrawTextureParams {
                        dest_size: Some(dest.size()),
                        source: Some(source),
                        ..Default::default()
                    };
                    draw_texture_ex(&self.texture, dest.x, dest.y, WHITE, params);
                }
                &DrawShape::Nineslice { dest, source, padding } => {
                    fn calculate_parts(rect: Rect, pad: f32) -> [Rect; 9] {
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
            }
        }
    }

    pub fn mouse_in_rect(&self, rect: Rect) -> bool {
        self.mouse_pos.x >= rect.x     &&
        self.mouse_pos.x <  rect.x + rect.w &&
        self.mouse_pos.y >= rect.y     &&
        self.mouse_pos.y <  rect.y + rect.h
    }
    // TODO:
    pub fn mouse_in_rect2(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
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

impl Into<bool> for ButtonState {
    fn into(self) -> bool {
        self == ButtonState::Released
    }
}

pub enum TextAlignment {
    Center,
    Left(f32),
    // TODO: Right(f32),
}

impl UIState {
    pub fn button(&mut self, text: String, align: TextAlignment, x: f32, y: f32, w: f32, h: f32) -> ButtonState {
        let id = hash_string(&text);

        let mut clicked = false;
        // Detect whether the button should be hot / active
        if self.hot_item == SelectedItem::None && self.mouse_in_rect2(x, y, w, h) {
            self.hot_item = SelectedItem::Some(id);
            if self.active_item == SelectedItem::None && self.mouse_down {
                self.active_item = SelectedItem::Some(id);
                clicked = true;
            }
        }

        // Rendering
        let (rect_x, rect_y, (col, tex_col), state) = match (&self.hot_item, &self.active_item) {
            // Hot and active (Held)
            (hot_item, active_item) if *hot_item == id && *active_item == id => (x + 2.0, y + 2.0, self.style.menubar_hovered, ButtonState::Held),
            // Hot, but not active (Hovered)
            (hot_item, _) if *hot_item == id => (x, y, self.style.menubar_hovered, ButtonState::Hovered),
            // Otherwise
            _  => (x, y, self.style.menubar_idle, ButtonState::Idle),
        };

        let label_rel_pos = match align {
            TextAlignment::Center => (Vec2::new(w, h)-self.text_renderer.text_size(&text, None))/2.0,
            TextAlignment::Left(gap) => Vec2::new(gap, (h - self.text_renderer.text_size(&text, None).y) / 2.0),
        };

        self.drawqueue.push(DrawShape::Label { x: rect_x + label_rel_pos.x, y: rect_y + label_rel_pos.y, text, color: tex_col });
        self.drawqueue.push(DrawShape::Rect { x: rect_x, y: rect_y, w, h, color: col });

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

    pub fn checkbox(&mut self, label: String, value: &mut bool, x: f32, y: f32, w: f32, h: f32) -> bool {
        let drawqueue_before_index = self.drawqueue.len();

        let offset = match self.button(label, TextAlignment::Center, x, y, w, h) {
            ButtonState::Held => 2.0,
            ButtonState::Released => {*value = !*value; 0.0}
            _ => 0.0
        };
        if *value {
            self.drawqueue.insert(drawqueue_before_index, DrawShape::Rect { x: x+w/4.0+offset, y: y+h/4.0+offset, w: w*(2.0/4.0), h: h*(2.0/4.0), color: Color::from_hex(0x333333) });
        }
        *value
    }
}
