use macroquad::{input::{clear_input_queue, get_char_pressed}, math::{vec2, Rect, Vec2}};

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

// TODO: These share a lot of code
pub fn button(id: Id, x: Align, y: Align, w: f32, h: f32, disabled: bool, state: &mut State, renderer: &mut Renderer) -> ButtonState {
    let rect = aligned_rect(x, y, w, h);
    let button_state = state.button_state(id, None, None, state.mouse_in_rect(rect), disabled, true);

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
    let button_state = state.button_state(id, None, None, state.mouse_in_rect(rect), disabled, true);

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

pub fn text_field(id: Id, x: Align, y: Align, w: f32, kind: TextFieldKind, text: &mut String, hint: String, next_id: Option<Id>, state: &mut State, renderer: &mut Renderer) {
    let lines = match kind {
        TextFieldKind::Digits {..} => 1,
    };
    let h = renderer.text_renderer.line_gap(None) * lines as f32 + 2.0;
    let rect = aligned_rect(x, y, w, h);
    let button_state = state.button_state(id, None, None, state.mouse_in_rect(rect), false, false);

    // if button_state == ButtonState::Clicked {
    //     state.text_field = SelectedTextField::Some { id, caret: 0 };
    //     clear_input_queue();
    // }
    // println!("{:?}", get_char_pressed());
    // TODO: Maybe keyboard support for moving?!?
    // TODO: This isn't that good...
    // let mut next_field = None;
    // match &mut state.text_field {
    //     SelectedTextField::Some { id: t_id, caret } if *t_id == id => {
    //         *caret = text.len().min(*caret);
    //         match get_char_pressed() {
    //             // Tab
    //             Some('\u{f02b}') => {
    //                 if let Some(next_id) = next_id {
    //                     next_field = Some(SelectedTextField::Some { id: next_id, caret: usize::MAX });
    //                 }
    //             }
    //             // Left
    //             Some('\u{f050}') => *caret = caret.saturating_sub(1).min(text.len()),
    //             // Right
    //             Some('\u{f04f}') => *caret = caret.saturating_add(1).min(text.len()),
    //             // Backspace
    //             Some('\u{f02a}') if *caret > 0 => {text.remove(*caret-1);}
    //             // Delete
    //             Some('\u{f04c}') if *caret < text.len() => {text.remove(*caret);}
    //             Some(c) if matches!(kind, TextFieldKind::Digits{..}) => {
    //                 if c.is_digit(10) { text.insert(*caret, c); *caret += 1; }
    //             }
    //             _ => {},
    //         }
    //         renderer.draw(DrawShape::text_caret(rect.x + 2.0 + renderer.text_renderer.caret_pos(&text, None, *caret).x, rect.y + 2.0, spritesheet::BUTTON_TEXT));
    //     }
    //     _ => {
    //     }
    // }

    // if let Some(next_field) = next_field {
    //     state.text_field = next_field;
    // }

    if text.is_empty() {
        renderer.draw(DrawShape::text(rect.x + 2.0, rect.y + 2.0, hint, spritesheet::BUTTON_TEXT_DISABLED));
    } else {
        renderer.draw(DrawShape::text(rect.x + 2.0, rect.y + 2.0, text.clone(), spritesheet::BUTTON_TEXT));
    }
    renderer.draw(DrawShape::nineslice(rect, spritesheet::input_field(false)));
}