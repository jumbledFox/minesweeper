use macroquad::{color::Color, math::{vec2, Rect, Vec2}};

#[derive(Clone, Copy)]
pub struct Nineslice {
    pub rect: Rect,
    pub padding: f32
}
impl Nineslice {
    pub const fn new(x: f32, y: f32, w: f32, h: f32, padding: f32) -> Nineslice {
        Nineslice { rect: Rect { x, y, w, h }, padding }
    }
}

const fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { x, y, w, h }
}


pub const BUTTON_IDLE: Nineslice = Nineslice::new(84.0, 16.0, 3.0, 3.0, 1.0);
pub const BUTTON_DOWN: Nineslice = Nineslice::new(87.0, 16.0, 3.0, 3.0, 1.0);


// Menubar
pub fn menubar_idle()    -> (Color, Color) { (Color::from_hex(0xC0CBDC), Color::from_hex(0x181425)) }
pub fn menubar_hovered() -> (Color, Color) { (Color::from_hex(0x262B44), Color::from_hex(0xFFFFFF)) }
pub const DROPDOWN_BACKGROUND: Nineslice = Nineslice::new(84.0, 16.0, 3.0, 3.0, 1.0);
pub const DROPDOWN_SEPARATOR: Rect = rect(89.0, 11.0, 1.0, 2.0);
pub fn shadow() -> Color { Color::from_rgba(0, 0, 0, 128) }


// Minesweeper stuff
pub const MINEFIELD_BORDER: Nineslice = Nineslice::new(84.0, 11.0, 5.0, 5.0, 2.0);
pub const fn minefield_tile(id: usize) -> Rect {
    let id = if id > 15 { 15 } else { id };
    rect((id % 4 * 9) as f32, (id / 4 * 9) as f32, 9.0, 9.0)
}
pub const MINEFIELD_SELECTED: Rect = rect(85.0, 0.0, 11.0, 11.0);

pub const COUNTER_BACKGROUND: Nineslice = Nineslice::new(90.0, 16.0, 3.0, 3.0, 1.0);
pub const fn counter_size(digits: usize) -> Vec2 {
    vec2((digits * 10 + 4) as f32, 18.0)
}
pub const fn counter_digit(digit: Option<usize>) -> Rect {
    // If the digit is None, or larger than 9, return the blank digit
    let pos = match digit {
        // TODO: Dash if digit is 10 maybe,, or make this an enum instead of option
        Some(d) if d <= 9 => vec2((d % 5 * 8 + 36) as f32, (d / 5 * 14) as f32),
        _ => vec2(76.0, 0.0),
    };
    rect(pos.x, pos.y, 8.0, 14.0)
}

pub const TIMER_BACKGROUND: Nineslice = Nineslice::new(93.0, 16.0, 3.0, 3.0, 1.0);
pub const TIMER_SIZE: Vec2 = Vec2 { x: 21.0, y: 9.0 };
pub const fn timer_digit(digit: Option<usize>) -> Rect {
    // Darn unwrap_or() not being constant!
    let digit = if let Some(digit) = digit { if digit <= 10 { digit } else { 10 } } else { 10 };
    rect((38 + digit * 3) as f32, 28.0, 3.0, 5.0)
}
pub const fn timer_colon(lit: bool) -> Rect {
    rect(if lit { 37.0 } else { 36.0 }, 28.0, 1.0, 5.0)
}