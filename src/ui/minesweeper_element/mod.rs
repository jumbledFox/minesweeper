use macroquad::math::Rect;

use crate::minesweeper::{Difficulty, GameState, Minesweeper};

use self::{exploder::Exploder, minefield::Minefield, status_bar::StatusBar};

use super::{renderer::Renderer, state::State};

pub mod minefield;
pub mod exploder;
pub mod status_bar;

pub struct MinesweeperElement {
    game:       Minesweeper,
    difficulty: Difficulty,
    timer:      Option<f32>,

    minefield:  Minefield,
    exploder:   Exploder,
    status_bar: StatusBar,
}

impl MinesweeperElement {
    pub async fn new() -> MinesweeperElement {
        let difficulty = Difficulty::Easy;
        let minefield = Minefield::new(difficulty).await;

        MinesweeperElement {
            game: Minesweeper::new(difficulty),
            difficulty,
            timer: None,

            minefield,
            exploder:   Exploder::default(),
            status_bar: StatusBar::default(),
        }
    } 

    pub fn update(&mut self, area: Rect, state: &mut State, renderer: &mut Renderer) {
        // Update the timer
        self.timer = match (self.game.turns(), self.game.state()) {
            // If we haven't made a move yet, the time should be None
            (0, _) => None,
            // If the game is being played, increment the timer
            (_, GameState::Playing) => Some(self.timer.unwrap_or(0.0) + macroquad::time::get_frame_time()),
            // Otherwise (meaning we've won or lost) keep the timer frozen, on a valid value
            _ => Some(self.timer.unwrap_or(0.0)),
        };

        let status_height = 23.0;
        let status_area    = Rect::new(area.x, area.y,                 area.w, status_height);
        let minefield_area = Rect::new(area.x, area.y + status_area.h, area.w, area.h - status_area.h);

        self.status_bar.update(status_area, &self.game, self.timer, state, renderer);
        self.minefield.update(minefield_area, &mut self.game, &mut self.exploder, state, renderer);
    }

    pub fn new_game(&mut self, difficulty: Difficulty) {
        self.game = Minesweeper::new(difficulty);
        self.minefield.new_game(difficulty);
    }
}