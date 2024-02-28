use ggez::winit::dpi::{LogicalSize, PhysicalSize};
use ggez::{graphics, Context, GameResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, InstanceArray, Rect};

use crate::minesweeper::{GameState, Minesweeper, TileType};

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
}

pub struct MainState {
    pub game: Minesweeper,
    pub rendering: Rendering,

    pub selected_cell: Option<usize>,

    pub i: usize,
}

impl MainState {
    pub fn new(ctx: &mut Context, width: usize, height: usize, bomb_count: usize) -> MainState {
        let game = Minesweeper::new(width, height, bomb_count);

        // Load the sprite sheet and set up the batch renderer for, well, batch rendering
        let spritesheet = Image::from_path(ctx, "/spritesheet.png").unwrap();
        let mut spritesheet_batch = InstanceArray::new(ctx, spritesheet.clone());
        spritesheet_batch.resize(ctx, width * height);

        // Generate images for each part of the display
        // Minefield
        // 9 is the size of one tile, 4 is added to make room for the nice border  
        let minefield_img = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), (width * 9 + 4) as u32, (height * 9 + 4) as u32, 1);
        // Bomb counter
        let bombcount_digits = game.bomb_count.checked_ilog10().unwrap_or(0) + 1 + 1;
        let bombcount_img = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), (10*bombcount_digits)+4, 18, 1);
        // Timer
        let timer_img     = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), 21, 9, 1);
        // Button (Don't think this has a proper name...)
        let button_img    = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), 19, 19, 1);

        // The window needs to have room for the board, as well as 4 pixels on either side around it, plus the top part
        let min_inner_size: PhysicalSize<f32> = PhysicalSize::from_logical(LogicalSize::new(
            (minefield_img.width()  + 8) as f32,
            (minefield_img.height() + 4 + 24) as f32), 1.0);
        
        ctx.gfx.window().set_inner_size(min_inner_size);
        ctx.gfx.window().set_min_inner_size(Some(min_inner_size));
        ctx.gfx.window().set_visible(true);

        let minefield = GuiElement::new(minefield_img);
        let bombcount = GuiElement::new(bombcount_img);
        let timer     = GuiElement::new(timer_img);
        let button    = GuiElement::new(button_img);

        let rendering = Rendering {
            spritesheet, spritesheet_batch,
            minefield, bombcount, timer, button,
            window_size: PhysicalSize::new(0, 0), min_inner_size: min_inner_size,
            scale_factor: 1.0, screen_size: Rect::zero(),
            timer_value: None, bombcount_digits: bombcount_digits as usize,
        };

        MainState { game, rendering, selected_cell: Some(width + 10), i: 0 }
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

        // Draw the nine-slice texture
        
        MainState::draw_nineslice(&mut canvas, &mut self.rendering.spritesheet_batch, Rect::new(27.0, 0.0, 5.0, 5.0), 2.0,
            Rect::new(0.0, 0.0, self.rendering.minefield.img.width() as f32, self.rendering.minefield.img.height() as f32));
        // Test nines-lice
        // MainState::draw_nineslice(&mut canvas, &mut self.rendering.spritesheet_batch, Rect::new(36.0, 35.0, 9.0, 9.0), 4.0,
        //     Rect::new(0.0, 0.0, self.rendering.minefield.img.width() as f32, self.rendering.minefield.img.height() as f32));

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

        canvas.draw(&self.rendering.spritesheet, DrawParam::new()
            .src(normalize_rect(Rect::new(36.0, 0.0, 8.0, 14.0), &self.rendering.spritesheet))
            .dest(Vec2::new(3.0, 2.0))
        );

        canvas.finish(ctx)?;
        Ok(())
    }

    // Renders the bomb counter
    pub fn draw_bombcount(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_image(ctx, self.rendering.bombcount.img.clone(), Color::from_rgba(0, 0, 0, 255));
        
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

        // Only draw the timer if it's a number
        let t = self.rendering.timer_value.unwrap_or_default();
        if t > 99 * 60 + 59 {
            // bigger than 99 minutes!! need some kind of easter egg...
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
    pub fn draw_nineslice(canvas: &mut Canvas, batch_img: &mut InstanceArray, src: Rect, border: f32, dest: Rect) {
        let corner_width  = border;
        let corner_height = border;
        let edge_width  = src.w - border * 2.0;
        let edge_height = border;
        
        // Element 0 is the source rect, Element 1 is the destination rect
        let parts: [(Rect, Rect); 9] = [
            // Top left
            (Rect::new(src.x, src.y, border, border), Rect::new(dest.x, dest.y, 1.0, 1.0)); 9
        ];
        // Draw each of the parts
        let image = &batch_img.image().clone();
        batch_img.set(
            parts.iter().map(|(src, dest)| DrawParam::new().src(normalize_rect(*src, image)).dest_rect(*dest))
        );
        canvas.draw(batch_img, DrawParam::new());

        // return;
        // // Top left
        // canvas.draw(&self.rendering.spritesheet, DrawParam::new()
        //     .src(normalize_rect(Rect::new(src.x, src.y, border, border), &self.rendering.spritesheet))
        //     .dest_rect(Rect::new(dest.x, dest.y, 1.0, 1.0))
        // );
        // // Top right
        // canvas.draw(&self.rendering.spritesheet, DrawParam::new()
        //     .src(normalize_rect(Rect::new(src.x + src.w - border, src.y, border, border), &self.rendering.spritesheet))
        //     .dest_rect(Rect::new(dest.x + dest.w - border, dest.y, 1.0, 1.0))
        // );
        // // Top
        // let src_r = Rect::new(src.x + border, src.y, edge_width, border);
        // let dest_r = Rect::new(dest.x + border, dest.y, dest.w - border * 2.0, 1.0);
        // canvas.draw(&self.rendering.spritesheet, DrawParam::new()
        //     .src(normalize_rect(src_r, &self.rendering.spritesheet))
        //     .dest_rect(dest_r)
        // );
        // println!("{:?}", (src, src_r, dest_r))
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