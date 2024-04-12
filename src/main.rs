use std::{cell::RefCell, rc::Rc};

use macroquad::{miniquad::window::order_quit, prelude::*};
use ui::{menubar::Menubar, spritesheet, DrawShape, UIState};

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
        if menubar.item(String::from("Game"), 86.0) {
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
        if menubar.item("Help", 34.0) {
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
        let screen_size = ui.borrow().screen_size();
        let lower_x = screen_size.x / 5.0;
        ui::minesweeper::bomb_counter(&mut ui.borrow_mut(), vec2(lower_x,                 menubar.height() + 12.0), vec2(24.0, 18.0), 3, 0);
        ui::minesweeper::bomb_counter(&mut ui.borrow_mut(), vec2(screen_size.x / 2.0,     menubar.height() + 12.0), vec2(19.0, 19.0), 1, 0);
        ui::minesweeper::bomb_counter(&mut ui.borrow_mut(), vec2(screen_size.x - lower_x, menubar.height() + 12.0), vec2(21.0,  9.0), 2, 0);
        ui::minesweeper::minefield(&mut ui.borrow_mut(), screen_size.x / 2.0, menubar.height() + 24.0);

        ui.borrow_mut().draw_queue().push(DrawShape::nineslice(Rect::new(0.0, menubar.height(), screen_size.x, screen_size.y-menubar.height()), spritesheet::BUTTON_IDLE));

        ui.borrow_mut().finish();

        next_frame().await
    }
}