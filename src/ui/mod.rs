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


use crate::minesweeper::Difficulty;

use self::minesweeper::MinesweeperUI;
use self::spritesheet::Nineslice;
use self::text_renderer::TextRenderer;

pub enum DrawShape {
    Label{x: f32, y: f32, text: String, color: Color},
    Rect{x: f32, y: f32, w: f32, h: f32, color: Color},
    Nineslice{dest: Rect, source: Rect, padding: f32},
    Image{x: f32, y: f32, source: Rect, color: Color},
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
    pub fn image(x: f32, y: f32, source: Rect, color: Option<Color>) -> Self {
        Self::Image { x, y, source, color: color.unwrap_or(WHITE) }
    }
    pub fn image_rect(dest: Rect, source: Rect) -> Self {
        Self::ImageRect { dest, source }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn assign_if_none_and(&mut self, id: u64, condition: bool) -> bool {
        match self.is_none() && condition {
            true  => {self.assign(id); true}
            false => false,
        }
    }
    pub fn assign(&mut self, id: u64) {
        *self = SelectedItem::Some(id);
    }
}

pub enum Align {
    Min, Mid, Max,
}

impl Align {
    pub fn align(rect: Rect, h_align: Align, v_align: Align) -> Rect {
        let x = rect.x - match h_align {
            Align::Min => 0.0,
            Align::Mid => rect.w / 2.0,
            Align::Max => rect.w,
        };
        let y = rect.y - match v_align {
            Align::Min  => 0.0,
            Align::Mid  => rect.h / 2.0,
            Align::Max  => rect.h,
        };
        Rect::new(x, y, rect.w, rect.h)
    }
}



pub struct UIState {
    mouse_pos: Vec2,
    mouse_buttons:    HashMap<MouseButton, (bool, bool)>, // (Down, Pressed)
    screen_size: Vec2,
    hot_item: SelectedItem,
    active_item: SelectedItem,

    popups: Vec<Popup>,
    popup_drag_offset: Vec2,

    text_renderer: TextRenderer,
    draw_queue: Vec<DrawShape>,

    texture: Texture2D,
}

impl UIState {
    pub async fn new() -> UIState {
        let texture = Texture2D::from_file_with_format(include_bytes!("../../resources/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);
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

            popups: vec![],
            popup_drag_offset: Vec2::ZERO,

            text_renderer: TextRenderer::new(),
            draw_queue: vec![],

            texture,
        }
    }

    // Some getters and helper functions
    pub fn draw_queue(&mut self) -> &mut Vec<DrawShape> {
        &mut self.draw_queue
    }
    pub fn screen_size(&self) -> Vec2 {
        self.screen_size
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
        if self.hot_item.assign_if_none_and(id, hovered) {
            if state == ButtonState::Idle {
                state = ButtonState::Hovered;
            }
            if self.active_item.assign_if_none_and(id, self.mouse_down(MouseButton::Left)) {
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

    // TODO: Maybe make struct with position and alignment
    pub fn label(&mut self, label: String, color: Color, pos: Vec2, align: (Align, Align)) {
        let size = self.text_renderer.text_size(&label, None);
        let pos = Align::align(Rect::new(pos.x, pos.y, size.x, size.y), align.0, align.1).point();

        self.draw_queue.push(DrawShape::label(pos.x, pos.y, label, color));
    }
    pub fn button(&mut self, id: u64, label: String, disabled: bool, pos: Vec2, align: (Align, Align)) -> ButtonState {
        let size = self.text_renderer.text_size(&label, None) + vec2(6.0, 4.0);
        let rect = Align::align(Rect::new(pos.x, pos.y, size.x, size.y), align.0, align.1);

        let state = self.button_state(id, self.mouse_in_rect(rect), true);
        
        let (offset, source, text_col) = match (&state, disabled) {
            (_, true) => (0.0, spritesheet::BUTTON_DISABLED, spritesheet::BUTTON_TEXT_DISABLED),
            (ButtonState::Held | ButtonState::Clicked | ButtonState::Released, _) => (1.0, spritesheet::BUTTON_DOWN, spritesheet::BUTTON_TEXT),
            _ => (0.0, spritesheet::BUTTON_IDLE, spritesheet::BUTTON_TEXT),
        };

        self.draw_queue.push(DrawShape::label(rect.x + offset + 2.0, rect.y + offset + 2.0, label, text_col));
        self.draw_queue.push(DrawShape::nineslice(rect.offset(Vec2::splat(offset)), source));
        if !disabled { state } else { ButtonState::Idle }
    }
    pub fn input_field(&mut self, id: u64, index: usize, text: &mut String, hint: String, error: bool, pos: Vec2, size: Vec2, align: (Align, Align)) {
        let rect = Align::align(Rect::new(pos.x, pos.y, size.x, size.y), align.0, align.1);
        
        if let Some(c) = get_char_pressed() {

        }

        let draw_shape = match text.is_empty() {
            true  => DrawShape::label(rect.x+2.0, rect.y+2.0, hint, spritesheet::BUTTON_TEXT_DISABLED),
            false => DrawShape::label(rect.x+2.0, rect.y+2.0, text.clone(), spritesheet::BUTTON_TEXT),
        };
        self.draw_queue.push(draw_shape);
        self.draw_queue.push(DrawShape::nineslice(rect, spritesheet::input_field(error)));
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
                &DrawShape::Image { x, y, source, color } => {
                    let params = DrawTextureParams {
                        source: Some(source.round()),
                        ..Default::default()
                    };
                    draw_texture_ex(&self.texture, x.round(), y.round(), color, params);
                },
            }
        }
    }
    
    pub fn new_game_popup(&mut self, new_difficulty: Difficulty, difficulty: &mut Difficulty, minesweeper_ui: &mut MinesweeperUI) {
        if minesweeper_ui.game_in_progress() {
            self.add_popup(PopupKind::NewGame { difficulty: new_difficulty })
        } else {
            *difficulty = new_difficulty;
            minesweeper_ui.new_game(new_difficulty);
        }
    }

    pub fn add_popup(&mut self, kind: PopupKind) {
        self.popups.retain(|p| std::mem::discriminant(&p.kind) != std::mem::discriminant(&kind));
        self.popups.push(Popup::new(kind, self.screen_size));
    }

    fn popup(&mut self, index: usize, menubar_height: f32) -> (PopupReturn, bool, bool) {
        let mut popup = match self.popups.get_mut(index) {
            Some(p) => p.clone(),
            None => return (PopupReturn::None, false, false),
        };

        let title = String::from(match popup.kind {
            PopupKind::Win         => "You Win!",
            PopupKind::Lose        => "You Lose",
            PopupKind::NewGame{..} => "Warning!",
            PopupKind::Custom      => "Custom...",
            PopupKind::About       => "About",
        });

        let titlebar_height = self.text_renderer.text_size(&title, None).y + 3.0;
        popup.pos = popup.pos.min(self.screen_size - popup.size).max(vec2(0.0, menubar_height)).round();

        let titlebar = Rect::new(popup.pos.x, popup.pos.y, popup.size.x, titlebar_height);
        let body = Rect::new(popup.pos.x, popup.pos.y + titlebar.h, popup.size.x, popup.size.y - titlebar.h);


        let id = 1966292073u64.wrapping_add(popup.hash);
        
        let mut return_state = PopupReturn::None;
        let mut close = false;
        let active_before = self.active_item;

        // The close button
        {
            let close_id = hash_string(&"closebutton".to_owned()).wrapping_add(id);
            let close_pos = titlebar.point() + vec2(titlebar.w - 8.0, 1.0);
            let close_rect = Rect::new(close_pos.x, close_pos.y, 7.0, 7.0);
            let close_state = self.button_state(close_id, self.mouse_in_rect(close_rect), false);
            self.hot_item.assign_if_none_and(id, close_state == ButtonState::Hovered);
            self.hot_item.assign_if_none_and(id, close_state == ButtonState::Clicked);
            close = close || close_state == ButtonState::Released;
            
            self.draw_queue.push(DrawShape::image(
                close_pos.x + 2.0,
                close_pos.y + 2.0,
                spritesheet::POPUP_CLOSE,
                Some(spritesheet::popup_close_color(close_state != ButtonState::Idle))
            ));
            self.draw_queue.push(DrawShape::rect(close_rect, spritesheet::popup_close_color(close_state == ButtonState::Idle)))
            
            // spritesheet::popup_close_color(hovered)
        }
        
        // Elements inside the popup
        let bottom_right = body.point() + body.size() - 3.0;
        let text_col = spritesheet::POPUP_BODY_TEXT;
        match popup.kind {
            PopupKind::Win => {
                close = close || self.button(id.wrapping_add(1), String::from("Yippee!"), false, bottom_right, (Align::Max, Align::Max)).into();
            }
            PopupKind::Lose => {
                close = close || self.button(id.wrapping_add(1), String::from("Awwww"), false, bottom_right, (Align::Max, Align::Max)).into();
            }
            PopupKind::NewGame { difficulty } => {
                self.label(
                    String::from("Are you sure you want\nto start a new game?"),
                    text_col, body.point() + 3.0, (Align::Min, Align::Min)
                );
                close = close || self.button(id.wrapping_add(1), String::from("Cancel"), false, bottom_right - vec2(21.0, 0.0), (Align::Max, Align::Max)).into();
                if self.button(id.wrapping_add(2), String::from("Yes"), false, bottom_right, (Align::Max, Align::Max)).into() {
                    return_state = PopupReturn::NewGame { difficulty };
                    close = true;
                }
            }
            PopupKind::Custom => {
                // TODO: This is all fucking shit, the whole thing
                for (i, &l) in ["Width:", "Height:", "Bombs:"].iter().enumerate() {
                    self.label(String::from(l), text_col, body.point() + vec2(30.0, 4.0 + (i * 11) as f32), (Align::Max, Align::Min));
                    // self.button(id + 223 + i as u64, String::from("_______"), false, body.point() + vec2(34.0, 1.0 + (i * 11) as f32), (Align::Min, Align::Min));
                    self.input_field(id, index, &mut String::new(), "4 - 200".to_string(), false, body.point() + vec2(34.0, 1.0 + (i * 11) as f32), vec2(30.0, 10.0), (Align::Min, Align::Min))
                }
                close = close || self.button(id.wrapping_add(1), String::from("Cancel"), false, bottom_right - vec2(33.0, 0.0), (Align::Max, Align::Max)).into();
                if self.button(id.wrapping_add(2), String::from("Submit"), true, bottom_right, (Align::Max, Align::Max)).into() {
                    return_state = PopupReturn::Custom { width: 200, height: 100, bomb_count: 2000 };
                    close = true;
                }
            }
            PopupKind::About => {

            }
        }

        // Dragging the popup and hovering over the body
        let hovered = self.mouse_in_rect(titlebar) || self.mouse_in_rect(body);
        if self.hot_item.assign_if_none_and(id, hovered) {
            if self.active_item.assign_if_none_and(id, self.mouse_down(MouseButton::Left) && hovered) {
                self.popup_drag_offset = self.mouse_pos - popup.pos; 
            }
        }

        if self.active_item == id {
            // TODO: Can hover over other things when dragging
            popup.pos = self.mouse_pos - self.popup_drag_offset;
        }

        self.draw_queue.extend(vec![
            DrawShape::label(titlebar.x + 2.0, titlebar.y + 2.0, title, spritesheet::POPUP_TITLE_TEXT),
            DrawShape::nineslice(titlebar, spritesheet::POPUP_TITLE),
            DrawShape::nineslice(body, spritesheet::POPUP_BODY),
            DrawShape::rect(body.combine_with(titlebar).offset(Vec2::splat(3.0)), spritesheet::SHADOW)
        ]);

        self.popups[index] = popup;
        (return_state, close, active_before.is_none() && !self.active_item.is_none())
    }

    pub fn popups(&mut self, menubar_height: f32) -> Vec<PopupReturn> {
        let mut popup_returns = Vec::new();
        let mut put_on_top: Option<usize> = None;
        for p in (0..self.popups.len()).rev() {
            let (popup_return, close, t) = self.popup(p, menubar_height);
            popup_returns.push(popup_return);
            if close {
                self.popups.remove(p);
            }
            if t {
                put_on_top = Some(p);
            }
        }
        if let Some(top_index) = put_on_top {
            let p = self.popups.remove(top_index);
            self.popups.push(p);
        }
        popup_returns
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ButtonState {
    Clicked, Released, Held, Hovered, Idle,
}
impl Into<bool> for ButtonState {
    fn into(self) -> bool {
        self == ButtonState::Released
    }
}

#[derive(Clone)]
pub enum PopupKind {
    Win,
    Lose,
    NewGame { difficulty: Difficulty },
    Custom,
    About,
}
// Things that persist across frames for the popup
#[derive(Clone)]
pub enum PopupState {
    Default,
    Custom { width: String, height: String, bomb_count: String },
}
// The return-type of the popup
pub enum PopupReturn {
    None,
    Custom { width: usize, height: usize, bomb_count: usize },
    NewGame { difficulty: Difficulty },
}
#[derive(Clone)]
pub struct Popup {
    pub pos: Vec2,
    pub size: Vec2,
    pub kind: PopupKind,
    pub state: PopupState,
    pub hash: u64,
}

impl Popup {
    // Makes an new popup and puts it in the middle
    pub fn new(kind: PopupKind, screen_size: Vec2) -> Popup {
        let (size, state) = match kind {
            PopupKind::Win         => (vec2( 60.0, 40.0), PopupState::Default),
            PopupKind::Lose        => (vec2( 60.0, 40.0), PopupState::Default),
            PopupKind::NewGame{..} => (vec2( 90.0, 40.0), PopupState::Default),
            PopupKind::Custom      => (vec2( 78.0, 58.0), PopupState::Custom { width: String::new(), height: String::new(), bomb_count: String::new() }),
            PopupKind::About       => (vec2(100.0, 80.0), PopupState::Default),
        };
        Popup {
            pos: (screen_size - size) / 2.0,
            size,
            kind,
            state,
            hash: macroquad::rand::RandomRange::gen_range(0, u64::MAX),
        }
    }
}