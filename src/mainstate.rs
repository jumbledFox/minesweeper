use core::f32;

use ggez::input::keyboard::{self, KeyCode};
use ggez::winit::dpi::{LogicalSize, PhysicalSize};
use ggez::{graphics, Context, GameResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, InstanceArray, Rect, Sampler};

use crate::minesweeper::{GameState, Minesweeper, TileType};

// TODO: think about how we render - maybe upscaling a smaller canvas is better than a bunch of upscaled smaller ones meshed together ?? makes drawing easier

pub struct NumberInput {
    pub value: Option<usize>,
    pub min: usize,
    pub max: usize,
    pub max_length: usize,
    pub valid: bool,
}

impl NumberInput {
    pub fn new(min: usize, max: usize, max_length: usize) -> NumberInput {
        NumberInput { value: None, min, max, max_length, valid: false }
    }
    pub fn add(&mut self, keycode: KeyCode) {
        let num_to_push;
        match keycode {
            // If we press backspace, divide the number by 10
            KeyCode::Back => {
                self.value = if self.value.is_some_and(|v| v>=10) { Some(self.value.unwrap()/10) } else { None };
                return;
            }
            KeyCode::Key0 | KeyCode::Numpad0 => { num_to_push = 0; }
            KeyCode::Key1 | KeyCode::Numpad1 => { num_to_push = 1; }
            KeyCode::Key2 | KeyCode::Numpad2 => { num_to_push = 2; }
            KeyCode::Key3 | KeyCode::Numpad3 => { num_to_push = 3; }
            KeyCode::Key4 | KeyCode::Numpad4 => { num_to_push = 4; }
            KeyCode::Key5 | KeyCode::Numpad5 => { num_to_push = 5; }
            KeyCode::Key6 | KeyCode::Numpad6 => { num_to_push = 6; }
            KeyCode::Key7 | KeyCode::Numpad7 => { num_to_push = 7; }
            KeyCode::Key8 | KeyCode::Numpad8 => { num_to_push = 8; }
            KeyCode::Key9 | KeyCode::Numpad9 => { num_to_push = 9; }
            _ => {return;}
        }
        let new_value = if let Some(v) = self.value {
            Some(v*10+num_to_push)
        } else {
            Some(num_to_push)
        };
        if self.length_valid(new_value) {
            self.value = new_value
        }
    }
    pub fn length_valid(&self, value: Option<usize>) -> bool {
        if let Some(v) = value { v.checked_ilog10().unwrap_or(0)as usize+1 <= self.max_length } else { true }
    }
    pub fn validity(&mut self) -> bool {
        // If it's more than or equal to min, less than or equal to max, and the amount of digits is less than or equal to the maximum length
        self.valid = self.value.is_some_and(|v| v >= self.min && v <= self.max) && self.length_valid(self.value);
        self.valid
    }
}

pub struct Menu {
    pub showing: bool,
    pub buttons: Vec<Rect>,
    pub number_inputs: Vec<NumberInput>,
    pub gui_element: GuiElement,
}

impl Menu {
    pub fn new(ctx: &Context, width: usize, height: usize, buttons: Vec<Rect>, number_inputs: Vec<NumberInput>) -> Menu {
        let img = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), 19, 19, 1);
        let gui_element = GuiElement::new(img);
        Menu {showing: false, buttons, number_inputs, gui_element }
    }
    pub fn hovering_button(&self, ctx: &Context) -> Option<usize> {
        let r = self.gui_element.dest_rect;
        for (i, b) in self.buttons.iter().enumerate() {
            let b_t = Rect::new(b.x + r.x, b.y + r.y, b.w * r.w, b.h * r.w);
            if b_t.contains(ctx.mouse.position()) { return Some(i); }
        }
        None
    }
}

pub struct GuiElement {
    pub middle: Vec2,
    pub dest_rect: Rect,
    pub img: Image,
}

impl GuiElement {
    pub fn new(img: Image) -> GuiElement {
        GuiElement { 
            middle: Vec2::new(img.width() as f32 / 2.0, img.height() as f32 / 2.0),
            dest_rect: Rect::one(),
            img,
        }
    }
    pub fn goto(&mut self, pos: Vec2, scale_factor: f32) {
        self.dest_rect = Rect::new(pos.x.floor(), pos.y.floor(), scale_factor, scale_factor);
    }
}

pub struct Rendering {
    // For batch rendering of elements of the spritesheet
    pub spritesheet: Image,
    pub spritesheet_batch: InstanceArray,
    // The different drawable images
    pub minefield: GuiElement,
    pub bombcount: GuiElement,
    pub timer: GuiElement,
    pub button: GuiElement,
    // Used for resizing, we don't want to waste valuable time calculating where all the elements should be every frame! Only when we resize the window.
    pub window_size: PhysicalSize<u32>,
    pub min_inner_size: PhysicalSize<f32>,
    pub screen_size: Rect,
    pub scale_factor: f32,

    pub bombcount_digits: usize,
    pub timer_value: Option<usize>,

    pub redraw: bool,
    pub mouse_in_window: bool,
    // Menus
    pub select_menu: Menu,
    pub custom_menu: Menu,
}

pub struct MainState {
    pub game: Minesweeper,
    pub rendering: Rendering,

    pub last_hovered_tile: Vec2,
    pub selected_tile: Option<usize>,
    pub erasing_flags: bool,
    pub holding_button: bool,

    pub i: usize,
}

impl MainState {
    pub fn new(ctx: &mut Context, width: usize, height: usize, bomb_count: usize) -> MainState {
        let mut game = Minesweeper::new(width, height, bomb_count);
        game.board.fill(TileType::Unopened);
        // Load the sprite sheet and set up the batch renderer for, well, batch rendering
        let spritesheet = Image::from_path(ctx, "/spritesheet.png").unwrap();
        let mut spritesheet_batch = InstanceArray::new(ctx, spritesheet.clone());
        spritesheet_batch.resize(ctx, width * height);

        let game_config_result = MainState::set_window_and_game_specific_elements(ctx, width, height, game.bomb_count);

        ctx.gfx.window().set_visible(true);

        let timer_img     = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), 21, 9, 1);
        let button_img    = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), 19, 19, 1);

        let minefield = game_config_result.0;
        let bombcount = game_config_result.1;
        let timer     = GuiElement::new(timer_img);
        let button    = GuiElement::new(button_img);

        let rendering = Rendering {
            spritesheet, spritesheet_batch,
            minefield, bombcount, timer, button,
            window_size: PhysicalSize::new(0, 0), min_inner_size: game_config_result.3,
            scale_factor: 1.0, screen_size: Rect::zero(),
            timer_value: None, bombcount_digits: game_config_result.2,
            redraw: true,  mouse_in_window: false,
            select_menu: Menu::new(ctx, 68, 34, vec![
                Rect::new(1.0,  1.0, 66.0, 8.0),
                Rect::new(1.0,  9.0, 66.0, 8.0),
                Rect::new(1.0, 17.0, 66.0, 8.0),
                Rect::new(1.0, 26.0, 66.0, 8.0),
            ], vec![]),
            custom_menu: Menu::new(ctx, 68, 50, vec![
                Rect::new(59.0,  0.0,  9.0, 9.0),
                Rect::new(20.0, 38.0, 28.0, 9.0),
                Rect::new(29.0, 10.0, 35.0, 7.0),
                Rect::new(29.0, 19.0, 35.0, 7.0),
                Rect::new(29.0, 28.0, 35.0, 7.0),
            ], vec![
                NumberInput::new(8, 200, 6),
                NumberInput::new(8, 100, 6),
                NumberInput::new(8, 999999, 6),
            ]),
        };

        MainState { game, rendering, 
            last_hovered_tile: Vec2::MAX, selected_tile: None, erasing_flags: false, holding_button: false, i: 0 }
    }

    // Generates GuiElements that change depending on the game, as well as set the window min size
    pub fn set_window_and_game_specific_elements(ctx: &mut Context, width: usize, height: usize, bomb_count: usize) -> (GuiElement, GuiElement, usize, PhysicalSize<f32>) {
        // 9 is the size of one tile, 4 is added to make room for the nice border  
        let minefield_img = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), (width * 9 + 4) as u32, (height * 9 + 4) as u32, 1);
        // For the bomb counter, we want to show the minimum amount of digits possible
        let bombcount_digits = bomb_count.checked_ilog10().unwrap_or(0) + 1;
        let bombcount_img = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), (10*bombcount_digits)+4, 18, 1);
        
        let min_inner_size: PhysicalSize<f32> = PhysicalSize::from_logical(LogicalSize::new(
            (minefield_img.width().max(9*8+4)  + 8) as f32,
            (minefield_img.height().max(9*8+4) + 4 + 24) as f32), 1.0);

        ctx.gfx.window().set_min_inner_size(Some(min_inner_size));

        (GuiElement::new(minefield_img), GuiElement::new(bombcount_img), bombcount_digits as usize, min_inner_size)
    }

    // Changes to a new game of minesweeper, remakes certain rendering elements
    pub fn new_game(&mut self, ctx: &mut Context, width: usize, height: usize, bomb_count: usize) -> GameResult {
        // Make the new game
        self.game = Minesweeper::new(width, height, bomb_count);
        // Reset some variables
        self.selected_tile = None;
        self.rendering.timer_value = None;

        // Reconfigure the window and elements
        let result = MainState::set_window_and_game_specific_elements(ctx, width, height, self.game.bomb_count);
        self.rendering.minefield = result.0;
        self.rendering.bombcount = result.1;
        self.rendering.bombcount_digits = result.2;
        self.rendering.min_inner_size = result.3;
        self.rendering.window_size = PhysicalSize::new(0, 0);

        self.draw_all(ctx)
    }

    // TODO: Make this function a LOT better (includes rework GuiElements somewhat)
    // Called every frame, reposition all of the UI elements if they need to be repositioned.
    pub fn update_window_size(&mut self, window_size: PhysicalSize<u32>) {
        // Return if the new window size is the same as it was last frame
        if window_size == self.rendering.window_size { return; }
        self.rendering.window_size = window_size;
        // Work out the scale factor
        let scale_factor = f32::min(
            window_size.width  as f32 / self.rendering.min_inner_size.width  as f32,
            window_size.height as f32 / self.rendering.min_inner_size.height as f32).floor();
        self.rendering.scale_factor = scale_factor;
        
        let minefield_pos = Vec2::new(window_size.width as f32, window_size.height as f32 + 20.0 * scale_factor) / 2.0 - self.rendering.minefield.middle * scale_factor;
        self.rendering.minefield.goto(minefield_pos, scale_factor);
        
        let mut xs: Vec<f32> = vec![];
        for i in 0..3 {
            let before = (i     as f32/3.0) * window_size.width as f32;
            let after  = ((i+1) as f32/3.0) * window_size.width as f32;
            let mid = (before + after) / 2.0;

            xs.push(mid.clamp(minefield_pos.x, minefield_pos.x + (self.rendering.minefield.middle.x * 2.0 * scale_factor)));
        }

        let y_between_top_and_board_top = self.rendering.minefield.dest_rect.y / 2.0;
        let mut positions: Vec<Vec2> = vec![];
        
        positions.push(Vec2::new(xs[0], y_between_top_and_board_top) - self.rendering.bombcount.middle * scale_factor);
        positions.push(Vec2::new(xs[1], y_between_top_and_board_top) - self.rendering.button.middle    * scale_factor);
        positions.push(Vec2::new(xs[2], y_between_top_and_board_top) - self.rendering.timer.middle     * scale_factor);

        self.rendering.bombcount.goto(positions[0], scale_factor);
        self.rendering.button.goto(positions[1], scale_factor);
        self.rendering.timer.goto(positions[2], scale_factor);
    }

    // TODO: Finish + slightly rework this
    // Renders the minefield
    pub fn draw_minefield(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_image(ctx, self.rendering.minefield.img.clone(), Color::from_rgba(90, 105, 136, 255));
        canvas.set_sampler(Sampler::nearest_clamp());

        // Draw the background
        // TODO: Maybe make the second line (where we get the dimensions) neater
        MainState::draw_nineslice(&mut canvas, &mut self.rendering.spritesheet_batch, Rect::new(36.0, 34.0, 5.0, 5.0), 2.0,
            Rect::new(0.0, 0.0, self.rendering.minefield.img.width() as f32, self.rendering.minefield.img.height() as f32));

        // Draw the tiles
        self.rendering.spritesheet_batch.set(
            self.game.board
            .iter().enumerate().map(|(i, tile)| DrawParam::new().dest(index_to_draw_coord(&self.game, i))
            .dest(index_to_draw_coord(&self.game, i))
            // Draw the tile as either unopened or dug
            // If the tile has been dug, OR if 
            // A) we've lost, B) a bomb is there, AND C) the tile isn't flagged, show an opened tile 
            .src(normalize_rect(Rect::new(
                if *tile == TileType::Dug ||
                (self.game.bombs.contains(&i) && *tile != TileType::Flag && self.game.state == GameState::Lose)
                {18.0} else {0.0},
                0.0, 9.0, 9.0), &self.rendering.spritesheet))
            )
        );
        canvas.draw(&self.rendering.spritesheet_batch, DrawParam::new().dest(Vec2::new(2.0, 2.0)));
        
        // If a tile is being held down, draw it
        if let Some(index) = self.selected_tile {
            if self.holding_button {
                canvas.draw(&self.rendering.spritesheet, DrawParam::new()
                    .src(normalize_rect(Rect::new(9.0, 0.0, 9.0, 9.0), &self.rendering.spritesheet))
                    .dest(index_to_draw_coord(&self.game, index) + Vec2::new(2.0, 2.0))
                );
            }
        }

        // TODO: If we've lost, draw all bombs / explosions
        self.rendering.spritesheet_batch.set(
            self.game.bombs
            .iter().enumerate()
            .map(|(_, &tile_index)| DrawParam::new().dest(index_to_draw_coord(&self.game, tile_index))
            .src(normalize_rect(Rect::new(9.0, 27.0, 9.0, 9.0), &self.rendering.spritesheet)),
            )
        );
        canvas.draw(&self.rendering.spritesheet_batch, DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        // Draw the neighbour count of all opened tiles
        self.rendering.spritesheet_batch.set(
            self.game.neighbour_count
            .iter().enumerate()
            // We only want to draw the little number if the neighbour count isn't zero and the cell has been dug 
            .filter_map(|(i, count)| match *count != 0 && *self.game.board.get(i).unwrap_or(&TileType::Unopened) == TileType::Dug {
                false => None,
                true  => Some(DrawParam::new().dest(index_to_draw_coord(&self.game, i))
                // Mods and division for nice access to all of the sprites :3
                .src(normalize_rect(Rect::new(9.0 * ((count-1)%4) as f32, 9.0 * ((count+3)/4) as f32, 9.0, 9.0), &self.rendering.spritesheet))),
            })
        );
        canvas.draw(&self.rendering.spritesheet_batch, DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        // Draw flags
        self.rendering.spritesheet_batch.set(
            self.game.board
            .iter().enumerate()
            .filter_map(|(i, tile)| match *tile == TileType::Flag {
                false => None,
                true  => Some(
                    DrawParam::new().dest(index_to_draw_coord(&self.game, i))
                    .src(normalize_rect(Rect::new(0.0, 27.0, 9.0, 9.0), &self.rendering.spritesheet))
                ),
            })
        );
        canvas.draw(&self.rendering.spritesheet_batch, DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        canvas.finish(ctx)?;
        Ok(())
    }

    // Renders the button
    pub fn draw_button(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_image(ctx, self.rendering.button.img.clone(), Color::from_rgba(0, 0, 0, 255));
        canvas.set_sampler(Sampler::nearest_clamp());

        // Draw the background
        MainState::draw_nineslice(&mut canvas, &mut self.rendering.spritesheet_batch, Rect::new(36.0, 42.0, 3.0, 3.0), 1.0,
            Rect::new(0.0, 0.0, self.rendering.button.img.width() as f32, self.rendering.button.img.height() as f32));
        
        // canvas.draw(&self.rendering.spritesheet, DrawParam::new()
        //     .src(normalize_rect(Rect::new(36.0, 0.0, 8.0, 14.0), &self.rendering.spritesheet))
        //     .dest(Vec2::new(3.0, 2.0))
        // );

        canvas.finish(ctx)?;
        Ok(())
    }

    // Renders the bomb counter
    pub fn draw_bombcount(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_image(ctx, self.rendering.bombcount.img.clone(), Color::from_rgba(0, 0, 0, 255));
        canvas.set_sampler(Sampler::nearest_clamp());

        // Draw the background
        MainState::draw_nineslice(&mut canvas, &mut self.rendering.spritesheet_batch, Rect::new(36.0, 39.0, 3.0, 3.0), 1.0,
            Rect::new(0.0, 0.0, self.rendering.bombcount.img.width() as f32, self.rendering.bombcount.img.height() as f32));

        // The number we're going to display
        // We COULD check this to make sure it fits within the given digits, however it would be impossible unless you have some weird invalid game.
        let count = self.game.bombs.len().saturating_sub(self.game.board.iter().filter(|&t| *t == TileType::Flag).count());
        // Vector of all the digits, None representing a blank segment
        // TODO: There's probably a better way to do this.. will do some thinking
        let mut digits: Vec<Option<usize>> = count.to_string().chars()
            .map(|c| c.to_digit(10).unwrap() as usize).map(|c| Some(c)).collect();
        let mut padded_digits: Vec<Option<usize>> = vec![None; self.rendering.bombcount_digits.saturating_sub(digits.len())];
        padded_digits.append(&mut digits);

        // Draw all of the numbers
        self.rendering.spritesheet_batch.set(
            padded_digits.iter().enumerate().rev()
            .map(|(i, &v)| DrawParam::new()
                .src(normalize_rect(Rect::new(
                    // If the digit is None, we want the empty segment
                    if v.is_none() {76.0} else {36.0 + ((v.unwrap_or(0)%5)*8) as f32},
                    (v.unwrap_or_default() / 5) as f32 * 14.0,
                    8.0, 14.0), &self.rendering.spritesheet))
                .dest(Vec2::new((i * 10) as f32 + 3.0, 2.0)))
        );
        canvas.draw(&self.rendering.spritesheet_batch, DrawParam::new());

        canvas.finish(ctx)?;
        Ok(())
    }

    // Renders the timer
    pub fn draw_timer(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_image(ctx, self.rendering.timer.img.clone(), Color::from_rgba(0, 0, 0, 255));
        canvas.set_sampler(Sampler::nearest_clamp());

        // Draw the background
        MainState::draw_nineslice(&mut canvas, &mut self.rendering.spritesheet_batch, Rect::new(39.0, 39.0, 3.0, 3.0), 1.0,
            Rect::new(0.0, 0.0, self.rendering.timer.img.width() as f32, self.rendering.timer.img.height() as f32));
            
        // Unwrap the number
        let t = self.rendering.timer_value.unwrap_or_default();
        if t > 99 * 60 + 59 {
            // bigger than 99 minutes!! make you lose the game for the fun of it lol
            // TODO
        }
        // The different numbers of the timer (and how far along they are on the texture)
        const ALONG: [f32; 4] = [2.0, 6.0, 12.0, 16.0];
        let values = [(t / 600) % 10, (t / 60) % 10 ,(t / 10) % 6, t % 10];
        
        // I originally stored a rect for each of the numbers texture coordinates,
        // but since the y value, width, and height are all the same I then changed it to only store the x value
        // I then realised they're all the same width and by extension distance apart (3 pixels),
        // so now I just add (the width of a number sprite * the value of the digit) to where the first number is, which is what 38.0 represents below 

        // Draw all of the numbers
        self.rendering.spritesheet_batch.set(
            values.iter().enumerate()
            .map(|(i, &v)| DrawParam::new()
                // If the tens place is a zero, don't draw it and instead draw the empty segment
                // If the timer isn't a number, just draw a bunch of empty segments
                .src(normalize_rect(Rect::new(
                    38.0 + (3*if (i == 0 && v == 0) || self.rendering.timer_value.is_none() {10} else {v}) as f32,
                    28.0, 3.0, 5.0), &self.rendering.spritesheet))
                .dest(Vec2::new(ALONG[i], 2.0)))
        );
        canvas.draw(&self.rendering.spritesheet_batch, DrawParam::new());
        // Draw the colon
        // I originally had it flashing which used some annoying code AND didn't really look good, so good riddance to bad rubbish i suppose
        let draw_colon = self.rendering.timer_value.is_some();
        canvas.draw(&self.rendering.spritesheet,
            DrawParam::new().src(normalize_rect(Rect::new(if draw_colon {37.0} else {36.0}, 28.0, 1.0, 5.0), &self.rendering.spritesheet)).dest(Vec2::new(10.0, 2.0)));


        canvas.finish(ctx)?;
        Ok(())
    }

    pub fn draw_all(&mut self, ctx: &mut Context) -> GameResult {
        ctx.gfx.begin_frame().unwrap();

        self.draw_minefield(ctx)?;
        self.draw_bombcount(ctx)?;
        self.draw_button(ctx)?;
        self.draw_timer(ctx)?;

        ctx.gfx.end_frame()?;
        Ok(())
    }

    // Draws a nine-slice texture, from a batch image, to a given canvas
    // TODO: Put this in a file called something like 'rendering.rs'.
    pub fn draw_nineslice(canvas: &mut Canvas, batch_img: &mut InstanceArray, src: Rect, s: f32, dest: Rect) {
        // 's' is the width of the slice from the edge.
        // Generate an array of parts to draw.
        // (source rect, destination rect), both of these are localised.
        let middle_size = Vec2::new((dest.w-s*2.0)/(src.w-s*2.0), (dest.h-s*2.0)/(src.h-s*2.0));
        let parts: [(Rect, Rect); 9] = [
            // ===== Middle ===== //
            (Rect::new(s, s, src.w-s*2.0, src.h-s*2.0), Rect::new(s, s, middle_size.x, middle_size.y)),
            // ===== Edges ===== //
            /*Left  */ (Rect::new(0.0,     s, s, src.h-2.0*s), Rect::new(0.0,      s, 1.0, middle_size.y)),
            /*Right */ (Rect::new(src.w-s, s, s, src.h-2.0*s), Rect::new(dest.w-s, s, 1.0, middle_size.y)),
            /*Top   */ (Rect::new(s, 0.0,     src.w-2.0*s, s), Rect::new(s, 0.0,      middle_size.x, 1.0)),
            /*Bottom*/ (Rect::new(s, src.h-s, src.w-2.0*s, s), Rect::new(s, dest.h-s, middle_size.x, 1.0)),
            // ===== Corners ===== //
            /*TL*/ (Rect::new(0.0,     0.0,     s, s), Rect::new(0.0,      0.0,      1.0, 1.0)),
            /*TR*/ (Rect::new(src.w-s, 0.0,     s, s), Rect::new(dest.w-s, 0.0,      1.0, 1.0)),
            /*BL*/ (Rect::new(0.0,     src.h-s, s, s), Rect::new(0.0,      dest.h-s, 1.0, 1.0)),
            /*BR*/ (Rect::new(src.w-s, src.h-s, s, s), Rect::new(dest.w-s, dest.h-s, 1.0, 1.0)),
        ];
        // Draw each of the parts
        let image = &batch_img.image().clone();
        batch_img.set(
            parts.iter().map(|(s, d)| DrawParam::new().src
                (normalize_rect(Rect::new(s.x + src.x,  s.y + src.y,  s.w, s.h), image))
                     .dest_rect(Rect::new(d.x + dest.x, d.y + dest.y, d.w, d.h)))
        );
        canvas.draw(batch_img, DrawParam::new());
    }
}

fn index_to_draw_coord(game: &Minesweeper, index: usize) -> Vec2 {
    Vec2::new(
        ((index % game.width) * 9) as f32,
        ((index / game.width) * 9) as f32,
    )
}

// Fits a rect to a given image
pub fn normalize_rect(rect: Rect, image: &Image) -> Rect {
    Rect::new(
        rect.x / image.width() as f32, rect.y / image.height() as f32,
        rect.w / image.width() as f32, rect.h / image.height() as f32,)
}
