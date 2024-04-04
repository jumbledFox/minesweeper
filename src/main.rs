use macroquad::{miniquad::window::request_quit, prelude::*};
use ui::UIState;

pub mod ui;
pub mod minesweeper;

#[macroquad::main("Minesweeper")]
async fn main() {
    let mut ui = UIState::new();
    let mut c = true;

    loop {
        clear_background(Color::from_hex(0x756853));

        ui.prepare();
        
        ui.checkbox(0, &mut c, 10.0, 10.0, 20.0, 20.0);
        if c {
            if ui.button(1, 10.0, 40.0, 50.0, 20.0) {
                println!("hello");
            }
        }
        if ui.button(2, 10.0, 80.0, 50.0, 20.0) {
            println!("hello 2");
        }
        if ui.button(3, 40.0, 80.0, 70.0, 40.0) {
            request_quit();
        }

        ui.finish();

        next_frame().await
    }
}