use std::{cell::RefCell, rc::Rc};

use macroquad::{miniquad::window::order_quit, prelude::*};
use ui::{UIState, menubar::Menubar};

pub mod ui;
pub mod minesweeper;

#[macroquad::main("Minesweeper")]
async fn main() {
    // Create the UI
    let texture = Texture2D::from_file_with_format(include_bytes!("../resources/spritesheet.png"), None);
    texture.set_filter(FilterMode::Nearest);

    let ui = Rc::new(RefCell::new(UIState::new(texture)));
    let mut menubar = Menubar::new(Rc::clone(&ui));

    let mut use_q_marks = false;
    let mut auto_resize = true;
    let mut scale = 3;

    let mut selected_difficulty = 0;

    loop {
        ui.borrow_mut().begin(scale as f32);
        
        menubar.begin();
        if menubar.item(String::from("Game"), 85.0) {
            if menubar.dropdown("New Game") { }
            menubar.dropdown_separator();
            // TODO: Secondary colour (maybe with some control character)
            if menubar.dropdown_radio("Easy       ¬¬9¬¬*¬¬9¬¬,¬10", selected_difficulty == 0) { selected_difficulty = 0 }
            if menubar.dropdown_radio("Normal    15*13,40",         selected_difficulty == 1) { selected_difficulty = 1 }
            if menubar.dropdown_radio("Hard      30*16,99",         selected_difficulty == 2) { selected_difficulty = 2 }
            // TODO: Popups (eeek.....)
            if menubar.dropdown_radio("Custom...",                  selected_difficulty == 3) {  }
            menubar.dropdown_separator();

            menubar.dropdown_checkbox("Use Question Marks", &mut use_q_marks);
            menubar.dropdown_checkbox("Auto-resize Window", &mut auto_resize); // TODO: Would 'r' be capitalised?
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
        menubar.finish();

        // TODO: Draw a background

        // TODO: Minesweeper elements
        let screen_width  = ui.borrow().screen_size().x;
        let lower_x = screen_width / 5.0;
        ui::minesweeper::bomb_counter(&mut ui.borrow_mut(), 18.0, lower_x,                menubar.height() + 3.0, 3, 12);
        ui::minesweeper::bomb_counter(&mut ui.borrow_mut(), 19.0, screen_width / 2.0,     menubar.height() + 2.0, 1, 14);
        ui::minesweeper::bomb_counter(&mut ui.borrow_mut(),  9.0, screen_width - lower_x, menubar.height() + 7.0, 2, 0);
        
        ui.borrow_mut().finish();

        next_frame().await
    }
}