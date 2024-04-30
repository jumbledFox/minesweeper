use macroquad::{miniquad::window::{cancel_quit, order_quit}, prelude::*};
use minesweeper::Difficulty;
use ui::{elements::Align, popups::PopupKind, Ui};

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
    prevent_quit();
    // Seed the random generation
    macroquad::rand::srand(macroquad::miniquad::date::now() as _);

    let mut ui = Ui::new();
    ui.popups.add(PopupKind::About, &mut ui.state);
    loop {
        let mut quit = is_quit_requested();
        let mut new_game = None;

        ui.begin();
        
        ui.menubar.begin();
        if ui.menubar.item("Game".to_owned(), 91.0, &mut ui.state, &mut ui.renderer) {
            if ui.menubar.dropdown("New Game".to_owned(), &mut ui.state, &mut ui.renderer) {
                new_game = Some(Difficulty::Hard);
            }
            ui.menubar.dropdown_separator(&mut ui.renderer);
            for (s, d) in [
                ("Easy".to_owned(),   Difficulty::Easy),
                ("Normal".to_owned(), Difficulty::Normal),
                ("Hard".to_owned(),   Difficulty::Hard),
            ] {
                if ui.menubar.dropdown_radio(s, true, &mut ui.state, &mut ui.renderer) {
                    new_game = Some(d);
                }
            }
            if ui.menubar.dropdown_radio("Custom".to_owned(), true, &mut ui.state, &mut ui.renderer) {
                ui.popups.add(PopupKind::Custom, &mut ui.state);
            };

            ui.menubar.dropdown_separator(&mut ui.renderer);
            if ui.menubar.dropdown("Exit".to_owned(), &mut ui.state, &mut ui.renderer) {
                quit = true;
            }

            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        if ui.menubar.item("Help".to_owned(), 34.0, &mut ui.state, &mut ui.renderer) {
            if ui.menubar.dropdown("About".to_owned(), &mut ui.state, &mut ui.renderer) {
                ui.popups.add(PopupKind::About, &mut ui.state);
            }
            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        if ui.menubar.item("Scale".to_owned(), 62.0, &mut ui.state, &mut ui.renderer) {
            ui.menubar.dropdown_toggle("Auto".to_owned(), &mut true, &mut ui.state, &mut ui.renderer);
            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        ui.menubar.finish(&mut ui.state, &mut ui.renderer);

        // Quit popup
        if quit {
            // if game in progress
            if game_in_progress() {
                cancel_quit();
                quit = false;
                ui.popups.add(PopupKind::Exit, &mut ui.state);
            }
        }
        if quit { order_quit(); }

        // ui::elements::button(1131, "hello:3".to_owned(), Align::Mid(ui.state.screen_size().x), Align::Mid(ui.state.screen_size().y), &mut ui.state, &mut ui.renderer);

        // Making a new game
        if let Some(new_difficulty) = new_game {
            // If game in progress
            if game_in_progress() {
                ui.popups.add(PopupKind::NewGame { difficulty: new_difficulty }, &mut ui.state);
            } else {
                // set the new game directly
            }
        }

        ui.popups.update(&mut ui.state, &ui.menubar, &mut ui.renderer);

        ui.renderer.draw_background(&mut ui.state, &mut ui.menubar);

        ui.finish();

        next_frame().await;
    }
}

fn game_in_progress() -> bool {
    true
}