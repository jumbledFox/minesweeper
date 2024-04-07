use macroquad::{miniquad::window::order_quit, prelude::*};
use ui::{ButtonState, UIState};

pub mod ui;
pub mod minesweeper;

#[macroquad::main("Minesweeper")]
async fn main() {
    let mut ui = UIState::new();
    let mut c = true;

    loop {
        clear_background(Color::from_hex(0x756853));

        ui.begin();
        
        ui.begin_menubar();
        if ui.menu_item(String::from("Game"), 50.0) {
            if ui.dropdown(String::from("New Game"))   {  }
            ui.dropdown_separator();
            if ui.dropdown(String::from("Easy"))   {  }
            if ui.dropdown(String::from("Normal")) {  }
            if ui.dropdown(String::from("Hard"))   {  }
            if ui.dropdown(String::from("Custom")) {  }
            ui.dropdown_separator();
            if ui.dropdown(String::from("Exit")) { order_quit(); }
        }
        if ui.menu_item(String::from("Help"), 25.0) {
            if ui.dropdown(String::from("About")) {  }
        }
        if ui.menu_item(String::from("Scale"), 15.0) {
            if ui.dropdown(String::from(" 1* ")) {}
            if ui.dropdown(String::from(" 2* ")) {}
            if ui.dropdown(String::from(" 3* ")) {}
            if ui.dropdown(String::from(" 4* ")) {}
            ui.dropdown_new_column();
            if ui.dropdown(String::from(" 5* ")) {}
            if ui.dropdown(String::from(" 6* ")) {}
            if ui.dropdown(String::from(" 7* ")) {}
            if ui.dropdown(String::from(" 8* ")) {}
        }
        ui.finish_menubar();
        
        // ui.checkbox(String::from("cb1"), &mut c, 10.0, 10.0, 20.0, 20.0);
        // if c {
        //     ui.label(
        //         String::from("Hello!! this is a test of the ui label..\nthat should've been a line break.. :3"),
        //         Color::from_hex(0x00FFFF), 250.0, 40.0
        //     );
        //     ui.label(
        //         String::from("Yippee!! It seems to be working. [] {} () - + = * /"),
        //         Color::from_hex(0xFF3333), 250.0, 60.0
        //     );
        //     ui.label(
        //         String::from("This invalid character, which isn't in the CHAR_MAP, \nshould be shown as a question mark. -> 🦊 <-. It's actually a fox emoji!"),
        //         Color::from_hex(0xFFFF00), 250.0, 70.0
        //     );
        //     ui.label(
        //         String::from("jumbledFox"),
        //         Color::from_hex(0x000000), 250.0, 90.0
        //     );
        //     if ui.button(1, 10.0, 40.0, 50.0, 20.0) == ButtonState::Released {
        //         println!("hello");
        //     }
        //     for i in 0..10 {
        //         if ui.button(i+10, 130.0, i as f32*25.0 + 40.0, 80.0, 20.0) == ButtonState::Released {
        //             println!("{:?}", i);
        //         }
        //     }
        // }
        if ui.button(String::from("Hello!"), 10.0, 80.0, 50.0, 20.0) == ButtonState::Released {
            println!("hello 2");
        }
        if ui.button(String::from("Quit"), 40.0, 80.0, 70.0, 40.0) == ButtonState::Released {
            order_quit();
        }

        ui.finish();

        next_frame().await
    }
}