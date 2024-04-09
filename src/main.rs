use macroquad::{miniquad::window::order_quit, prelude::*};
use ui::{Style, UIState};

pub mod ui;
pub mod minesweeper;

#[macroquad::main("Minesweeper")]
async fn main() {
    // Create the UI
    let texture = Texture2D::from_file_with_format(include_bytes!("../resources/spritesheet.png"), None);
    texture.set_filter(FilterMode::Nearest);

    let mut ui = UIState::new(texture, Style {
        button_idle_source: (Rect { x: 84.0, y: 16.0, w: 3.0, h: 3.0 }, 1.0),
        button_down_source: (Rect { x: 87.0, y: 16.0, w: 3.0, h: 3.0 }, 1.0),
        dropdown_bg_source: (Rect { x: 84.0, y: 16.0, w: 3.0, h: 3.0 }, 1.0),
        menubar_idle:    (Color::from_hex(0xC0CBDC), Color::from_hex(0x181425)),
        menubar_hovered: (Color::from_hex(0x262B44), Color::from_hex(0xFFFFFF)),
        separator_source: Rect { x: 89.0, y: 11.0, w: 1.0, h: 2.0 },
        shadow_col: Color::from_rgba(0, 0, 0, 64),
    });


    let mut c = false;
    let mut scale = 1;

    let mut selected_difficulty = 0;

    loop {
        clear_background(Color::from_hex(0x756853));

        ui.begin();
        
        ui.begin_menubar();
        if ui.menu_item(String::from("Game"), 85.0) {
            if ui.dropdown_item(String::from("New Game")).into() {  }
            ui.dropdown_separator();
            // TODO: Secondary colour
            if ui.dropdown_item_radio(String::from("Easy       ¬¬9¬¬*¬¬9¬¬,¬10"), selected_difficulty == 0).into() { selected_difficulty = 0 }
            if ui.dropdown_item_radio(String::from("Normal    15*13,40"),         selected_difficulty == 1).into() { selected_difficulty = 1 }
            if ui.dropdown_item_radio(String::from("Hard      30*16,99"),         selected_difficulty == 2).into() { selected_difficulty = 2 }
            if ui.dropdown_item_radio(String::from("Custom..."),                  selected_difficulty == 3).into() {  }
            ui.dropdown_separator();

            ui.dropdown_item_checkbox(String::from("Use Question Marks"), &mut c);
            ui.dropdown_separator();

            if ui.dropdown_item(String::from("Exit")).into() { order_quit(); }
            ui.finish_menu_item();
        }
        if ui.menu_item(String::from("Help"), 35.0) {
            if ui.dropdown_item(String::from("About")).into() {  }
            ui.finish_menu_item();
        }
        if ui.menu_item(String::from("Scale"), 22.0) {
            for i in 1..=8 {
                if i == 5 { ui.dropdown_new_column(); }
                if ui.dropdown_item_radio(format!("{:?}{}*", i, if i == 1 {"¬"} else {""}), scale == i).into() {
                    scale = i;
                }
            }
            ui.finish_menu_item();
        }
        ui.finish_menubar();
        
        if ui.button(String::from("Hello!"), ui::TextAlignment::Left(2.0), 10.0, 70.0, 50.0, 20.0).into() {
            println!("hello 2");
        }
        if ui.button(String::from("Quit"), ui::TextAlignment::Center, 40.0, 80.0, 70.0, 40.0).into() {
            order_quit();
        }

        ui.finish();

        next_frame().await
    }
}