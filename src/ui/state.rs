use std::collections::HashMap;

use macroquad::{input::{get_last_key_pressed, is_key_down, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}, time::get_frame_time, window::{screen_height, screen_width}};

use super::{menubar::Menubar, minesweeper_element::MinesweeperElement, renderer::Renderer};

pub type Id = u64;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelectedItem {
    None,
    Some(Id),
    Unavailable,
}

impl SelectedItem {
    pub fn assign(&mut self, id: Id) {
        *self = Self::Some(id);
    }
    pub fn is_none(&self) -> bool {
        matches!(*self, Self::None)
    }
    pub fn assign_if_none_and(&mut self, id: Id, condition: bool) -> bool {
        match (self.is_none(), condition) {
            (true, true) => { self.assign(id); true }
            _ => false,
        }
    }
    pub fn make_unavailable_if_none_and(&mut self, condition: bool) -> bool {
        match (self.is_none(), condition) {
            (true, true) => { *self = Self::Unavailable; true }
            _ => false,
        }
    }
}

impl PartialEq<Id> for SelectedItem {
    fn eq(&self, other: &Id) -> bool {
        matches!(self, Self::Some(s) if s == other)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ButtonState {
    Disabled, Idle, Hovered, Clicked, Held, Released,
}
impl ButtonState {
    pub fn clicked(&self) -> bool {
        matches!(self, ButtonState::Clicked)
    }
    pub fn released(&self) -> bool {
        matches!(self, ButtonState::Released)
    }
}

pub struct State {
    mouse_pos: Vec2,
    mouse_buttons: HashMap<MouseButton, (bool, bool, bool)>, // (Down, Pressed, Released)
    screen_size: Vec2,
    auto_scale: bool,
    scale: f32,
    new_scale: Option<f32>,

    pub hot_item:    SelectedItem,
    pub active_item: SelectedItem,
    pub text_field: Option<Id>,
    pub tabbed: bool,
    pub caret: usize,

    key_held: Option<KeyCode>,
    key_hold_timer: f32,
}

impl State {
    pub fn new() -> State {
        State {
            mouse_pos: Vec2::default(),
            mouse_buttons: HashMap::from([
                (MouseButton::Left,   (false, false, false)),
                (MouseButton::Right,  (false, false, false)),
                (MouseButton::Middle, (false, false, false)),
            ]),
            screen_size: Vec2::default(),
            auto_scale:    true,
            scale: 2.0,
            new_scale: None,

            hot_item:     SelectedItem::None,
            active_item:  SelectedItem::None,
            text_field: None,
            tabbed: false,
            caret: 0,

            key_held: None,
            key_hold_timer: 0.0,
        }
    }

    pub fn mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }
    pub fn screen_size(&self) -> Vec2 {
        self.screen_size
    }
    pub fn scale(&self) -> f32 {
        self.scale
    }
    pub fn set_scale(&mut self, scale: f32) {
        self.new_scale = Some(scale);
    }
    pub fn auto_scale(&self) -> bool {
        self.auto_scale
    }
    pub fn set_auto_scale(&mut self, auto_scale: bool) {
        self.auto_scale = auto_scale;
    }

    fn mouse_button_values(&self, button: MouseButton) -> (bool, bool, bool) {
        match self.mouse_buttons.get(&button) {
            Some(s) => *s,
            None => (false, false, false)
        }
    }
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_button_values(button).0
    }
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_button_values(button).1
    }
    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_button_values(button).2
    }

    pub fn mouse_in_rect(&self, rect: Rect) -> bool {
        self.mouse_pos.x >= rect.x          &&
        self.mouse_pos.x <  rect.x + rect.w &&
        self.mouse_pos.y >= rect.y          &&
        self.mouse_pos.y <  rect.y + rect.h
    }

    pub fn button_state(&mut self, id: Id, hovered: bool, disabled: bool, held_when_not_hovered: bool) -> ButtonState {
        let mut state = ButtonState::Idle;
        let mouse_down = self.mouse_down(MouseButton::Left);

        if self.hot_item.assign_if_none_and(id, hovered) {
            state = match self.active_item.assign_if_none_and(id, mouse_down) {
                true  => ButtonState::Clicked,
                false => ButtonState::Hovered
            };
        }
        if (self.hot_item == id || held_when_not_hovered) && self.active_item == id && state != ButtonState::Clicked {
            state = match !mouse_down && hovered {
                true  => ButtonState::Released,
                false => ButtonState::Held,
            }
        }

        if disabled {
            state = ButtonState::Disabled;
        }
        state
    }

    pub fn begin(&mut self, menubar: &Menubar, minesweeper_element: &MinesweeperElement, renderer: &Renderer) {
        self.mouse_pos = vec2(mouse_position().0, mouse_position().1) / self.scale;

        for (&button, (down, pressed, released)) in &mut self.mouse_buttons.iter_mut() {
            *down     = is_mouse_button_down(button);
            *pressed  = is_mouse_button_pressed(button);
            *released = is_mouse_button_released(button);
        }

        let window_size = vec2(screen_width(), screen_height());
        if self.auto_scale {
            let min_fit = minesweeper_element.minimum_size(renderer) + vec2(0.0, menubar.height()) + renderer.style().background().padding * 2.0;
            self.scale = (window_size / min_fit).floor().min_element().max(1.0);
        } else if let Some(new_scale) = self.new_scale.take() {
            self.auto_scale = false;
            self.scale = new_scale;
        } 

        self.screen_size = window_size / self.scale;
        self.hot_item = SelectedItem::None;
        self.tabbed = false;

        if let Some(k) = get_last_key_pressed() {
            self.key_held = Some(k);
            self.key_hold_timer = 0.0;
        }
        if self.key_held.map(|k| is_key_down(k)) != Some(true) {
            self.key_held = None;
        }
        self.key_hold_timer += get_frame_time();
    }

    pub fn key_held(&mut self) -> Option<KeyCode> {
        match (self.key_held, self.key_hold_timer > 0.5) {
            (Some(k), true) => { self.key_hold_timer -= 0.025; Some(k) },
            _ => None
        }
    }

    pub fn finish(&mut self) {
        match self.mouse_down(MouseButton::Left) {
            false => self.active_item = SelectedItem::None,
            true if self.active_item == SelectedItem::None => self.active_item = SelectedItem::Unavailable,
            _ => {}
        };
    }
}