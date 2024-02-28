use std::time::{Duration, Instant};

use ggez::conf::{WindowMode, WindowSetup};
use ggez::context::HasMut;
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::dpi::{LogicalSize, PhysicalSize};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawParam, Drawable, FilterMode, GraphicsContext, Image, ImageFormat, InstanceArray, Rect, ScreenImage};
use ggez::event::{self, EventHandler};

use mainstate::{normalize_rect, MainState};
use rand::prelude::*;

pub mod minesweeper;
pub mod mainstate;
use minesweeper::{GameState, Minesweeper};

use crate::minesweeper::TileType;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("MineSweeper", "jumbledFox")
        .window_mode(WindowMode {
            resizable: true,
            visible: false,
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
    // let my_game = MainState2::new(&mut ctx, game, min_window_size);

    let mut main_state = MainState::new(&mut ctx, 9, 9, 1990);
    main_state.draw_all(&mut ctx);
    // Run!
    event::run(ctx, event_loop, main_state);
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...
        if self.game.state == GameState::Playing {
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {        
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(192, 203, 220));
        canvas.set_sampler(FilterMode::Nearest);
        
        self.update_window_size(ctx.gfx.window().inner_size());

        // Redraw the timer if it's value has changed
        let game_timer_elapsed = match self.game.state {
            GameState::Prelude => None,
            GameState::Playing => Some(self.game.start_time.elapsed().as_secs() as usize),
            _ => self.rendering.timer_value,
        };
        if game_timer_elapsed != self.rendering.timer_value {
            self.rendering.timer_value = game_timer_elapsed;
            self.draw_timer(ctx)?;
        }
        self.draw_bombcount(ctx)?;

        canvas.draw(&self.rendering.bombcount.img, DrawParam::new().dest_rect(self.rendering.bombcount.dest_rect));
        canvas.draw(&self.rendering.timer    .img, DrawParam::new().dest_rect(self.rendering.timer    .dest_rect));
        canvas.draw(&self.rendering.button   .img, DrawParam::new().dest_rect(self.rendering.button   .dest_rect));
        canvas.draw(&self.rendering.minefield.img, DrawParam::new().dest_rect(self.rendering.minefield.dest_rect));

        if let Some(s) = self.selected_cell {
            let pos = Vec2::new((s % self.game.width) as f32, (s / self.game.width) as f32) * 9.0 * self.rendering.scale_factor;
            let relative_pos = Vec2::new(self.rendering.minefield.dest_rect.x, self.rendering.minefield.dest_rect.y) + Vec2::ONE * self.rendering.scale_factor + pos;
            canvas.draw(&self.rendering.spritesheet, DrawParam::new().src(
                normalize_rect(Rect::new(73.0, 28.0, 11.0, 11.0), &self.rendering.spritesheet))
                .dest_rect(Rect::new(relative_pos.x, relative_pos.y, self.rendering.scale_factor, self.rendering.scale_factor))
            );
        }
        
        // self.selected_cell = Some((self.selected_cell.unwrap()+1).rem_euclid(self.game.width * self.game.height));
        
    
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        if input.keycode == Some(KeyCode::Q) {
            println!("lose game");
            self.game.state = GameState::Lose;
            self.draw_all(ctx)?;
        }
        if input.keycode == Some(KeyCode::Space) {
            println!("dug");
            self.game.dig();
        }
        if input.keycode == Some(KeyCode::Key1) {
            self.new_game(5, 5, 5);
            self.draw_all(ctx)?;
        }
        if input.keycode == Some(KeyCode::Key2) {
            self.new_game(9, 9, 10);
            self.draw_all(ctx)?;
        }
        if input.keycode == Some(KeyCode::Key3) {
            self.new_game(15, 13, 40);
            self.draw_all(ctx)?;
        }
        if input.keycode == Some(KeyCode::Key4) {
            self.new_game(30, 16, 99);
            self.draw_all(ctx)?;
        }
        if input.keycode == Some(KeyCode::Key5) {
            self.new_game(50, 24, 250);
            self.draw_all(ctx)?;
        }
        Ok(())
    }
}