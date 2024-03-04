use ggez::glam::Vec2;
use ggez::{event::EventHandler, Context, GameResult};

use crate::gui::{Gui, MenuBar};
use crate::minesweeper::{self, Minesweeper};
use crate::rendering::Rendering;

pub struct MainState {
    game: Minesweeper,
    // The 'Minesweeper' game class should be a black box that you can query, and not be linked with this programs rendering or logic code
    // That's why 'selected_tile' is defined here. 
    selected_tile: Option<usize>,
    mouse_down: bool,
    erasing_flags: bool,

    gui: Gui,
    rendering: Rendering,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let game = Minesweeper::new(10, 10, 9);
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
                Some(String::from("Easy      ¬¬9*9,¬10")),
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
        let rendering = Rendering::new(ctx, tr, (game.width, game.height), menu_bar.height+2.0);

        let gui = Gui::new(menu_bar);
        MainState { game, rendering, gui, selected_tile: Some(42), mouse_down: false, erasing_flags: false }
    }

    fn new_game(&mut self, ctx: &mut Context, width: usize, height: usize, bomb_count: usize) {
        self.game = Minesweeper::new(width, height, bomb_count);
        self.rendering.new_game(ctx, (self.game.width, self.game.height));
    }

    // Returns if the mouse was over the GUI
    fn gui_logic(&mut self, ctx: &mut Context, mouse_pos: Vec2) -> bool {
        self.gui.update(mouse_pos, crate::gui::MousePressMode::None);
        // TODO: Add confirmation pop-ups and text boxes.
        // Exiting
        if self.gui.menu_bar.menu_button_pressed(0, 7) {
            ctx.gfx.window().set_visible(false);
            ctx.request_quit();
        }
        // Make new games if the buttons are pressed
        if self.gui.menu_bar.menu_button_pressed(0, 2) {
            self.new_game(ctx, 10, 10, 11);
        }
        if self.gui.menu_bar.menu_button_pressed(0, 3) {
            self.new_game(ctx, 15, 13, 40);
        }
        if self.gui.menu_bar.menu_button_pressed(0, 4) {
            self.new_game(ctx, 30, 16, 99);
        }
        if self.gui.menu_bar.menu_button_pressed(0, 5) {
            self.new_game(ctx, 200, 100, 250);
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

    fn dig(&mut self) {
        // If we've not selected a tile, don't do anything!!
        if self.selected_tile.is_none() { return; }
        let selected_tile = self.selected_tile.unwrap();

        // Dig at the selected tile, if it didn't do anything return
        if !self.game.dig(selected_tile) { return; }
        self.rendering.redraw_minefield();

        // Check if this made us win the game or not...
    }

    fn flag(&mut self) {
        // If we've not selected a tile, don't do anything!!!!!
        if self.selected_tile.is_none() { return; }
        let selected_tile = self.selected_tile.unwrap();

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

        if self.gui_logic(ctx, mouse_pos) {
            self.selected_tile = None;
        } else {
            self.selected_tile_logic(mouse_pos);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let selection_depressed = self.selected_tile.is_some_and(|s| self.game.board[s] == minesweeper::TileType::Unopened) && self.mouse_down;
        self.rendering.render(ctx, &self.gui, &self.game, self.selected_tile, selection_depressed)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: ggez::event::MouseButton, x: f32, y: f32) -> GameResult {
        match button {
            ggez::event::MouseButton::Left  => self.mouse_down = true,
            // FLAGGING
            ggez::event::MouseButton::Right => { self.flag(); },
            _ => {}
        }
        self.gui.update(self.rendering.scaled_mouse_pos(x, y), crate::gui::MousePressMode::Down);
        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: ggez::event::MouseButton, x: f32, y: f32) -> GameResult {
        match button {
            // Digging!
            ggez::event::MouseButton::Left  => {
                self.mouse_down = false;
                self.dig();
            }
            ggez::event::MouseButton::Right => {
                self.erasing_flags = false;
            }
            _ => {}
        }
        self.gui.update(self.rendering.scaled_mouse_pos(x, y), crate::gui::MousePressMode::Up);
        Ok(())
    }
}