use macroquad::{miniquad::window::{cancel_quit, order_quit}, prelude::*};
use minesweeper::Difficulty;
use ui::{popups::PopupKind, Ui};

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

            let difficulties = [
                ("Easy".to_owned(),   Difficulty::Easy),
                ("Normal".to_owned(), Difficulty::Normal),
                ("Hard".to_owned(),   Difficulty::Hard),
            ];
            for (s, d) in difficulties {
                let is_current_difficulty = std::mem::discriminant(&ui.minesweeper_element.difficulty()) == std::mem::discriminant(&d);
                if ui.menubar.dropdown_radio(s, is_current_difficulty, &mut ui.state, &mut ui.renderer) {
                    new_game = Some(d);
                }
            }
            if ui.menubar.dropdown_radio("Custom".to_owned(), matches!(ui.minesweeper_element.difficulty(), Difficulty::Custom(_)), &mut ui.state, &mut ui.renderer) {
                ui.popups.add(PopupKind::custom(ui.minesweeper_element.custom_values()), &mut ui.state);
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

        ui.popups.update(&mut ui.state, &ui.menubar, &mut ui.renderer);
        
        // Draw the minesweeper to a specific area
        let minesweeper_area = Rect::new(
            0.0,
            ui.menubar.height(),
            ui.state.screen_size().x,
            ui.state.screen_size().y - ui.menubar.height(),
        );
        ui.minesweeper_element.update(minesweeper_area, &mut ui.state, &mut ui.renderer);

        if quit {
            if ui.minesweeper_element.game_in_progress() {
                cancel_quit();
                quit = false;
                ui.popups.add(PopupKind::Exit, &mut ui.state);
            }
        }
        if quit {
            order_quit();
        }

        if new_game.is_some() || ui.minesweeper_element.requesting_new_game() {
            let difficulty = match new_game {
                Some(d) => d,
                None => ui.minesweeper_element.difficulty(),
            };
            if ui.minesweeper_element.game_in_progress() {
                ui.popups.add(PopupKind::NewGame { difficulty }, &mut ui.state);
            } else {
                ui.minesweeper_element.new_game(difficulty);
            }
        }

        ui.renderer.draw_background(&mut ui.state, &mut ui.menubar);

        ui.finish();

        next_frame().await;
    }
}