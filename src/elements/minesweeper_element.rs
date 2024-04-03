use ggez::{glam::Vec2, graphics::{Canvas, Color, DrawParam, Image, InstanceArray, Rect}, mint::Point2, winit::window, Context, GameResult};

use crate::minesweeper::{Difficulty, GameState, Minesweeper, TileType};

use super::{rect_at_middle, round_rect, MouseAction};

const SPRITESHEET_IMAGE_BYTES: &[u8] = include_bytes!("../../resources/spritesheet.png");

pub struct MinesweeperElement {
    pub game: Minesweeper,
    minefield_image: Image,
    spritesheet: Image,
    spritesheet_batch: InstanceArray,
    tile_src_rects: Vec<Rect>,
    pos: Vec2,
    size: Vec2,
    selected_tile: Option<usize>,
    exploded_bombs: Vec<usize>,
}

impl MinesweeperElement {
    pub fn new(ctx: &mut Context, difficulty: Difficulty) -> MinesweeperElement {
        let (game, exploded_bombs, minefield_image, size) = MinesweeperElement::remake_game_values(ctx, difficulty);
        
        let spritesheet = Image::from_bytes(ctx, SPRITESHEET_IMAGE_BYTES).expect("Failed to load game spritesheet! Unable to run :/");
        let mut spritesheet_batch = InstanceArray::new(ctx, spritesheet.clone());
        spritesheet_batch.resize(ctx, game.board().len());
        
        // 
        let tile_src_rects = (0..16).map(|i| Rect::new(((i%4)*9) as f32 / 96.0, ((i/4)*9) as f32 / 64.0, 9.0/96.0, 9.0/64.0)).collect();

        MinesweeperElement {
            spritesheet, spritesheet_batch, tile_src_rects,

            selected_tile: None,
            game, exploded_bombs, minefield_image, size,
            
            pos: Vec2::ZERO,
        }
    }

    // These are things that should get remade when we change the difficulty
    fn remake_game_values(ctx: &mut Context, difficulty: Difficulty) -> (Minesweeper, Vec<usize>, Image, Vec2) {
        let game = Minesweeper::new(difficulty);
        let exploded_bombs = Vec::with_capacity(game.bomb_count());
        let minefield_image = Image::new_canvas_image(
            ctx,
            ctx.gfx.surface_format(),
            game.width()  as u32 * 9,
            game.height() as u32 * 9,
            1,
        );
        let size = Vec2::new(minefield_image.width() as f32, minefield_image.height() as f32);
        (game, exploded_bombs, minefield_image, size)
    }
    // Change the difficulty and make a new game
    pub fn new_game(&mut self, ctx: &mut Context, difficulty: Difficulty) {
        let (game, exploded_bombs, minefield_image, size) = MinesweeperElement::remake_game_values(ctx, difficulty);
        self.game = game;
        self.minefield_image = minefield_image;
        self.size = size;
        self.exploded_bombs = exploded_bombs;
    }

    // 
    pub fn size(&self) -> Vec2 { self.size }
    pub fn pos(&self) -> Vec2 { self.pos }
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Vec2, mouse_action: (MouseAction, MouseAction)) {
        // If the mouse isn't free, or if it's not over the minefield, return
        let rect = Rect { x: self.pos.x, y: self.pos.y, w: self.size.x, h: self.size.y };
        if !*mouse_free || !rect.contains(mouse_pos) {
            self.selected_tile = None;
            return;
        }
        // Update the selected tile
        let local_mouse_pos = mouse_pos - self.pos;
        let tile_pos = (local_mouse_pos / 9.0).floor();
        let tile_index = tile_pos.y as usize * self.game.width() + tile_pos.x as usize;
        // If tile index is out of bounds... something has gone very wrong... so i'm just gonna clamp it between valid values
        self.selected_tile = Some(tile_index.min(self.game.board().len()));

        // We don't care about doing anything if we're not playing
        
        // Left click (digging)
        match mouse_action.0 {
            // Dig and react accordingly
            MouseAction::Release => {
                self.game.dig(tile_index);
                // If this dig has just made us lose... lol
                if *self.game.state() == GameState::Lose {
                    self.exploded_bombs.push(tile_index);
                }
                
            }
            _ => {}
        }
        // Right click (flagging)
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
                .src(self.tile_src_rects[
                    if self.exploded_bombs.get(0).is_some_and(|x| *x == i) { 3 }
                    else if self.game.bombs().contains(&i) && *self.game.state() == GameState::Lose { 2 }
                    else if *tile == TileType::Unopened || *tile == TileType::Flag { 0 }
                    else { 2 }
                ])
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
        canvas.draw(&self.minefield_image, DrawParam::new().dest(self.pos));
        // Draw the selected tile (if it's valid)
        if let Some(selected_tile) = self.selected_tile {
            let draw_position = index_to_draw_coord(&self.game, selected_tile) + self.pos;
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