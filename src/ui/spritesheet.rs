use macroquad::{color::Color, math::Rect};

// TODO: Should this be called Nineslice or NinesliceSource? 
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

// Minesweeper
pub const MINEFIELD_BORDER: Nineslice = Nineslice::new(84.0, 11.0, 5.0, 5.0, 2.0);
pub const fn minefield_tile(id: usize) -> Rect {
    let id = if id <= 15 { id } else { 15 };
    rect((id % 4 * 9) as f32, (id / 4 * 9) as f32, 9.0, 9.0)
}
pub const MINEFIELD_SELECTED: Rect = rect(85.0, 0.0, 11.0, 11.0);

pub const COUNTER_BACKGROUND: Nineslice = Nineslice::new(90.0, 16.0, 3.0, 3.0, 1.0);
pub const TIMER_BACKGROUND: Nineslice = Nineslice::new(93.0, 16.0, 3.0, 3.0, 1.0);
// counter_digit_0: Rect::new(36.0, 0.0, 8.0, 14.0),