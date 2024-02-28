use std::time::{self, Duration, Instant};

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
    let (mut ctx, event_loop) = ContextBuilder::new("Minesweeper", "jumbledFox")
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

    let mut main_state = MainState::new(&mut ctx, 9, 9, 10);
    main_state.draw_all(&mut ctx);
    // Run!
    event::run(ctx, event_loop, main_state);
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        // TODO: When resizing, the selected tile calculates from where the mouse was for some reason which is odd
        
        // Update the selected tile
        let mouse_pos = Vec2::new(ctx.mouse.position().x, ctx.mouse.position().y);
        let minefield_inner_pos = Vec2::new(self.rendering.minefield.dest_rect.x, self.rendering.minefield.dest_rect.y);
        // We take away 2.0 to account for the border on the minefield
        let hovered_tile_coords = (((mouse_pos-minefield_inner_pos)/self.rendering.scale_factor-2.0)/9.0).floor();
        // If the mouse is over a valid tile, make it the selected one! Otherwise make the selected one None
        self.selected_tile = if hovered_tile_coords.x >= 0.0 && hovered_tile_coords.x < self.game.width  as f32 &&
                                hovered_tile_coords.y >= 0.0 && hovered_tile_coords.y < self.game.height as f32
        {
            let hovering_index = hovered_tile_coords.x as usize % self.game.width + hovered_tile_coords.y as usize * self.game.width;
            // If the tile we WERE hovering over is in a different position to the current one, or is None:
            // Make it so we're no-longer holding down a tile, AND, if we're flagging, set the flag state of the new tile
            if !self.selected_tile.is_some_and(|x| x == hovering_index) {
                self.rendering.redraw = true;
                self.holding_button = false;
                if let Some(flagging_mode) = self.flagging_mode {
                    self.game.flag(flagging_mode, hovering_index);
                }
            }
            Some(hovering_index)
        } else {
            None
        };

        if self.rendering.redraw {
            self.rendering.redraw = false;
            self.draw_all(ctx)?;
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: event::MouseButton, _x: f32,_y: f32) -> GameResult {
        // TODO: Care about states
        match button {
            event::MouseButton::Left => { self.holding_button = true; },
            event::MouseButton::Right => {
                // Start flagging
                self.flagging_mode = if let Some(index) = self.selected_tile {
                    self.holding_button = false;
                    // If the tile isnt a flag, we want to set it to one, and vice versa
                    let f_mode = self.game.board[index] != TileType::Flag;
                    // And then we want to actually update the one we're currently selecting
                    // This is because flags are only changed when we hover over a different cell
                    self.game.flag(f_mode, index);
                    Some(f_mode)
                } else {
                    // If we're not selecting a tile, make it so when we do we're adding flags, as that's the expected behaviour
                    Some(true)
                };
            }
            _ => {}
        }
        self.rendering.redraw = true;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: event::MouseButton, _x: f32,_y: f32) -> GameResult {
        // TODO: Care about states
        match button {
            event::MouseButton::Left  => {
                // If we were holding down on a cell and we've just let go.. dig it up!!
                if self.holding_button && self.selected_tile.is_some() {
                    self.game.dig(self.selected_tile.unwrap());
                }
                self.holding_button = false;
            },
            event::MouseButton::Right => { self.flagging_mode = None; },
            _ => { return Ok(()); }
        }
        self.rendering.redraw = true;
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

        if let Some(index) = self.selected_tile {
            let pos = Vec2::new((index % self.game.width) as f32, (index / self.game.width) as f32) * 9.0 * self.rendering.scale_factor;
            let relative_pos = Vec2::new(self.rendering.minefield.dest_rect.x, self.rendering.minefield.dest_rect.y) + self.rendering.scale_factor + pos;
            canvas.draw(&self.rendering.spritesheet, DrawParam::new().src(
                normalize_rect(Rect::new(73.0, 28.0, 11.0, 11.0), &self.rendering.spritesheet))
                .dest_rect(Rect::new(relative_pos.x, relative_pos.y, self.rendering.scale_factor, self.rendering.scale_factor))
            );
        }

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
        if input.keycode == Some(KeyCode::Key1) {
            self.new_game(ctx, 6, 6, 5)?;
        }
        if input.keycode == Some(KeyCode::Key2) {
            self.new_game(ctx, 9, 9, 10)?;
        }
        if input.keycode == Some(KeyCode::Key3) {
            self.new_game(ctx, 15, 13, 40)?;
        }
        if input.keycode == Some(KeyCode::Key4) {
            self.new_game(ctx, 30, 16, 99)?;
        }
        if input.keycode == Some(KeyCode::Key5) {
            self.new_game(ctx, 50, 24, 250)?;
        }
        Ok(())
    }
}