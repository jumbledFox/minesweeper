use macroquad::math::{vec2, Rect, Vec2};

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
    
    game_state_change: Option<GameState>,
    new_game_request:  Option<Difficulty>,
    custom_values:     Option<Difficulty>,
}

impl MinesweeperElement {
    pub async fn new(renderer: &Renderer) -> MinesweeperElement {
        let difficulty = Difficulty::Easy;
        let minefield = Minefield::new(difficulty, renderer).await;

        MinesweeperElement {
            game: Minesweeper::new(difficulty),
            difficulty,
            timer: None,

            minefield,
            exploder:   Exploder::default(),
            status_bar: StatusBar::default(),

            game_state_change: None,
            new_game_request:  None,
            custom_values:     None
        }
    }

    pub fn difficulty(&self)    -> Difficulty         { self.difficulty }
    pub fn custom_values(&self) -> Option<Difficulty> { self.custom_values }

    pub fn game_in_progress(&self) -> bool {
        self.game.state().is_playing() && self.game.turns() != 0
    }

    pub fn new_game_requested(&mut self) -> Option<Difficulty> {
        self.new_game_request.take()
    }

    pub fn game_state_change(&mut self) -> Option<GameState> {
        self.game_state_change
    }

    // The minimum size the area can be before clipping
    pub fn minimum_size(&self, renderer: &Renderer) -> Vec2 {
        let minefield_size  = self.minefield .min_size(renderer);
        let status_bar_size = self.status_bar.min_size(renderer);
        vec2(
            f32::max(minefield_size.x, status_bar_size.x),
            minefield_size.y + status_bar_size.y,
        )
    }

    pub fn new_game(&mut self, difficulty: Difficulty, renderer: &Renderer) {
        self.game = Minesweeper::new(difficulty);
        self.difficulty = difficulty;
        self.minefield.new_game(difficulty, &renderer);
        self.exploder.reset();

        if difficulty.is_custom() {
            self.custom_values = Some(difficulty);
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

        let status_area    = Rect::new(area.x, area.y,                 area.w, self.status_bar.min_size(renderer).y);
        let minefield_area = Rect::new(area.x, area.y + status_area.h, area.w, area.h - status_area.h);

        // If the button was clicked, we want to start a new game
        if self.status_bar.update(status_area, self.minefield.about_to_dig(), &self.game, self.timer, state, renderer) {
            self.new_game_request = Some(self.difficulty)
        }

        let prev_state = self.game.state();
        self.minefield.update(minefield_area, &mut self.game, &mut self.exploder, state, renderer);
        let new_state  = self.game.state();

        self.game_state_change = match prev_state != new_state {
            true  => Some(new_state),
            false => None,
        }
    }
}