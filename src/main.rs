use macroquad::{miniquad::window::{order_quit, request_quit}, prelude::*};
use ui::{ButtonState, UIState};

pub mod ui;
pub mod minesweeper;

#[macroquad::main("Minesweeper")]
async fn main() {
    let mut ui = UIState::new();
    let mut c = true;

    loop {
        clear_background(Color::from_hex(0x756853));

        ui.prepare();
        
        ui.begin_menubar();
        if ui.menu_item(String::from("Game")) {
            // if ui.dropdown("Exit") { order_quit(); }
        }
        if ui.menu_item(String::from("Help")) {
            // if ui.dropdown("About") {  }
        }
        if ui.menu_item(String::from("Scale")) {
            // if ui.dropdown("Exit") { order_quit(); }
        }
        
        ui.checkbox(0, &mut c, 10.0, 10.0, 20.0, 20.0);
        if c {
            if ui.button(1, 10.0, 40.0, 50.0, 20.0) == ButtonState::Released {
                println!("hello");
            }
            for i in 0..10 {
                if ui.button(i+10, 130.0, i as f32*25.0 + 40.0, 80.0, 20.0) == ButtonState::Released {
                    println!("{:?}", i);
                }
            }
        }
        if ui.button(2, 10.0, 80.0, 50.0, 20.0) == ButtonState::Released {
            println!("hello 2");
        }
        if ui.button(3, 40.0, 80.0, 70.0, 40.0) == ButtonState::Released {
            order_quit();
        }

        ui.finish();

        next_frame().await
    }
}