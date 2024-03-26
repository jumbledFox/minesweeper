use ggez::{graphics::{Canvas, DrawParam, Image, Rect}, mint::Point2, Context};

use crate::minesweeper::Minesweeper;

pub struct MinesweeperElement {
    pub game: Minesweeper,
    minefield_image: Image,
    rect: Rect,
    selected_tile: Option<usize>,
}

impl MinesweeperElement {
    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Point2<f32>) {
        
    }

    pub fn render_minefield(&mut self, ctx: &mut Context) {

    }

    pub fn draw(&self, canvas: &mut Canvas) {
        // Draw the nice border
        
        // Draw the actual minefield
        canvas.draw(&self.minefield_image, DrawParam::new().dest(Point2 {x: 2.0, y: 2.0}));
        // Draw the selected tile (if it's valid)
        if let Some(selected_tile) = self.selected_tile {
            // if self.game.board()[selected_tile]
        }
    }
}