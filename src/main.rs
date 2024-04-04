use macroquad::prelude::*;
use ui::UIState;

pub mod ui;
pub mod minesweeper;

#[macroquad::main("Minesweeper")]
async fn main() {
    let mut ui_state = UIState::new();
    let mut c = false;

    loop {
        clear_background(Color::from_hex(0x756853));

        ui_state.prepare();
        
        ui_state.checkbox(0, &mut c, 10.0, 10.0, 20.0, 20.0);
        if c {
            ui_state.button(1, 10.0, 40.0, 50.0, 20.0);
        }
        // ui_state.button(2, 10.0, 10.0, 50.0, 20.0);
        // ui_state.button(3, 30.0, 15.0, 50.0, 20.0);

        ui_state.finish();

        next_frame().await
    }
}