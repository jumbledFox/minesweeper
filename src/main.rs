use std::collections::HashMap;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawParam, FilterMode, Image, Rect};
use ggez::event::{self, EventHandler};

use mainstate::{normalize_rect, MainState};

pub mod minesweeper;
pub mod mainstate;
pub mod gui;
use minesweeper::GameState;
use rand::{thread_rng, Rng};

use crate::minesweeper::TileType;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Minesweeper", "jumbledFox")
        .window_mode(WindowMode {
            resizable: false,
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

    // let mut main_state = MainState::new(&mut ctx, 210, 106, 100);
    // let mut main_state = MainState::new(&mut ctx, 8, 8, 2);
    // let mut main_state = MainState::new(&mut ctx, 200, 100, 2);
    let mut main_state = MainState::new(&mut ctx, 30, 16, 99);
    main_state.draw_all(&mut ctx);
    // Run!
    event::run(ctx, event_loop, main_state);
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.rendering.select_menu.active = true;
        
        // Update the selected tile
        let mouse_pos = Vec2::new(ctx.mouse.position().x, ctx.mouse.position().y);
        self.rendering.b.update(mouse_pos, gui::button::MouseMode::None);
        if self.rendering.b.pressed() {
            println!("pressed!!!!!!!!!!!!!!!!!!!!!!!!!");
        }

        let minefield_inner_pos = Vec2::new(self.rendering.minefield.dest_rect.x, self.rendering.minefield.dest_rect.y);
        // We take away 2.0 to account for the border on the minefield
        let hovered_tile_coords = (((mouse_pos-minefield_inner_pos)/self.rendering.scale_factor-2.0)/9.0).floor();
        // If we're hovering over a new tile (and the mouse is in the window)
        if self.last_hovered_tile != hovered_tile_coords && self.rendering.mouse_in_window  {
            self.last_hovered_tile = hovered_tile_coords;
            // Check if we're hovering over an actual tileS
            let hovered_tile_on_board =  hovered_tile_coords.x >= 0.0 && hovered_tile_coords.x < self.game.width as f32 && hovered_tile_coords.y >= 0.0 && hovered_tile_coords.y < self.game.height as f32;
            self.selected_tile = if hovered_tile_on_board {
                // If we're hovering over an actual tile, work out where it is!!!
                let hovering_index = hovered_tile_coords.x as usize % self.game.width + hovered_tile_coords.y as usize * self.game.width;
                // Remove the flag at this position if we should (and redraw!)
                if self.erasing_flags {
                    if self.game.set_flag(true, hovering_index) {
        self.i = 0;
        self.rendering.redraw = true;
                    }
                }
                Some(hovering_index)
            } else { None }
        }

        // println!("{:?}", Rect::new(self.rendering.button.dest_rect.x, self.rendering.button.dest_rect.y,
        //     self.rendering.button.dest_rect.w * self.rendering.button.img.width()  as f32,
        //     self.rendering.button.dest_rect.h * self.rendering.button.img.height() as f32,
        // ).contains(mouse_pos));

        self.rendering.menubar.update(mouse_pos, gui::button::MouseMode::None);

        for b in &mut self.rendering.menubar.items {
            if b.pressed() {
                println!("{:?} pressed!", &b.label);
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: event::MouseButton, x: f32,y: f32) -> GameResult {
        self.rendering.menubar.update(Vec2::new(x, y), gui::button::MouseMode::Down);
        self.rendering.b.update(Vec2::new(x, y), gui::button::MouseMode::Down);

        // TODO: Care about states
        match button {
            event::MouseButton::Left => { self.holding_button = true; },
            event::MouseButton::Right => {
                self.holding_button = false;
                // We only want to start erasing flags if we right click on a flag
                self.erasing_flags = self.selected_tile.is_some_and(|i| self.game.board.get(i).is_some_and(|t| *t == TileType::Flag));
                if let Some(index) = self.selected_tile {
                    self.game.set_flag(self.erasing_flags, index);
                }
            }
            _ => {}
        }
        self.rendering.redraw = true;
        self.i = 0;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: event::MouseButton, x: f32,y: f32) -> GameResult {
        self.rendering.menubar.update(Vec2::new(x, y), gui::button::MouseMode::Up);
        self.rendering.b.update(Vec2::new(x, y), gui::button::MouseMode::Up);

        // TODO: Care about states
        match button {
            event::MouseButton::Left  => {
                // If we were holding down on a cell and we've just let go.. dig it up!!
                if self.holding_button && self.selected_tile.is_some() {
                    self.game.dig(self.selected_tile.unwrap());
                }
                self.holding_button = false;
            },
            event::MouseButton::Right => { self.erasing_flags = false; },
            _ => { return Ok(()); }
        }
        self.i = 0;
        self.rendering.redraw = true;
        Ok(())
    }

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, entered: bool) -> GameResult {
        self.rendering.mouse_in_window = entered;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {        
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(192, 203, 220));
        // canvas.set_screen_coordinates(Rect::new(0.0, 0.0, 400.0, 300.0));
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
        if self.i == 0 {
            // self.i = 1;
            self.draw_minefield(ctx)?;
        }

        canvas.draw(&self.rendering.bombcount.img, DrawParam::new().dest_rect(self.rendering.bombcount.dest_rect));
        canvas.draw(&self.rendering.timer    .img, DrawParam::new().dest_rect(self.rendering.timer    .dest_rect));
        canvas.draw(&self.rendering.button   .img, DrawParam::new().dest_rect(self.rendering.button   .dest_rect));
        canvas.draw(&self.rendering.minefield.img, DrawParam::new().dest_rect(self.rendering.minefield.dest_rect));

        if let Some(index) = self.selected_tile {
            let pos = Vec2::new((index % self.game.width) as f32, (index / self.game.width) as f32) * 9.0 * self.rendering.scale_factor;
            let relative_pos = Vec2::new(self.rendering.minefield.dest_rect.x, self.rendering.minefield.dest_rect.y) + self.rendering.scale_factor + pos;
            // Draw selection thing
            canvas.draw(&self.rendering.spritesheet, DrawParam::new().src(
                normalize_rect(Rect::new(73.0, 28.0, 11.0, 11.0), &self.rendering.spritesheet))
                .dest_rect(Rect::new(relative_pos.x, relative_pos.y, self.rendering.scale_factor, self.rendering.scale_factor))
            );
            // If holding down and the cell is clickable, draw depressed tile
            if self.holding_button && self.game.board.get(index).is_some_and(|t| *t == TileType::Unopened) {
                canvas.draw(&self.rendering.spritesheet, DrawParam::new()
                    .src(normalize_rect(Rect::new(9.0, 0.0, 9.0, 9.0), &self.rendering.spritesheet))
                    .dest_rect(Rect::new(relative_pos.x + self.rendering.scale_factor, relative_pos.y + self.rendering.scale_factor,
                        self.rendering.scale_factor, self.rendering.scale_factor))
                );
            }
        }

        // self.rendering.tr.draw_text(&mut canvas, &String::from("does text rendering work? I THINK it does!!! :) :( :P :-)"), DrawParam::new().color(Color::MAGENTA));
        // self.rendering.tr.draw_text(&mut canvas, &String::from("black sphinx of quartz, \"judge\" my vow!"), DrawParam::new().color(Color::RED).dest(Vec2::new(1.0, 10.0)));
        // self.rendering.tr.draw_text(&mut canvas, &String::from("BLACK, SPHINX. OF! QUARTZ? JUDGE: MY; 'VOW'"), DrawParam::new().color(Color::BLACK).dest(Vec2::new(1.0, 20.0)));
        // self.rendering.tr.draw_text(&mut canvas, &String::from("123, 456, numbers!!! >:3 8-200  Width Submit Game Help"), DrawParam::new().color(Color::BLUE).dest(Vec2::new(1.0, 30.0)));
        // self.rendering.tr.draw_text(&mut canvas, &String::from("This is the test of a longer bit of text. I'm using a line break HERE ->\nand hopefully it should work."), DrawParam::new().color(Color::BLACK).dest(Vec2::new(1.0, 40.0)));
        
        let t = String::from("Short line\nLOOOOOOOOOOOOOOOOOOOOONG line!!!\nshorter line");
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(Vec2::new(10.0, 50.0))
                .scale(self.rendering.tr.text_size(&t))
                .color(Color::MAGENTA),
        );
        self.rendering.tr.draw_text(&mut canvas, &t, DrawParam::new().color(Color::BLACK).dest(Vec2::new(10.0, 50.0)));

        self.rendering.menubar.render(&mut canvas, &mut self.rendering.tr);
        
        canvas.finish(ctx)
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> GameResult {
        Ok(())
    }
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(k) = input.keycode {
            self.rendering.custom_menu.number_inputs[0].add(k);
        }
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        if input.keycode == Some(KeyCode::Q) {
            println!("lose game");
            self.game.state = GameState::Lose;
            // self.draw_all(ctx)?;
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