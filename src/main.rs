use macroquad::{miniquad::window::request_quit, prelude::*};
use minesweeper::Difficulty;
use ui::Ui;

pub mod ui;
pub mod minesweeper;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Minesweeper"),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    // Seed the random generation
    macroquad::rand::srand(macroquad::miniquad::date::now() as _);

    let mut ui = Ui::new();
    
    let mut p = 0;


    loop {
        clear_background(Color::from_rgba(255, 0, 0, 255));
        if is_key_pressed(KeyCode::A) { p += 1; }

        ui.begin();
        
        ui.menubar.begin();
        if ui.menubar.item("Game".to_owned(), 91.0, &mut ui.state, &mut ui.renderer) {
            if ui.menubar.dropdown("New Game".to_owned(), &mut ui.state, &mut ui.renderer) {
                /* make a new game */
            }
            ui.menubar.dropdown_separator(&mut ui.renderer);
            for (s, d) in [
                ("Easy".to_owned(),   Difficulty::Easy),
                ("Normal".to_owned(), Difficulty::Normal),
                ("Hard".to_owned(),   Difficulty::Hard),
            ] {
                ui.menubar.dropdown_radio(s, true, &mut ui.state, &mut ui.renderer);
            }
            ui.menubar.dropdown_radio("Custom".to_owned(), true, &mut ui.state, &mut ui.renderer);

            ui.menubar.dropdown_separator(&mut ui.renderer);
            if ui.menubar.dropdown("Exit".to_owned(), &mut ui.state, &mut ui.renderer) {
                request_quit();
            }

            for i in 0..p {
                if i % 3 == 0 {
                    ui.menubar.dropdown_radio("Custom".to_owned(), true, &mut ui.state, &mut ui.renderer);
                } else {
                    ui.menubar.dropdown_separator(&mut ui.renderer);
                }
            }

            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        if ui.menubar.item("Help".to_owned(), 34.0, &mut ui.state, &mut ui.renderer) {
            if ui.menubar.dropdown("About".to_owned(), &mut ui.state, &mut ui.renderer) {
                /* open about menu */
            }
            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        if ui.menubar.item("Scale".to_owned(), 22.0, &mut ui.state, &mut ui.renderer) {
            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        ui.menubar.finish(&mut ui.state, &mut ui.renderer);

        ui.finish();
        next_frame().await;
    }
}