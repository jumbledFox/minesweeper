use macroquad::{miniquad::window::{cancel_quit, order_quit}, prelude::*};
use minesweeper::{Difficulty, GameState};
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

    let mut ui = Ui::new().await;

    loop {
        let mut quit = is_quit_requested();
        let mut new_game = None;

        ui.begin();

        ui.menubar.begin();
        if ui.menubar.item("Game".to_owned(), 91.0, &mut ui.state, &mut ui.renderer) {
            // New game
            if ui.menubar.dropdown("New Game".to_owned(), None, &mut ui.state, &mut ui.renderer) {
                new_game = Some(ui.minesweeper_element.difficulty());
            }
            ui.menubar.dropdown_separator(&mut ui.renderer);

            // Easy, Normal, Hard
            let difficulties = [
                ("Easy"  .to_owned(), "9¬¬*¬¬9¬¬,  ¬9 ¬¬".to_owned(), Difficulty::Easy),
                ("Normal".to_owned(), "¬15*13, ¬¬40¬¬"   .to_owned(), Difficulty::Normal),
                ("Hard"  .to_owned(), "30*16, 100"       .to_owned(), Difficulty::Hard),
            ];
            for (text, other_text, difficulty) in difficulties {
                let is_current = ui.minesweeper_element.difficulty() == difficulty;
                if ui.menubar.dropdown_radio(text, Some(other_text), is_current, &mut ui.state, &mut ui.renderer) {
                    new_game = Some(difficulty);
                }
            }

            // Custom
            if ui.menubar.dropdown_radio("Custom...".to_owned(), None, ui.minesweeper_element.difficulty().is_custom(), &mut ui.state, &mut ui.renderer) {
                ui.popups.add(PopupKind::custom(ui.minesweeper_element.custom_values()), &mut ui.state);
            };
            ui.menubar.dropdown_separator(&mut ui.renderer);

            // Screen shake toggle
            if ui.menubar.dropdown_radio("Screen Shake".to_owned(), None, ui.renderer.shake_enabled, &mut ui.state, &mut ui.renderer) {
                ui.renderer.shake_enabled = !ui.renderer.shake_enabled;
            }
            ui.menubar.dropdown_separator(&mut ui.renderer);

            // Exit
            if ui.menubar.dropdown("Exit".to_owned(), None, &mut ui.state, &mut ui.renderer) {
                quit = true;
            }

            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        if ui.menubar.item("Help".to_owned(), 34.0, &mut ui.state, &mut ui.renderer) {
            if ui.menubar.dropdown("Hint".to_owned(), None, &mut ui.state, &mut ui.renderer) {
                ui.popups.add(PopupKind::Hint, &mut ui.state);
            }
            if ui.menubar.dropdown("About".to_owned(), None, &mut ui.state, &mut ui.renderer) {
                ui.popups.add(PopupKind::About, &mut ui.state);
            }
            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        if ui.menubar.item("Scale".to_owned(), 28.0, &mut ui.state, &mut ui.renderer) {
            if ui.menubar.dropdown_radio("Auto".to_owned(), None, ui.state.auto_scale(), &mut ui.state, &mut ui.renderer) {
                ui.state.set_auto_scale(!ui.state.auto_scale());
            }
            ui.menubar.dropdown_separator(&mut ui.renderer);
            for i in 1..=8 {
                if ui.menubar.dropdown_radio(format!(" {}{}* ", if i == 1 {"¬"} else {""}, i), None, ui.state.scale() == i as f32, &mut ui.state, &mut ui.renderer) {
                    ui.state.set_auto_scale(false);
                    ui.state.set_scale(i as f32);
                }
            }
            ui.menubar.finish_item(&mut ui.state, &mut ui.renderer);
        }
        ui.menubar.finish(&mut ui.state, &mut ui.renderer);

        // TODO: Make it so popup positions change depending on the new and old scale of the window
        ui.popups.update(&mut ui.state, &ui.menubar, &mut ui.renderer);

        // Draw the minesweeper game below the menubar
        let minesweeper_area = Rect::new(
            0.0                                            + ui.renderer.style().background().padding,
            ui.menubar.height()                            + ui.renderer.style().background().padding,
            ui.state.screen_size().x                       - ui.renderer.style().background().padding * 2.0,
            ui.state.screen_size().y - ui.menubar.height() - ui.renderer.style().background().padding * 2.0,
        );
        ui.minesweeper_element.update(minesweeper_area, &mut ui.state, &mut ui.renderer);

        match ui.minesweeper_element.game_state_change() {
            Some(GameState::Win) => {
                ui.popups.add(PopupKind::Win, &mut ui.state);
            }
            _ => {},
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
        
        // Making / requesting a new game 
        if let Some(difficulty) = new_game.or_else(|| ui.minesweeper_element.new_game_requested()) {
            if ui.minesweeper_element.game_in_progress() {
                ui.popups.add(PopupKind::NewGame { difficulty }, &mut ui.state);
            } else {
                ui.minesweeper_element.new_game(difficulty, &ui.renderer);
            }
        }

        ui.popups.handle_returns(&mut ui.minesweeper_element, &ui.renderer);

        ui.finish();

        next_frame().await;
    }
}