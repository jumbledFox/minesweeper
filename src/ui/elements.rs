use macroquad::{color::Color, input::{clear_input_queue, get_char_pressed}, math::{vec2, Rect, Vec2}};

use super::{renderer::{text_renderer::Caret, DrawShape, Renderer}, spritesheet, state::{ButtonState, Id, State}};

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

// TODO: ALL of these share a lot of code
pub fn text(text: String, line_gap: Option<f32>, color: Color, x: Align, y: Align, renderer: &mut Renderer) {
    let size = renderer.text_renderer.text_size(&text, line_gap);
    let rect = aligned_rect(x, y, size.x, size.y);
    renderer.draw(DrawShape::text(rect.x, rect.y, text, line_gap, None, None, color));
}

pub fn url(id: Id, text: String, url: String, line_gap: Option<f32>, x: Align, y: Align, state: &mut State, renderer: &mut Renderer) {
    let size = renderer.text_renderer.text_size(&text, line_gap);
    let rect = aligned_rect(x, y, size.x, size.y);
    let underline = Rect::new(rect.x, rect.y + rect.h - 1.0, rect.w, 1.0);
    renderer.draw(DrawShape::text(rect.x, rect.y, text, line_gap, None, None, spritesheet::URL_TEXT));
    renderer.draw(DrawShape::rect(underline, spritesheet::URL_TEXT));

    if state.button_state(id, state.mouse_in_rect(rect), false, false).released() {
        let _ = webbrowser::open(&url);
    }
}

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
    renderer.draw(DrawShape::text(rect.x + 3.0, rect.y + 2.0, text, None, None, None, text_col));
    renderer.draw(DrawShape::nineslice(rect, source));

    button_state
}

// TODO: Rethink textfieldkind
pub enum TextFieldKind {
    Digits{min: usize, max: usize},
}

pub fn text_field(id: Id, x: Align, y: Align, w: f32, text: &mut String, hint: String, kind: TextFieldKind, max_chars: usize, state: &mut State, renderer: &mut Renderer) {
    let lines = match kind {
        TextFieldKind::Digits {..} => 1,
    };
    let h = renderer.text_renderer.line_gap(None) * lines as f32 + 2.0;
    let rect = aligned_rect(x, y, w, h);
    let button_state = state.button_state(id, state.mouse_in_rect(rect), false, false);

    if state.text_field.is_some_and(|i| i == id) {
        if let Some(click_pos) = renderer.text_click_pos {
            state.caret = click_pos;
        }
        state.caret = text.len().min(state.caret);
        let mut reset_flash = true;
        match get_char_pressed() {
            // Left
            Some('\u{f050}') => state.caret = state.caret.saturating_sub(1).min(text.len()),
            // Right
            Some('\u{f04f}') => state.caret = state.caret.saturating_add(1).min(text.len()),
            // Backspace
            Some('\u{f02a}') if state.caret > 0 => {state.caret -= 1; text.remove(state.caret);}
            // Delete
            Some('\u{f04c}') if state.caret < text.len() => {text.remove(state.caret);}
            Some(c) if matches!(kind, TextFieldKind::Digits{..}) && text.len() < max_chars => {
                if c.is_digit(10) { text.insert(state.caret, c); state.caret += 1; }
            }
            _ => reset_flash = false,
        }
        if reset_flash {
            renderer.caret_timer = 0.0;
        }
    }
    let caret = match state.text_field {
        Some(i) if i == id => Some(Caret { index: state.caret, color: spritesheet::BUTTON_TEXT }),
        _ => None,
    };
    // Doing this down here gives a frame of delay... but.... it works! Plus it's an immediate mode gui so delays are a given
    let click_pos = match button_state {
        ButtonState::Clicked => { 
            state.text_field = Some(id);
            clear_input_queue();
            Some(state.mouse_pos())
        },
        _ => None,
    };
    if text.is_empty() {
        renderer.draw(DrawShape::text(rect.x + 2.0, rect.y + 2.0, hint, None, caret, click_pos, spritesheet::BUTTON_TEXT_DISABLED));
    } else {
        renderer.draw(DrawShape::text(rect.x + 2.0, rect.y + 2.0, text.clone(), None, caret, click_pos, spritesheet::BUTTON_TEXT));
    }
    renderer.draw(DrawShape::nineslice(rect, spritesheet::input_field(false)));
}