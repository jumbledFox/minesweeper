use ggez::conf::{WindowMode, WindowSetup};
use ggez::context::HasMut;
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::dpi::{LogicalSize, PhysicalSize};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, FilterMode, GraphicsContext, Image, ImageFormat, InstanceArray, Rect, ScreenImage};
use ggez::event::{self, EventHandler};

use rand::prelude::*;

pub mod minesweeper;
use minesweeper::Minesweeper;

use crate::minesweeper::TileType;

fn main() {
    // Make the Minesweeper game
    // let game = Minesweeper::new(100, 40);
    let game = Minesweeper::new(9, 9);
    let min_window_size = Vec2::new((game.width * 9 + 4 + 8) as f32, (game.height * 9 + 4 + 8 + 20) as f32);

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("MineSweeper", "jumbledFox")
        .window_mode(WindowMode {
            resizable: true,
            logical_size: Some(LogicalSize::new(min_window_size.x, min_window_size.y)),
            min_width: min_window_size.x, min_height: min_window_size.y,
            ..Default::default()
        })
        .window_setup(WindowSetup {
            title: String::from("jumbledFox's Minesweeper"),
            icon: String::from("/icon.png"),
            ..Default::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MainState::new(&mut ctx, game, min_window_size);
    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Assets {
    mockup: Image,
    spritesheet: Image,
    spritesheet_dimensions: Vec2,
}

struct Canvases {
    minefield: Image,
    bombcounter: Image,
    timer: Image,
    tile_batch: InstanceArray,
}

struct MainState {
    assets: Assets,
    canvases: Canvases,
    game: Minesweeper,

    window_min_size: Vec2,
}

impl MainState {
    pub fn new(ctx: &mut Context, game: Minesweeper, window_min_size: Vec2) -> MainState {
        // Load/create resources such as images here.
        let spritesheet = Image::from_path(ctx, "/spritesheet.png").unwrap();
        let assets = Assets {
            mockup:      Image::from_path(ctx, "/mockup.png").unwrap(),
            spritesheet_dimensions: Vec2::new(spritesheet.width() as f32, spritesheet.height() as f32), spritesheet,
        };

        let minefield_canvas_size = game.get_minefield_canvas_size();

        let mut tile_batch = InstanceArray::new(ctx, assets.spritesheet.clone());
        tile_batch.resize(ctx, game.width * game.height);

        let bombcount = 10;
        let bombcounter_size = Vec2::new((f32::log10(bombcount as f32).floor() + 1.0) * 10.0 + 4.0, 18.0);

        let mut m = MainState {
            assets,
            canvases: Canvases {
                minefield: Image::new_canvas_image(&ctx.gfx, ctx.gfx.surface_format(), minefield_canvas_size.0, minefield_canvas_size.1, 1),
                bombcounter: Image::new_canvas_image(&ctx.gfx, ctx.gfx.surface_format(), bombcounter_size.x as u32, bombcounter_size.y as u32, 1),
                timer: Image::new_canvas_image(&ctx.gfx, ctx.gfx.surface_format(), 21, 9, 1),
                tile_batch,
            },
            window_min_size,
            game,
        };
        m.draw_minefield(ctx);
        m.draw_bombcounter(ctx);
        m
    }

    fn draw_9_slice(&self, gfx: &mut impl HasMut<GraphicsContext>, canvas: &mut graphics::Canvas, border: Vec2, source_rect: Rect, final_rect: Rect) -> GameResult {
        // Calculate 9 slice values
        
        Ok(())
    }

    // Given the windows size, calculate the scale factor and such of the screen. We don't want any stretching!
    fn calculate_screen_coordinates(&self, window_size: PhysicalSize<u32>) -> Vec2 {
        let window_size = Vec2::new(window_size.width as f32, window_size.height as f32);
        let scale_factor = f32::min(window_size.x / self.window_min_size.x, window_size.y / self.window_min_size.y).floor();
        Vec2::new(window_size.x / scale_factor, window_size.y / scale_factor)
    }

    fn draw_minefield(&mut self, ctx: &mut Context) -> GameResult {
        ctx.gfx.begin_frame().unwrap();

        let mut minefield_canvas = Canvas::from_image(ctx, self.canvases.minefield.clone(), Color::RED);

        // Draw the fancy border
        self.draw_9_slice(ctx, &mut minefield_canvas, Vec2::new(2.0, 2.0),
            Rect::new(27.0, 0.0, 5.0, 5.0),
             Rect::new(0.0, 0.0, self.canvases.minefield.width() as f32, self.canvases.minefield.height() as f32))?;

        // Draw cells
        self.canvases.tile_batch.set(
            self.game.board
            .iter().enumerate()
            .map(|(i, tile)| DrawParam::new()
                .dest(self.game.index_to_draw_coord(i))
                // And the source rect
                .src(
                    bound_rect(Rect::new(if *tile == TileType::Dug {18.0} else {0.0}, 0.0, 9.0, 9.0), self.assets.spritesheet_dimensions)
                )
            )
        );
        minefield_canvas.draw(&self.canvases.tile_batch, graphics::DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        // If there's a tile being selected, draw the pushed down version of the tile
        if let Some(index) = self.game.selected_tile {
            minefield_canvas.draw(&self.assets.spritesheet, graphics::DrawParam::new()
            .src(Rect::new(9.0 / 86.0, 0.0, 9.0 / 86.0, 9.0 / 36.0))
            .dest(self.game.index_to_draw_coord(index) + 2.0));
        }

        // Draw neighbour count
        // Positions of each number in the sprite sheet
        const COORDS: [(f32, f32); 8] = [
            ( 0.0,  9.0), // 1
            ( 9.0,  9.0), // 2
            (18.0,  9.0), // 3
            (27.0,  9.0), // 4
            ( 0.0, 18.0), // 5
            ( 9.0, 18.0), // 6
            (18.0, 18.0), // 7
            (27.0, 18.0), // 8
        ];

        self.canvases.tile_batch.set(
            self.game.neighbour_count
            .iter().enumerate()
            // Don't draw the neighbour count if it's zero or if the tile hasn't been dug yet
            .filter_map(|(i, count)| match self.game.board[i] != TileType::Dug || *count == 0 {
                false => Some((i, count)),
                true  => None,
            })
            .map(|(i, &count)| DrawParam::new()
                .dest(self.game.index_to_draw_coord(i))
                .src(
                    bound_rect(Rect::new(COORDS[count as usize - 1].0, COORDS[count as usize - 1].1, 9.0, 9.0), self.assets.spritesheet_dimensions)
                )
            )
        );
        minefield_canvas.draw(&self.canvases.tile_batch, graphics::DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        // Draw flags
        self.canvases.tile_batch.set(
            self.game.board
            .iter().enumerate()
            .filter_map(|(i, tile)| match *tile {
                TileType::Flag => Some(i),
                _ => None,
            })
            .map(|i| DrawParam::new()
                .dest(self.game.index_to_draw_coord(i))
                .src(
                    bound_rect(Rect::new(0.0, 27.0, 9.0, 9.0), self.assets.spritesheet_dimensions)
                )
            )
        );
        minefield_canvas.draw(&self.canvases.tile_batch, graphics::DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        minefield_canvas.finish(ctx)?;

        ctx.gfx.end_frame()?;
        Ok(())
    }
    fn draw_bombcounter(&mut self, ctx: &mut Context) -> GameResult {
        ctx.gfx.begin_frame().unwrap();

        let mut bombcounter_canvas = Canvas::from_image(ctx, self.canvases.bombcounter.clone(), Color::RED);
        /*
        // Draw the fancy border
        self.draw_9_slice(ctx, &mut minefield_canvas, Vec2::new(2.0, 2.0),
            Rect::new(27.0, 0.0, 5.0, 5.0),
             Rect::new(0.0, 0.0, self.canvases.minefield.width() as f32, self.canvases.minefield.height() as f32))?;

        // Draw cells
        self.canvases.tile_batch.set(
            self.game.board
            .iter().enumerate()
            .map(|(i, tile)| DrawParam::new()
                .dest(self.game.index_to_draw_coord(i))
                // And the source rect
                .src(
                    bound_rect(Rect::new(if *tile == TileType::Dug {18.0} else {0.0}, 0.0, 9.0, 9.0), self.assets.spritesheet_dimensions)
                )
            )
        );
        minefield_canvas.draw(&self.canvases.tile_batch, graphics::DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        // If there's a tile being selected, draw the pushed down version of the tile
        if let Some(index) = self.game.selected_tile {
            minefield_canvas.draw(&self.assets.spritesheet, graphics::DrawParam::new()
            .src(Rect::new(9.0 / 86.0, 0.0, 9.0 / 86.0, 9.0 / 36.0))
            .dest(self.game.index_to_draw_coord(index) + 2.0));
        }

        // Draw neighbour count
        // Positions of each number in the sprite sheet
        const COORDS: [(f32, f32); 8] = [
            ( 0.0,  9.0), // 1
            ( 9.0,  9.0), // 2
            (18.0,  9.0), // 3
            (27.0,  9.0), // 4
            ( 0.0, 18.0), // 5
            ( 9.0, 18.0), // 6
            (18.0, 18.0), // 7
            (27.0, 18.0), // 8
        ];

        self.canvases.tile_batch.set(
            self.game.neighbour_count
            .iter().enumerate()
            // Don't draw the neighbour count if it's zero or if the tile hasn't been dug yet
            .filter_map(|(i, count)| match self.game.board[i] != TileType::Dug || *count == 0 {
                false => Some((i, count)),
                true  => None,
            })
            .map(|(i, &count)| DrawParam::new()
                .dest(self.game.index_to_draw_coord(i))
                .src(
                    bound_rect(Rect::new(COORDS[count as usize - 1].0, COORDS[count as usize - 1].1, 9.0, 9.0), self.assets.spritesheet_dimensions)
                )
            )
        );
        minefield_canvas.draw(&self.canvases.tile_batch, graphics::DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        // Draw flags
        self.canvases.tile_batch.set(
            self.game.board
            .iter().enumerate()
            .filter_map(|(i, tile)| match *tile {
                TileType::Flag => Some(i),
                _ => None,
            })
            .map(|i| DrawParam::new()
                .dest(self.game.index_to_draw_coord(i))
                .src(
                    bound_rect(Rect::new(0.0, 27.0, 9.0, 9.0), self.assets.spritesheet_dimensions)
                )
            )
        );
        minefield_canvas.draw(&self.canvases.tile_batch, graphics::DrawParam::new().dest(Vec2::new(2.0, 2.0)));
        */
        bombcounter_canvas.finish(ctx)?;

        ctx.gfx.end_frame()?;
        Ok(())
    }
}

fn bound_rect(rect: Rect, bound: Vec2) -> Rect {
    Rect::new(rect.x / bound.x, rect.y / bound.y, rect.w / bound.x, rect.h / bound.y)
}

impl EventHandler for MainState {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // TODO: MAYBE
        // Come up with some kind of system that makes it so resizing isn't all pixely
        // As in, when we're rendering at a higher aspect ratio, the actual rects are drawn larger, maybe i could make a low-res 'holder' holds the whole
        // screen and is resized and aligned to the middle, making it nice and smooth.

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(192, 203, 220));

        let window_size = self.calculate_screen_coordinates(ctx.gfx.window().inner_size());

        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, window_size.x, window_size.y));
        canvas.set_sampler(FilterMode::Nearest);

        // Locate the middle of the screen + 10 on the y axis (to make room for the top bar) (10 comes from 20 / 2)
        let minefield_middle = window_size / 2.0 + Vec2::new(0.0, 10.0);
        // Then use this to find out where to draw the minefield
        let minefield_size = Vec2::new(self.canvases.minefield.width() as f32, self.canvases.minefield.height() as f32);
        // This is rounded to make sure it always aligns to the pixel grid
        let minefield_top_left = (minefield_middle - minefield_size / 2.0).round();
        canvas.draw(&self.canvases.minefield, DrawParam::new().dest(minefield_top_left));

        // We want to split the top of the screen into thirds and draw the three elements (bomb counter, button, and timer) up there.
        // TODO: Figure out how to make them balance easier.
        for i in 1..4 {
            let one_before = ((i as f32 - 1.0) * window_size.x / 3.0);
            let this_one = ((i as f32) * window_size.x / 3.0);
            let middle = (this_one+one_before)/2.0;


            let bombcounter_middle = Vec2::new(middle, 12.0);
            let bombcounter_size = Vec2::new(self.canvases.bombcounter.width() as f32, self.canvases.bombcounter.height() as f32);
            let bombcounter_top_left = (bombcounter_middle - bombcounter_size / 2.0).round();
    
            canvas.draw(&self.canvases.bombcounter, DrawParam::new().dest(bombcounter_top_left));
        }
        

        canvas.finish(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) -> GameResult {
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        if input.keycode == Some(KeyCode::W) {
            self.canvases.minefield = Image::new_canvas_image(&ctx.gfx, ctx.gfx.surface_format(), self.canvases.minefield.width() + 1, self.canvases.minefield.height() + 1, 1);
            self.draw_minefield(ctx)?;
        }
        Ok(())
    }
}