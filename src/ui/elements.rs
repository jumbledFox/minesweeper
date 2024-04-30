use macroquad::math::{vec2, Rect, Vec2};

use super::{renderer::{DrawShape, Renderer}, spritesheet, state::{ButtonState, Id, State}};

pub enum Align {
    // TODO: Maybe these names aren't that good
    Beg(f32), Mid(f32), End(f32),
}

pub fn align_beg(v: f32) -> Align {
    Align::Beg(v)
}
pub fn align_mid(v: f32) -> Align {
    Align::Mid(v)
}
pub fn align_end(v: f32) -> Align {
    Align::End(v)
}

impl Align {
    pub fn align_to(&self, size: f32) -> f32 {
        match *self {
            Self::Beg(v) => v,
            Self::Mid(v) => v - size / 2.0,
            Self::End(v) => v - size,
        }
    }
}

pub fn aligned_rect(x: Align, y: Align, w: f32, h: f32) -> Rect {
    let x = x.align_to(w);
    let y = y.align_to(h);
    Rect::new(x, y, w, h)
}

pub fn button(id: Id, x: Align, y: Align, w: f32, h: f32, state: &mut State, renderer: &mut Renderer) -> ButtonState {
    let rect = aligned_rect(x, y, w, h);
    let button_state = state.button_state(id, state.mouse_in_rect(rect), false, true);

    let (offset, source, text_col) = match button_state {
        ButtonState::Disabled                    => (0.0, spritesheet::BUTTON_DISABLED, spritesheet::BUTTON_TEXT_DISABLED),
        ButtonState::Held | ButtonState::Clicked => (1.0, spritesheet::BUTTON_DOWN,     spritesheet::BUTTON_TEXT),
        _                                        => (0.0, spritesheet::BUTTON_IDLE,     spritesheet::BUTTON_TEXT),
    };

    let rect = rect.offset(Vec2::splat(offset));
    renderer.draw(DrawShape::nineslice(rect, source));

    button_state
}

pub fn button_text(id: Id, text: String, x: Align, y: Align, state: &mut State, renderer: &mut Renderer) -> ButtonState {
    let button_size = renderer.text_renderer.text_size(&text, None) + vec2(6.0, 4.0);
    let rect = aligned_rect(x, y, button_size.x, button_size.y);
    let button_state = state.button_state(id, state.mouse_in_rect(rect), false, true);

    let (offset, source, text_col) = match button_state {
        ButtonState::Disabled                    => (0.0, spritesheet::BUTTON_DISABLED, spritesheet::BUTTON_TEXT_DISABLED),
        ButtonState::Held | ButtonState::Clicked => (1.0, spritesheet::BUTTON_DOWN,     spritesheet::BUTTON_TEXT),
        _                                        => (0.0, spritesheet::BUTTON_IDLE,     spritesheet::BUTTON_TEXT),
    };

    let rect = rect.offset(Vec2::splat(offset));
    renderer.draw(DrawShape::text(rect.x + 3.0, rect.y + 2.0, text, text_col));
    renderer.draw(DrawShape::nineslice(rect, source));

    button_state
}