use macroquad::{color::Color, input::{clear_input_queue, get_char_pressed, get_last_key_pressed, KeyCode}, math::{vec2, Rect}};

use super::{renderer::{text_renderer::Caret, DrawShape, Renderer}, state::{ButtonState, Id, State}};

#[derive(Clone, Copy)]
pub enum Align {
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
    
    renderer.draw(DrawShape::text(rect.x, rect.y, text, line_gap, None, None, renderer.style().text_url()));
    renderer.draw(DrawShape::rect(underline, renderer.style().text_url()));

    if state.button_state(id, state.mouse_in_rect(rect), false, false).released() {
        let _ = webbrowser::open(&url);
    }
}

pub fn button(id: Id, x: Align, y: Align, w: f32, h: f32, disabled: bool, state: &mut State, renderer: &mut Renderer) -> ButtonState {
    let rect = aligned_rect(x, y, w, h);
    let button_state = state.button_state(id, state.mouse_in_rect(rect), disabled, true);

    let (offset, source, _) = renderer.style().button(&button_state);
    renderer.draw(DrawShape::nineslice(rect.offset(offset), source));

    button_state
}

pub fn button_text(id: Id, text: String, x: Align, y: Align, disabled: bool, state: &mut State, renderer: &mut Renderer) -> ButtonState {
    let button_size = renderer.text_renderer.text_size(&text, None) + vec2(6.0, 4.0);
    let rect = aligned_rect(x, y, button_size.x, button_size.y);
    let button_state = state.button_state(id, state.mouse_in_rect(rect), disabled, true);

    let (offset, source, text_col) = renderer.style().button(&button_state);

    let rect = rect.offset(offset);
    renderer.draw(DrawShape::text(rect.x + 3.0 , rect.y + 2.0, text, None, None, None, text_col));
    renderer.draw(DrawShape::nineslice(rect, source));

    button_state
}

pub fn number_field(id: Id, number: &mut String, max: usize, hint: String, x: Align, y: Align, w: f32, state: &mut State, renderer: &mut Renderer) {
    let h = renderer.text_renderer.line_gap(None) + 2.0;
    let rect = aligned_rect(x, y, w, h);

    let button_state = state.button_state(id, state.mouse_in_rect(rect), false, false);

    if state.text_field.is_some_and(|i| i == id) {
        if let Some(click_pos) = renderer.text_click_pos {
            state.caret = click_pos;
        }
        state.caret = state.caret.min(number.len());

        let mut reset_flash = 0;
        match get_char_pressed() {
            Some(c) if c.is_digit(10) && number.len() < max => { number.insert(state.caret, c); state.caret += 1; }
            _ => reset_flash += 1,
        }

        match get_last_key_pressed().or_else(|| state.key_held()) {
            // Left
            Some(KeyCode::Left)  => state.caret = state.caret.saturating_sub(1).min(number.len()),
            // Right
            Some(KeyCode::Right) => state.caret = state.caret.saturating_add(1).min(number.len()),
            // Backspace
            Some(KeyCode::Backspace) if state.caret > 0 => {state.caret -= 1; number.remove(state.caret);}
            // Delete
            Some(KeyCode::Delete) if state.caret < number.len() => { number.remove(state.caret); }
            _ => reset_flash += 1,
        }
        if reset_flash != 2 {
            renderer.caret_timer = 0.0;
        }
    }

    let caret = match state.text_field {
        Some(i) if i == id => Some(Caret { index: state.caret, color: renderer.style().text() }),
        _ => None
    };

    let click_pos = match button_state {
        ButtonState::Clicked => {
            state.text_field = Some(id);
            clear_input_queue();
            Some(state.mouse_pos())
        },
        _ => None,
    };

    let text = match number.is_empty() {
        true  => DrawShape::text(rect.x + 2.0, rect.y + 2.0, hint,           None, caret, click_pos, renderer.style().text_disabled()),
        false => DrawShape::text(rect.x + 2.0, rect.y + 2.0, number.clone(), None, caret, click_pos, renderer.style().text()),
    };

    renderer.draw(text);
    renderer.draw(DrawShape::nineslice(rect, renderer.style().text_input()));
}