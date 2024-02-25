use ggez::conf::{WindowMode, WindowSetup};
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::dpi::{LogicalSize, PhysicalSize};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, FilterMode, Image, ImageFormat, Rect};
use ggez::event::{self, EventHandler};

use rand::prelude::*;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("MineSweeper", "jumbledFox")
        .window_mode(WindowMode {
            logical_size: Some(LogicalSize::new(95.0, 114.0)),
            resizable: true,
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
    let my_game = MainState::new(&mut ctx);
    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Assets {
    mockup: Image,
    spritesheet: Image,
}

struct Canvases {
    minefield: Canvas,
}

struct MainState {
    assets: Assets,
    canvases: Canvases,
    board: [[u8; 9]; 9],
}


impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        // Load/create resources such as images here.

        let mut p: [[u8; 9]; 9] = [[0; 9]; 9];
        for y in 0..9 {
        for x in 0..9 {
            p[y][x] = thread_rng().gen_range(0..9);
        }
        }

        MainState {
            assets: Assets {
                mockup:      Image::from_path(ctx, "/mockup.png").unwrap(),
                spritesheet: Image::from_path(ctx, "/spritesheet.png").unwrap(),
            },
            // canvases: Canvases { minefield: Image::new_canvas_image(ctx, ctx.gfx.surface_format(), 85, 85, 1) }
            // canvases: Canvases { minefield: Image::from_color(&ctx.gfx, 85, 85, Some(Color::RED)) }
            canvases: Canvases { minefield: graphics::Canvas::from_frame(ctx, Color::from_rgb(192, 203, 220)) },
            board: p,
        }
    }
}

impl EventHandler for MainState {
   fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(192, 203, 220));
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 95.0, 114.0));
        canvas.set_sampler(FilterMode::Nearest);

        // canvas.draw(&self.canvases.minefield, DrawParam::new().dest(Vec2::ZERO));

        for y in 0..9 {
        for x in 0..9 {
            canvas.draw(&self.assets.mockup, DrawParam::new()
                .src(Rect::new(25.0 / 95.0, 35.0 / 114.0, 9.0 / 95.0, 9.0 / 114.0))
                .dest(Vec2::new(x as f32 * 9.0, y as f32 * 9.0))
            );
        }
        }
        for y in 0..9 {
        for x in 0..9 {
            let coords = match self.board[y][x] {
                1 => (0.0,  9.0),
                2 => (9.0,  9.0),
                3 => (18.0, 9.0),
                4 => (27.0, 9.0),
                5 => (0.0,  18.0),
                6 => (9.0,  18.0),
                7 => (18.0, 18.0),
                8 => (27.0, 18.0),
                
                _ => (0.0, 0.0),
            };
            canvas.draw(&self.assets.spritesheet, DrawParam::new()
                .src(Rect::new(coords.0 / 86.0, coords.1 / 36.0, 9.0 / 86.0, 9.0 / 36.0))
                .dest(Vec2::new(x as f32 * 9.0, y as f32 * 9.0))
            );
        }
        }
        // canvas.draw(&self.assets.mockup, DrawParam::new()
        //     .src(Rect::new(5.0 / 95.0, 4.0 / 114.0, 24.0 / 95.0, 18.0 / 114.0))
        //     .dest(Vec2::new(10.0, 10.0))
        // );

        canvas.finish(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) -> GameResult {
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        if input.keycode == Some(KeyCode::D) {
            // self.canvases.minefield.draw(&self.assets.mockup, DrawParam::new().dest(Vec2::new(10.0, 10.0)));
        }
        Ok(())
    }
}