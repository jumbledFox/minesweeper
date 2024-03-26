// ELEMENTS are things that make up the game window, they often have an update() and draw() function

use ggez::graphics::{Canvas, Color};

pub mod text_renderer;
pub mod button;
pub mod menubar;
pub mod minesweeper_element;

pub const TEXT_IDLE         : Color = Color { r:  24.0 / 255.0, g:  20.0 / 255.0, b:  37.0 / 255.0, a: 1.0 };
pub const TEXT_DISABLED     : Color = Color { r:  90.0 / 255.0, g: 105.0 / 255.0, b: 136.0 / 255.0, a: 1.0 };
pub const BACKGROUND_IDLE   : Color = Color { r: 192.0 / 255.0, g: 203.0 / 255.0, b: 220.0 / 255.0, a: 1.0 };
pub const BACKGROUND_HOVERED: Color = Color { r:  38.0 / 255.0, g:  43.0 / 255.0, b:  68.0 / 255.0, a: 1.0 };

pub fn draw_9_slice(canvas: &mut Canvas) {
    ()
}