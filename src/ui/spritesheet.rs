use macroquad::{color::Color, color_u8, math::{vec2, Rect, Vec2}};

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


pub const BACKGROUND: Nineslice = Nineslice::new(84.0, 0.0, 3.0, 3.0, 1.0);
pub const BUTTON_IDLE:     Nineslice = Nineslice::new(84.0, 3.0, 3.0, 3.0, 1.0);
pub const BUTTON_DOWN:     Nineslice = Nineslice::new(87.0, 3.0, 3.0, 3.0, 1.0);
pub const BUTTON_DISABLED: Nineslice = Nineslice::new(90.0, 3.0, 3.0, 3.0, 1.0);
pub const BUTTON_TEXT:          Color = color_u8!( 24,  20,  37, 255);
pub const BUTTON_TEXT_DISABLED: Color = color_u8!(139, 155, 180, 255);
pub const SHADOW: Color = color_u8!(0, 0, 0, 128);
pub const fn input_field(error: bool) -> Nineslice {
    match error {
        false => Nineslice::new(84.0, 6.0, 3.0, 3.0, 1.0),
        true  => Nineslice::new(87.0, 6.0, 3.0, 3.0, 1.0),
    }
}

// Menubar
pub fn menubar_colors(hovered: bool) -> (Color, Color) {
    match hovered {
        false => (Color::from_hex(0xC0CBDC), Color::from_hex(0x181425)),
        true  => (Color::from_hex(0x262B44), Color::from_hex(0xFFFFFF)),
    }
}
pub const DROPDOWN_BACKGROUND: Nineslice = Nineslice::new(84.0, 9.0, 3.0, 3.0, 1.0);
pub const DROPDOWN_SEPARATOR: Rect = rect(87.0, 9.0, 1.0, 2.0);

// Popups
pub const POPUP_TITLE: Nineslice = Nineslice::new(84.0, 12.0, 3.0, 3.0, 1.0);
pub const POPUP_BODY:  Nineslice = Nineslice::new(87.0, 12.0, 3.0, 3.0, 1.0);
pub const POPUP_CLOSE: Rect = rect(90.0, 12.0, 3.0, 3.0);
pub fn popup_close_color(hovered: bool) -> Color {
    match hovered {
        true  => color_u8!( 58,  68, 102, 255),
        false => color_u8!(255, 255, 255, 255),
    }
}
pub const POPUP_TITLE_TEXT: Color = color_u8!(255, 255, 255, 255);
pub const POPUP_BODY_TEXT : Color = color_u8!( 24,  20,  37, 255);


// Minesweeper stuff
pub const MINEFIELD_BORDER: Nineslice = Nineslice::new(84.0, 15.0, 5.0, 5.0, 2.0);
pub const fn minefield_tile(id: usize) -> Rect {
    let id = if id > 15 { 15 } else { id };
    rect((id % 4 * 9) as f32, (id / 4 * 9) as f32, 9.0, 9.0)
}
pub const MINEFIELD_SELECTED: Rect = rect(84.0, 20.0, 11.0, 11.0);

// The bomb counter
pub const COUNTER_BACKGROUND: Nineslice = Nineslice::new(89.0, 15.0, 3.0, 3.0, 1.0);
pub const fn counter_size(digits: usize) -> Vec2 {
    vec2((digits * 10 + 4) as f32, 18.0)
}
pub enum CounterDigit {
    Empty, Dash, Digit(usize)
}
pub const fn counter_digit(digit: CounterDigit) -> Rect {
    // If the digit is None, or larger than 9, return the blank digit
    let pos = match digit {
        CounterDigit::Digit(d) if d <= 9 => vec2((d % 5 * 8 + 36) as f32, (d / 5 * 14) as f32),
        CounterDigit::Dash => vec2(76.0, 14.0),
        _ => vec2(76.0, 0.0),
    };
    rect(pos.x, pos.y, 8.0, 14.0)
}

// The timer
pub const TIMER_BACKGROUND: Nineslice = Nineslice::new(92.0, 15.0, 3.0, 3.0, 1.0);
pub const TIMER_SIZE: Vec2 = Vec2 { x: 21.0, y: 9.0 };
pub const fn timer_digit(digit: Option<usize>) -> Rect {
    // Darn unwrap_or() not being constant!
    let digit = if let Some(digit) = digit { if digit <= 10 { digit } else { 10 } } else { 10 };
    rect((38 + digit * 3) as f32, 28.0, 3.0, 5.0)
}
pub const fn timer_colon(lit: bool) -> Rect {
    rect(if lit { 37.0 } else { 36.0 }, 28.0, 1.0, 5.0)
}