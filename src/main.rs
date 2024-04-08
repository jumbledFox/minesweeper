use macroquad::{miniquad::window::order_quit, prelude::*};
use minesweeper::Difficulty;
use ui::{ButtonState, Style, UIState};

pub mod ui;
pub mod minesweeper;

#[macroquad::main("Minesweeper")]
async fn main() {
    // Create the UI
    let texture = Texture2D::from_file_with_format(include_bytes!("../resources/spritesheet.png"), None);
    texture.set_filter(FilterMode::Nearest);

    let style = Style {
        button_idle_source: (Rect { x: 84.0, y: 16.0, w: 3.0, h: 3.0 }, 1.0),
        button_down_source: (Rect { x: 87.0, y: 16.0, w: 3.0, h: 3.0 }, 1.0),
        dropdown_bg_source: (Rect { x: 84.0, y: 16.0, w: 3.0, h: 3.0 }, 1.0),
        menubar_idle:    (Color::from_hex(0xC0CBDC), Color::from_hex(0x181425)),
        menubar_hovered: (Color::from_hex(0x262B44), Color::from_hex(0xFFFFFF)),
        separator_col: Color::from_hex(0x8B9BB4),
        shadow_col: Color::from_rgba(0, 0, 0, 64),
    };

    let mut ui = UIState::new(texture, style);


    let mut c = false;
    let mut scale = 1;

    let mut selected_difficulty = 0;

    loop {
        clear_background(Color::from_hex(0x756853));

        ui.begin();
        
        ui.begin_menubar();
        if ui.menu_item(String::from("Game"), 82.0) {
            if ui.dropdown_item(String::from("New Game"))   {  }
            ui.dropdown_item_separator();

            if ui.dropdown_item_radio(String::from("Easy       ¬¬9¬¬*¬¬9¬¬,¬10"), selected_difficulty == 0, &mut selected_difficulty, Some(0)) {  }
            if ui.dropdown_item_radio(String::from("Normal    15*13,40"),         selected_difficulty == 1, &mut selected_difficulty, Some(1)) {  }
            if ui.dropdown_item_radio(String::from("Hard      30*16,99"),         selected_difficulty == 2, &mut selected_difficulty, Some(2)) {  }
            if ui.dropdown_item_radio(String::from("Custom..."),                  selected_difficulty == 3, &mut selected_difficulty, None) {  }
            ui.dropdown_item_separator();

            ui.dropdown_item_checkbox(String::from("Use Question Marks"), &mut c);
            ui.dropdown_item_separator();

            if ui.dropdown_item(String::from("Exit")) { order_quit(); }
            ui.finish_dropdown();
        }
        if ui.menu_item(String::from("Help"), 35.0) {
            if ui.dropdown_item(String::from("About")) {  }
            ui.finish_dropdown();
        }
        if ui.menu_item(String::from("Scale"), 25.0) {
            for i in 1..=8 {
                if i == 5 { ui.dropdown_new_column(); }
                if ui.dropdown_item_radio(format!("{:?}{}*", i, if i == 1 {"¬"} else {""}), scale == i, &mut scale, Some(i)) {

                }
            }
            ui.finish_dropdown();
        }
        ui.finish_menubar();
        
        // if ui.button(String::from("Hello!"), 10.0, 80.0, 50.0, 20.0) == ButtonState::Released {
        //     println!("hello 2");
        // }
        // if ui.button(String::from("Quit"), 40.0, 80.0, 70.0, 40.0) == ButtonState::Released {
        //     order_quit();
        // }

        ui.finish();

        next_frame().await
    }
}