// ELEMENTS are things that make up the game window, they often have an update() and draw() function

use ggez::{glam::Vec2, graphics::{Canvas, Color, Rect}};

pub mod text_renderer;
pub mod button;
pub mod menubar;
pub mod minesweeper_element;

pub const TEXT_IDLE         : Color = Color { r:  24.0 / 255.0, g:  20.0 / 255.0, b:  37.0 / 255.0, a: 1.0 };
pub const TEXT_DISABLED     : Color = Color { r:  90.0 / 255.0, g: 105.0 / 255.0, b: 136.0 / 255.0, a: 1.0 };
pub const BACKGROUND_IDLE   : Color = Color { r: 192.0 / 255.0, g: 203.0 / 255.0, b: 220.0 / 255.0, a: 1.0 };
pub const BACKGROUND_HOVERED: Color = Color { r:  38.0 / 255.0, g:  43.0 / 255.0, b:  68.0 / 255.0, a: 1.0 };

pub enum MouseAction {
    None,
    Press,
    Release,
}

pub fn draw_9_slice(canvas: &mut Canvas) {
    ()
}

// Makes a new rect that's in the middle of a point
fn rect_at_middle(point: Vec2, width: f32, height: f32) -> Rect {
    Rect::new(
        point.x - (width  / 2.0),
        point.y - (height / 2.0),
        width,
        height,
    )
}
// Rounds a rect
fn round_rect(rect: Rect) -> Rect {
    Rect::new(rect.x.round(), rect.y.round(), rect.w.round(), rect.h.round())
}