use macroquad::math::Rect;

use crate::minesweeper::{Difficulty, Minesweeper};

use self::{exploder::Exploder, minefield::Minefield, status_bar::StatusBar};

use super::{elements::Align, renderer::Renderer, state::State};

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
    pub async fn new(difficulty: Difficulty) -> MinesweeperElement {
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
        self.status_bar.update(&self.game, self.timer, state, renderer);
        self.minefield.update(
            Align::Mid(area.x + area.w / 2.0),
            Align::Mid(area.y + area.h / 2.0), area.y,
            &mut self.game, &mut self.exploder,
            state, renderer
        );
    }
}