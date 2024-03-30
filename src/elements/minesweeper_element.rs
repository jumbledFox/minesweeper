use ggez::{glam::Vec2, graphics::{Canvas, Color, DrawParam, Image, InstanceArray, Rect}, mint::Point2, winit::window, Context, GameResult};

use crate::minesweeper::{Difficulty, Minesweeper, TileType};

use super::{rect_at_middle, round_rect, MouseAction};

const SPRITESHEET_IMAGE_BYTES: &[u8] = include_bytes!("../../resources/spritesheet.png");

pub struct MinesweeperElement {
    pub game: Minesweeper,
    minefield_image: Image,
    spritesheet: Image,
    spritesheet_batch: InstanceArray,
    tile_src_rects: Vec<Rect>,
    rect: Rect,
    selected_tile: Option<usize>,
}

impl MinesweeperElement {
    pub fn new(ctx: &mut Context, difficulty: Difficulty, window_middle: Vec2) -> MinesweeperElement {
        let (game, minefield_image, rect) = MinesweeperElement::remake_game_values(ctx, difficulty, window_middle);
        
        let spritesheet = Image::from_bytes(ctx, SPRITESHEET_IMAGE_BYTES).expect("Failed to load game spritesheet! Unable to run :/");
        let mut spritesheet_batch = InstanceArray::new(ctx, spritesheet.clone());
        spritesheet_batch.resize(ctx, game.board().len());
        
        // 
        let tile_src_rects = (0..16).map(|i| Rect::new(((i%4)*9) as f32 / 96.0, ((i/4)*9) as f32 / 64.0, 9.0/96.0, 9.0/64.0)).collect();

        MinesweeperElement {
            game, minefield_image, spritesheet, spritesheet_batch, tile_src_rects, rect,
            selected_tile: None
        }
    }

    // These are things that should get remade when we change the difficulty
    fn remake_game_values(ctx: &mut Context, difficulty: Difficulty, window_middle: Vec2) -> (Minesweeper, Image, Rect) {
        let game = Minesweeper::new(difficulty);
        let minefield_image = Image::new_canvas_image(
            ctx,
            ctx.gfx.surface_format(),
            game.width()  as u32 * 9,
            game.height() as u32 * 9,
            1,
        );
        let rect = round_rect(rect_at_middle(window_middle, minefield_image.width() as f32, minefield_image.height() as f32));
        (game, minefield_image, rect)
    }
    // Change the difficulty and make a new game
    pub fn new_game(&mut self, ctx: &mut Context, difficulty: Difficulty, window_middle: Vec2) {
        let (game, minefield_image, rect) = MinesweeperElement::remake_game_values(ctx, difficulty, window_middle);
        self.game = game;
        self.minefield_image = minefield_image;
        self.rect = rect;
    }

    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Vec2, mouse_action: (MouseAction, MouseAction)) {
        // If the mouse isn't free, or if it's not over the minefield, return
        if !*mouse_free || !self.rect.contains(mouse_pos) {
            self.selected_tile = None;
            return;
        }
        // Update the selected tile
        let local_mouse_pos = mouse_pos - Vec2::from(self.rect.point());
        let tile_pos = (local_mouse_pos / 9.0).floor();
        let tile_index = tile_pos.y as usize * self.game.width() + tile_pos.x as usize;
        // If tile index is out of bounds... something has gone very wrong... so i'm just gonna clamp it between valid values
        self.selected_tile = Some(tile_index.min(self.game.board().len()));

        match mouse_action.0 {
            MouseAction::Release => {
                self.game.dig(tile_index);
                println!("{:?}", self.game.state());
            }
            _ => {}
        }
        match mouse_action.1 {
            MouseAction::Release => {
                self.game.set_flag(false, tile_index);
            }
            _ => {}
        }
    }

    pub fn render_minefield(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_image(ctx, self.minefield_image.clone(), Color::RED);

        // Draw the tiles
        self.spritesheet_batch.set(
            self.game.board()
            .iter()
            .enumerate()
            .map(|(i, tile)| DrawParam::new()
                .dest(index_to_draw_coord(&self.game, i))
                .src(self.tile_src_rects[if *tile == TileType::Unopened || *tile == TileType::Flag { 0 } else { 1 }])
            )
        );
        canvas.draw(&self.spritesheet_batch, DrawParam::new());
        // Draw the numbers and flags
        self.spritesheet_batch.set(
            self.game.board()
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| match *tile {
                TileType::Numbered(n) => Some(
                    DrawParam::new()
                        .dest(index_to_draw_coord(&self.game, i))
                        .src(self.tile_src_rects[3 + n as usize])
                    ),
                TileType::Flag => Some(
                    DrawParam::new()
                        .dest(index_to_draw_coord(&self.game, i))
                        .src(self.tile_src_rects[12])
                    ),
                _ => None,
            })
        );
        canvas.draw(&self.spritesheet_batch, DrawParam::new());

        canvas.finish(ctx)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        // Draw the nice border
        
        // Draw the minefield
        canvas.draw(&self.minefield_image, DrawParam::new().dest(self.rect.point()));
        // Draw the selected tile (if it's valid)
        if let Some(selected_tile) = self.selected_tile {
            let draw_position = index_to_draw_coord(&self.game, selected_tile) + Vec2::from(self.rect.point());
            canvas.draw(&self.spritesheet, DrawParam::new().dest(draw_position - Vec2::ONE).src(Rect::new(85.0/96.0, 0.0, 11.0/96.0, 11.0/64.0)));
        }
    }
}

fn index_to_draw_coord(game: &Minesweeper, index: usize) -> Vec2 {
    Vec2::new(
        ((index % game.width()) * 9) as f32,
        ((index / game.width()) * 9) as f32,
    )
}