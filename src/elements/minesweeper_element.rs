use ggez::{glam::Vec2, graphics::{Canvas, Color, DrawParam, Image, InstanceArray, Rect}, mint::Point2, Context, GameResult};

use crate::minesweeper::{Difficulty, Minesweeper};

use super::MouseAction;

const SPRITESHEET_IMAGE_BYTES: &[u8] = include_bytes!("../../resources/spritesheet.png");

pub struct MinesweeperElement {
    pub game: Minesweeper,
    minefield_image: Image,
    spritesheet: Image,
    spritesheet_batch: InstanceArray,
    rect: Rect,
    selected_tile: Option<usize>,
}

impl MinesweeperElement {
    pub fn new(ctx: &mut Context, difficulty: Difficulty) -> MinesweeperElement {
        let game = Minesweeper::new(difficulty);
        let spritesheet = Image::from_bytes(ctx, SPRITESHEET_IMAGE_BYTES).expect("Failed to load game spritesheet! Unable to run :/");

        let mut spritesheet_batch = InstanceArray::new(ctx, spritesheet);
        spritesheet_batch.resize(ctx, game.board().len());

        todo!()
    }

    // 
    pub fn remade_game_values(ctx: &mut Context, difficulty: Difficulty) {
        let game = Minesweeper::new(difficulty);
        let minefield_image = Image::new_canvas_image(
            ctx,
            ctx.gfx.surface_format(),
            game.width()  as u32 * 9,
            game.height() as u32 * 9,
            1,
        );
    }

    pub fn initialise_minefield(&mut self) {

    }

    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Point2<f32>, mouse_action: (MouseAction, MouseAction)) {
        // If the mouse isn't free, or if it's not over the minefield, return
        if !*mouse_free || !self.rect.contains(mouse_pos) {
            return;
        }
    }

    pub fn render_minefield(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_image(ctx, self.minefield_image.clone(), Color::RED);

        self.spritesheet_batch.set(
            self.game.board()
            .iter()
            .enumerate()
            .map(|(i, tile)| DrawParam::new()
                .dest(index_to_draw_coord(&self.game, i))
            )
        );
        canvas.draw(&self.spritesheet_batch, DrawParam::new());

        canvas.finish(ctx)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        // Draw the nice border
        
        // Draw the minefield
        canvas.draw(&self.minefield_image, DrawParam::new().dest(Point2 {x: 2.0, y: 2.0}));
        // Draw the selected tile (if it's valid)
        if let Some(selected_tile) = self.selected_tile {
            // if self.game.board()[selected_tile]
        }
    }
}

fn index_to_draw_coord(game: &Minesweeper, index: usize) -> Vec2 {
    Vec2::new(
        ((index % game.width()) * 9) as f32,
        ((index / game.width()) * 9) as f32,
    )
}