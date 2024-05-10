use macroquad::{miniquad::window::{cancel_quit, order_quit}, prelude::*};
use minesweeper::Difficulty;
use ui::{popups::{PopupKind, PopupReturn}, Ui};

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

    let mut ui = Ui::new().await;

    loop {
        let mut quit = is_quit_requested();
        let mut new_game = None;

        ui.begin();
        
        // ui.renderer.draw(ui::renderer::DrawShape::text(50.0, 50.0, "HELLO\nWORLD\nFOX:3".to_owned(), None, macroquad::color::RED));

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
            if ui.menubar.dropdown("Hint".to_owned(), &mut ui.state, &mut ui.renderer) {
                ui.popups.add(PopupKind::Hint, &mut ui.state);
            }
            if ui.menubar.dropdown("About".to_owned(), &mut ui.state, &mut ui.renderer) {
                ui.popups.add(PopupKind::About, &mut ui.state);
            }
            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        if ui.menubar.item("Scale".to_owned(), 28.0, &mut ui.state, &mut ui.renderer) {
            if ui.menubar.dropdown_radio("Auto".to_owned(), ui.state.auto_scale(), &mut ui.state, &mut ui.renderer) {
                ui.state.set_auto_scale(!ui.state.auto_scale());
            }
            ui.menubar.dropdown_separator(&mut ui.renderer);
            for i in 1..=8 {
                if ui.menubar.dropdown_radio(format!(" {}{}* ", if i == 1 {"¬"} else {""}, i), ui.state.scale() == i as f32, &mut ui.state, &mut ui.renderer) {
                    ui.state.set_auto_scale(false);
                    ui.state.set_scale(i as f32);
                }
            }
            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        ui.menubar.finish(&mut ui.state, &mut ui.renderer);

        ui.popups.update(&mut ui.state, &ui.menubar, &mut ui.renderer);

        // Draw the minesweeper below the menubar
        let minesweeper_area = Rect::new(0.0, ui.menubar.height(), ui.state.screen_size().x, ui.state.screen_size().y - ui.menubar.height());
        ui.minesweeper_element.update(minesweeper_area, &mut ui.state, &mut ui.renderer);
        // Winning popup
        if ui.minesweeper_element.won_this_frame() {
            ui.popups.add(PopupKind::Win, &mut ui.state);
            ui.minesweeper_element.win_sound();
        }

        // Quiting
        if quit {
            if ui.minesweeper_element.game_in_progress() {
                cancel_quit();
                ui.popups.add(PopupKind::Exit, &mut ui.state);
            } else {
                order_quit();
            }
        }

        // Updating popups, done at the end to prevent flicker
        for p in ui.popups.returns() {
            match p {
                &PopupReturn::NewGame { difficulty } => ui.minesweeper_element.new_game(difficulty)
            }
        }
        // Making a new game
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

        ui.finish();

        next_frame().await;
    }
}