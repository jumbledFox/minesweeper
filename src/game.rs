use std::time::Instant;

use ggez::glam::Vec2;
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::{event::EventHandler, Context, GameResult};
use rand::seq::IteratorRandom;
use rand::Rng;

use crate::gui::{self, Gui, MenuBar};
use crate::minesweeper::{self, Minesweeper};
use crate::rendering::Rendering;

pub struct MainState {
    game: Minesweeper,
    // The 'Minesweeper' game struct should be a black box that you can query, and not be linked with this programs rendering or logic code
    // That's why some minesweeper related things are defined here
    difficulty: minesweeper::Difficulty,
    selected_tile: Option<usize>,
    holding_tile: bool,  // If we're holding down the mouse on a tile
    erasing_flags: bool, // If we're erasing flags or not
    button: gui::Button,
    // Animating the loss screen
    last_explosion: Instant, // When the previous bomb exploded
    time_until_next_explosion: u128,    // How many ms we should wait until we explode another one

    gui: Gui,
    rendering: Rendering,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let difficulty = minesweeper::Difficulty::Easy;
        let game = Minesweeper::new(difficulty);
        let tr = Rendering::new_text_renderer(ctx);

        /* Maybe dropdowns could work like this
        let menu_bar_items_test = vec![
            (String::from("Game"), 0.0,
            vec![
                (Toggle, vec![
                    Some(String::from("Use ?"))]),
                (Button, vec![
                    Some(String::from("New game"))]),
                (SelectOne , vec![
                    Some(String::from("Easy      ¬¬9*9,¬10")),
                    Some(String::from("Normal 15*13,40")),
                    Some(String::from("Hard   30*16,99")),
                    Some(String::from("Custom...")),
                ]),
                (Button, vec![
                    Some(String::from("Exit..."))]),
            ]),
            (String::from("Scale"), 0.0,
            vec![
                (true , vec![
                    Some(String::from(" 1x ")), Some(String::from(" 2x ")), Some(String::from(" 3x ")), Some(String::from(" 4x ")),
                    Some(String::from(" 5x ")), Some(String::from(" 6x ")), Some(String::from(" 7x ")), Some(String::from(" 8x ")),
                ]),
            ]),
        ];
        */
        let menu_bar_items = vec![
            (String::from("Game"),  0.0, vec![
                Some(String::from("New game")),
                None,
                Some(String::from("Easy    10*10, 9")),
                Some(String::from("Normal 15*13,40")),
                Some(String::from("Hard   30*16,99")),
                Some(String::from("Custom...")),
                None,
                Some(String::from("Exit"))]),
            (String::from("Scale"), 0.0, vec![
                Some(String::from(" 1x ")), Some(String::from(" 2x ")), Some(String::from(" 3x ")), Some(String::from(" 4x ")),
                Some(String::from(" 5x ")), Some(String::from(" 6x ")), Some(String::from(" 7x ")), Some(String::from(" 8x "))]),
            (String::from("Help"),  0.0, vec![
                Some(String::from("How to play")),
                Some(String::from("About"))]
        )];

        let menu_bar = MenuBar::new(&tr, menu_bar_items);
        let rendering = Rendering::new(ctx, tr, (game.width, game.height, game.bomb_count), menu_bar.height+2.0);
        let button = gui::Button::new(Rect::new(
            ((rendering.window_size.x-19.0)/2.0).floor(),
            rendering.menu_bar_height+2.0, 19.0, 19.0), gui::button::PressMode::Release, false);

        let gui = Gui::new(menu_bar);
        MainState {
            game, difficulty,
            selected_tile: None, holding_tile: false, erasing_flags: false,
            button,
            last_explosion: Instant::now(), time_until_next_explosion: 0,
            gui, rendering
        }
    }

    fn new_game(&mut self, ctx: &mut Context, difficulty: minesweeper::Difficulty) {
        self.difficulty = difficulty;
        self.game = Minesweeper::new(self.difficulty);
        self.time_until_next_explosion = 0;
        self.rendering.new_game(ctx, (self.game.width, self.game.height, self.game.bomb_count));
        self.button = gui::Button::new(Rect::new(
            ((self.rendering.window_size.x-19.0)/2.0).floor(),
            self.rendering.menu_bar_height+2.0, 19.0, 19.0), gui::button::PressMode::Release, false);
    }

    fn selected_tile_diggable(&self) -> bool {
        self.selected_tile.is_some_and(|s| self.game.board[s] == minesweeper::TileType::Unopened)
    }

    // Updates the gui, returns if the mouse was over the GUI
    fn gui_logic(&mut self, ctx: &mut Context, mouse_pos: Vec2) -> bool {
        self.gui.update(mouse_pos, crate::gui::MousePressMode::None);
        // TODO: Add confirmation pop-ups and text boxes.
        // Exiting
        if self.gui.menu_bar.menu_button_pressed(0, 7) {
            // gui.popup(Popup::Quit);
            ctx.gfx.window().set_visible(false);
            ctx.request_quit();
        }
        // Make new games if the buttons are pressed
        if self.gui.menu_bar.menu_button_pressed(0, 0) {
            // gui.popup(Popup::NewGame(difficulty));
            self.new_game(ctx, self.difficulty);
        }
        if self.gui.menu_bar.menu_button_pressed(0, 2) {
            self.new_game(ctx, minesweeper::Difficulty::Easy);
        }
        if self.gui.menu_bar.menu_button_pressed(0, 3) {
            self.new_game(ctx, minesweeper::Difficulty::Normal);
        }
        if self.gui.menu_bar.menu_button_pressed(0, 4) {
            self.new_game(ctx, minesweeper::Difficulty::Hard);
        }
        if self.gui.menu_bar.menu_button_pressed(0, 5) {
            self.new_game(ctx, minesweeper::Difficulty::Custom(200, 100, 250));
        }
        // Update scale factor if one of the scale buttons is pressed
        for i in 0..8 {
            if self.gui.menu_bar.menu_button_pressed(1, i) {
                self.rendering.resize(ctx, i+1);
            }
        }

        self.gui.hovering(mouse_pos)
    }

    fn selected_tile_logic(&mut self, mouse_pos: Vec2) {
        let hovered_tile = (mouse_pos - self.rendering.minefield_pos - 2.0) / 9.0;
        // Check the bounds
        if  hovered_tile.x < 0.0 || hovered_tile.x > self.game.width  as f32 || 
            hovered_tile.y < 0.0 || hovered_tile.y > self.game.height as f32 {
            self.selected_tile = None;
            return;
        }
        let hovered_tile_index = hovered_tile.y as usize * self.game.width + hovered_tile.x as usize ;
        // If the selected tile has changed to another valid tile...
        if self.selected_tile != Some(hovered_tile_index) {
            // If we're erasing flags.. do that (and redraw the minefield if we must)
            if self.erasing_flags {
                if self.game.set_flag(self.erasing_flags, hovered_tile_index) { self.rendering.redraw_minefield(); }
            }
            self.selected_tile = Some(hovered_tile_index);
        }
    }

    fn lose(&mut self) {
        self.game.state = minesweeper::GameState::Lose;
    }

    fn explode_bomb(&mut self, specific_bomb: Option<usize>) {
        // If we have a specific bomb to explode (and it exists and hasn't been exploded yet), explode that, otherwise explode a random one!!!
        let bomb_index = match specific_bomb {
            Some(b) if self.game.bombs.contains(&b) && !self.rendering.exploded_bombs.contains(&b) && self.game.board[b] != minesweeper::TileType::Flag => b,
            _ => {
                // Find a random bomb
                let random_bomb = self.game.bombs.iter().filter(|b|
                    !self.rendering.exploded_bombs.contains(b) && self.game.board[**b] != minesweeper::TileType::Flag).choose_multiple(&mut rand::thread_rng(), 1);
                // If we managed to get one, explode it, otherwise return
                match random_bomb.get(0) {
                    Some(&&b) => b,
                    _ => return,
                }
            }
        };
        self.rendering.exploded_bombs.push(bomb_index);
        self.time_until_next_explosion = rand::thread_rng().gen_range(50..500);
        self.last_explosion = Instant::now();
        self.rendering.play_explosion_sound();
        self.rendering.redraw_minefield();
    }

    fn dig(&mut self) {
        // If we've not selected a tile, don't do anything!!
        let selected_tile = match self.selected_tile {
            Some(i) => i,
            None => return,
        };

        // Dig at the selected tile, if it didn't do anything return
        if !self.game.dig(selected_tile) { return; }
        
        // Check if this made us lose the game
        if self.game.bombs.contains(&selected_tile) {
            self.rendering.losing_tile = Some(selected_tile);
            self.explode_bomb(Some(selected_tile));
            self.lose();
        }
        // Check if it made us win the game

        // I think I'm going crazy, but for some reason I feel like crossed out flags appear a SPLIT second after the board is redrawn with bombs showing,
        // but I used a screen recorder and stepped through each frame and there wasn't a delay?? so I'm going crazy.
        self.rendering.redraw_minefield();
    }

    fn flag(&mut self) {
        // If we've not selected a tile, don't do anything!!!!!
        let selected_tile = match self.selected_tile {
            Some(i) => i,
            None => return,
        };

        // If we're hovering over a tile and press the right mouse button we want to do one of two things, place a flag or erase a flag
        // If the tile isn't a flag, we want to try and place one, if it IS a flag, we want to start erasing them!
        self.erasing_flags = self.game.board[selected_tile] == minesweeper::TileType::Flag;
        // Flag the selected tile, if it didn't do anything return
        if !self.game.set_flag(self.erasing_flags, selected_tile) { return };
        self.rendering.redraw_minefield();
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_pos = self.rendering.mouse_pos(ctx);

        /*
        TODO: maybe this kind of thing can work ??

        if let Some(gui.popup) = popup {
            match gui.popup.result() {
                Exit       => /* exit */,
                NewGame(d) => self.new_game(d),
                _ => ()
            }
        } else {
            // update
        }
         */
        // if let Some(gui.popup) = popup
        if self.gui_logic(ctx, mouse_pos) {
            self.selected_tile = None;
        } else {
            self.selected_tile_logic(mouse_pos);
            self.button.update(mouse_pos, gui::MousePressMode::None);
        }
        if self.button.pressed() {
            self.new_game(ctx, self.difficulty);
        }

        // If the game has been lost and there are bombs to explode... explode a random one!
        let explodable_mines_count = self.game.bombs.iter().filter(|&&b| self.game.board[b] != minesweeper::TileType::Flag).count();
        if self.game.state == minesweeper::GameState::Lose && self.rendering.exploded_bombs.len() != explodable_mines_count {
            if self.last_explosion.elapsed().as_millis() > self.time_until_next_explosion {
                self.explode_bomb(None);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let tile_held = self.selected_tile_diggable() && self.holding_tile;
        self.rendering.draw(ctx, &self.gui, &self.game, (self.selected_tile, tile_held), &self.button)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: ggez::event::MouseButton, x: f32, y: f32) -> GameResult {
        let mouse_pos = self.rendering.scaled_mouse_pos(x, y);
        // Update our button if we're not hovering over the gui
        if !self.gui.hovering(mouse_pos) {
            self.button.update(mouse_pos, gui::MousePressMode::Down);
        }
        // Update the gui
        self.gui.update(mouse_pos, crate::gui::MousePressMode::Down);
        // If we're not playing the game, we don't wanna interact with it now, do we?
        if !self.game.playing_state() { return Ok(()) }

        match button {
            ggez::event::MouseButton::Left  => {
                self.rendering.reset_button_looking();
                if self.selected_tile_diggable() { self.holding_tile = true }
            },
            // FLAGGING
            ggez::event::MouseButton::Right => { self.flag(); },
            _ => {}
        }
        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: ggez::event::MouseButton, x: f32, y: f32) -> GameResult {
        let mouse_pos = self.rendering.scaled_mouse_pos(x, y);
        // Update our button if we're not hovering over the gui
        if !self.gui.hovering(mouse_pos) {
            self.button.update(mouse_pos, gui::MousePressMode::Up);
        }
        // Update the gui
        self.gui.update(mouse_pos, crate::gui::MousePressMode::Up);

        // If we're not playing the game, we don't wanna interact with it!!
        if !self.game.playing_state() { return Ok(()) }

        match button {
            // Digging!
            ggez::event::MouseButton::Left  => {
                if self.holding_tile {
                    self.holding_tile = false;
                    self.dig();
                }
            }
            ggez::event::MouseButton::Right => {
                self.erasing_flags = false;
            }
            _ => {}
        }
        Ok(())
    }
}