use macroquad::{color::Color, input::{clear_input_queue, get_char_pressed}, math::{vec2, Rect, Vec2}};

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

pub fn centered_rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    aligned_rect(align_mid(x), align_mid(y), w, h)
}

pub fn text(text: String, line_gap: Option<f32>, color: Color, x: Align, y: Align, renderer: &mut Renderer) {
    let size = renderer.text_renderer.text_size(&text, line_gap);
    let rect = aligned_rect(x, y, size.x, size.y);
    renderer.draw(DrawShape::text(rect.x, rect.y, text, color));
}

// TODO: These share a lot of code
pub fn button(id: Id, x: Align, y: Align, w: f32, h: f32, disabled: bool, state: &mut State, renderer: &mut Renderer) -> ButtonState {
    let rect = aligned_rect(x, y, w, h);
    let button_state = state.button_state(id, state.mouse_in_rect(rect), disabled, true);

    let (offset, source) = match button_state {
        ButtonState::Disabled                    => (0.0, spritesheet::BUTTON_DISABLED),
        ButtonState::Held | ButtonState::Clicked => (1.0, spritesheet::BUTTON_DOWN),
        _                                        => (0.0, spritesheet::BUTTON_IDLE),
    };

    let rect = rect.offset(Vec2::splat(offset));
    renderer.draw(DrawShape::nineslice(rect, source));

    button_state
}

pub fn button_text(id: Id, text: String, x: Align, y: Align, disabled: bool, state: &mut State, renderer: &mut Renderer) -> ButtonState {
    let button_size = renderer.text_renderer.text_size(&text, None) + vec2(6.0, 4.0);
    let rect = aligned_rect(x, y, button_size.x, button_size.y);
    let button_state = state.button_state(id, state.mouse_in_rect(rect), disabled, true);

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

pub enum TextFieldKind {
    Digits{min: usize, max: usize},
}

pub fn text_field(id: Id, x: Align, y: Align, w: f32, kind: TextFieldKind, text: &mut String, hint: String, state: &mut State, renderer: &mut Renderer) {
    let lines = match kind {
        TextFieldKind::Digits {..} => 1,
    };
    let h = renderer.text_renderer.line_gap(None) * lines as f32 + 2.0;
    let rect = aligned_rect(x, y, w, h);
    let button_state = state.button_state(id, state.mouse_in_rect(rect), false, false);

    if button_state == ButtonState::Clicked {
        state.text_field = Some(id);
        // TODO: Set caret to where we clicked
        clear_input_queue();
    }

    match state.text_field {
        Some(i) if i == id => {
            state.caret = text.len().min(state.caret);
            match get_char_pressed() {
                // Left
                Some('\u{f050}') => state.caret = state.caret.saturating_sub(1).min(text.len()),
                // Right
                Some('\u{f04f}') => state.caret = state.caret.saturating_add(1).min(text.len()),
                // Backspace
                Some('\u{f02a}') if state.caret > 0 => {state.caret -= 1; text.remove(state.caret);}
                // Delete
                Some('\u{f04c}') if state.caret < text.len() => {text.remove(state.caret);}
                Some(c) if matches!(kind, TextFieldKind::Digits{..}) => {
                    if c.is_digit(10) { text.insert(state.caret, c); state.caret += 1; }
                }
                _ => {}
            }
            renderer.draw(DrawShape::text_caret(rect.x + 2.0 + renderer.text_renderer.caret_pos(&text, None, state.caret).x, rect.y + 2.0, spritesheet::BUTTON_TEXT));
        }
        _ => {
        }
    }

    if text.is_empty() {
        renderer.draw(DrawShape::text(rect.x + 2.0, rect.y + 2.0, hint, spritesheet::BUTTON_TEXT_DISABLED));
    } else {
        renderer.draw(DrawShape::text(rect.x + 2.0, rect.y + 2.0, text.clone(), spritesheet::BUTTON_TEXT));
    }
    renderer.draw(DrawShape::nineslice(rect, spritesheet::input_field(false)));
}