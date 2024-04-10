use macroquad::{miniquad::window::order_quit, prelude::*};
use ui::{menubar::Menubar, Style, UIState};

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
        shadow_color: Color::from_rgba(0, 0, 0, 128),
    });
    let mut menubar = Menubar::default();


    let mut c = false;
    let mut scale = 3;

    let mut selected_difficulty = 0;

    loop {
        ui.begin(scale as f32);
        
        menubar.begin(ui);
        if menubar.item(String::from("Game"), 85.0) {
            if menubar.dropdown("New Game") { }
            menubar.dropdown_separator();
            // TODO: Secondary colour (maybe with some control character)
            if menubar.dropdown_radio("Easy       ¬¬9¬¬*¬¬9¬¬,¬10", selected_difficulty == 0) { selected_difficulty = 0 }
            if menubar.dropdown_radio("Normal    15*13,40",         selected_difficulty == 1) { selected_difficulty = 1 }
            if menubar.dropdown_radio("Hard      30*16,99",         selected_difficulty == 2) { selected_difficulty = 2 }
            if menubar.dropdown_radio("Custom...",                  selected_difficulty == 3) {  }
            menubar.dropdown_separator();

            menubar.dropdown_checkbox("Use Question Marks", &mut c);
            menubar.dropdown_separator();

            if menubar.dropdown("Exit") { order_quit(); }
            menubar.finish_item();
        }
        if menubar.item("Help", 35.0) {
            menubar.dropdown("About");
            menubar.finish_item();
        }
        if menubar.item("Scale", 22.0) {
            for i in 1..=8 {
                if menubar.dropdown_radio(format!("{:?}{}*", i, if i == 1 {"¬"} else {""}), scale == i) {
                    scale = i
                }
                if i == 4 { menubar.dropdown_new_column(); }
            }
            menubar.finish_item();
        }
        ui = menubar.finish();

        if ui.button(String::from("Hello!"), ui::TextAlignment::Left(2.0), 10.0, 70.0, 50.0, 20.0).into() {
            println!("hello 2");
        }
        if ui.button(String::from("Quit"), ui::TextAlignment::Center, 40.0, 80.0, 70.0, 40.0).into() {
            order_quit();
        }
        // TODO: Draw a background

        // TODO: Minesweeper elements

        ui.finish();

        next_frame().await
    }
}