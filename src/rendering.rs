use ggez::{glam::Vec2, graphics::{self, Canvas, Color, DrawParam, Image, InstanceArray, Rect}, winit::dpi::LogicalSize, Context, GameResult};
use rand::Rng;

use crate::{gui::{self, button, text_renderer, Gui, TextRenderer}, minesweeper::{self, Minesweeper}};

const SPRITESHEET_IMAGE_BYTES: &[u8] = include_bytes!("../resources/spritesheet.png");
const ICON_IMAGE_BYTES: &[u8] = include_bytes!("../resources/icon.png");

// TODO: Work out colours at runtime from the spritesheet file to make custom skins waaaay easier
// (Also in general organize the spritesheet better!!!)
pub const LIGHT_GRAY: Color = Color { r: 192.0/255.0, g: 203.0/255.0, b: 220.0/255.0, a: 1.0, };
pub const DARK_GRAY:  Color = Color { r: 139.0/255.0, g: 155.0/255.0, b: 180.0/255.0, a: 1.0, };
pub const NAVY:       Color = Color { r:  38.0/255.0, g:  43.0/255.0, b:  68.0/255.0, a: 1.0, };
pub const BLACK:      Color = Color { r:  24.0/255.0, g:  20.0/255.0, b:  37.0/255.0, a: 1.0, };
pub const WHITE:      Color = Color { r: 255.0/255.0, g: 255.0/255.0, b: 255.0/255.0, a: 1.0, };
pub const SHADOW:     Color = Color { r:  24.0/255.0, g:  20.0/255.0, b:  37.0/255.0, a: 200.0/255.0, };

// Variables that change for every new game. This is good as it means we don't have to rebuild the entire renderer each time the game changes.
struct GameSpecifics {
    pub window_size: Vec2,
    pub window_middle: Vec2,
    pub minefield_image: Image,
}
pub struct Rendering {
    tr: TextRenderer,
    scale_factor: f32, // Can only be a whole number but stored as a float to make calculations easier (and probably quicker)
    window_size: Vec2,
    window_middle: Vec2,
    menu_bar_height: f32, // Needed for some calculations, better to keep it rather than ask for it each time we call new_game

    spritesheet_image: Image,
    spritesheet: InstanceArray,
    redraw_minefield: bool,
    minefield_image: Image,
    pub minefield_pos: Vec2,
}

impl Rendering {
    // Rendering needs to know the height of the menu bar to be created, however MenuBar needs a TextRenderer,
    // so we make the TextRenderer first, then make the MenuBar, then make the Renderer, giving it the TextRenderer
    pub fn new_text_renderer(ctx: &mut Context) -> TextRenderer {
        let font_image = Image::from_bytes(ctx, text_renderer::DEFAULT_FONT_BYTES).expect("Unable to load font image from bytes!!");
        TextRenderer::new(ctx, font_image, text_renderer::default_char_map())
    }

    pub fn new(ctx: &mut Context, tr: TextRenderer, board_size: (usize, usize), menu_bar_height: f32) -> Rendering {
        // Make the spritesheet image and batch
        let spritesheet_image = Image::from_bytes(ctx, SPRITESHEET_IMAGE_BYTES).expect("Unable to load spritesheet from bytes!!");
        let mut spritesheet = InstanceArray::new(ctx, spritesheet_image.clone());
        spritesheet.resize(ctx, board_size.0 * board_size.1);

        // Generate game-specific things
        let game_specifics = Rendering::generate_game_specifics(ctx, board_size, menu_bar_height);

        let mut r = Rendering {
            tr, scale_factor: 1.0, window_size: game_specifics.window_size, window_middle: game_specifics.window_middle, menu_bar_height,
            spritesheet_image, spritesheet,
            redraw_minefield: true,
            minefield_image: game_specifics.minefield_image, minefield_pos: Vec2::new(5.0, 24.0+menu_bar_height)
        };

        // TODO: Window icon (right now it's all blurry too!!!)
        // let icon_pixels = Image::from_bytes(ctx, ICON_IMAGE_BYTES).unwrap().to_pixels(ctx).unwrap();
        // let icon = Icon::from_rgba(icon_pixels, 64, 64).unwrap();
        // ctx.gfx.window().set_window_icon(Some(icon));
        
        ctx.gfx.window().set_visible(true);
        r.resize(ctx, 3);
        r
    }

    // If we've made a new game, regenerate the game specifics!
    pub fn new_game(&mut self, ctx: &mut Context, board_size: (usize, usize)) {
        let game_specifics = Rendering::generate_game_specifics(ctx, board_size, self.menu_bar_height);
        self.window_size = game_specifics.window_size;
        self.window_middle = game_specifics.window_middle;
        self.minefield_image = game_specifics.minefield_image;
        self.redraw_minefield();
        // TODO: Work out the new scale factor and resize the window
        let new_scale_factor = self.scale_factor;
        self.resize(ctx, new_scale_factor as usize);
    }

    // Generates parts of 'Rendering' that are specific to a game, such as the board image and the window size
    fn generate_game_specifics(ctx: &mut Context, board_size: (usize, usize), menu_bar_height: f32) -> GameSpecifics {
        // Stores the minefield (plus the nice borders)
        let minefield_image = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), board_size.0 as u32*9+4, board_size.1 as u32*9+4, 1);
        let minefield_padding = (24.0, 5.0, 5.0, 5.0); // Up, down, left, right
        // Work out how big the window should be
        let window_size = Vec2::new(
            minefield_image.width()  as f32 + minefield_padding.2 + minefield_padding.3,
            minefield_image.height() as f32 + minefield_padding.0 + minefield_padding.1 + menu_bar_height
        );
        GameSpecifics { window_size, window_middle: window_size / 2.0, minefield_image }
    }

    // Resizes the window to a multiple
    pub fn resize(&mut self, ctx: &mut Context, new_scale_factor: usize) {
        // TODO: Make sure it can't exceed limit (8192 iirc, but it might be platform dependant)
        self.scale_factor = (new_scale_factor as f32).clamp(1.0, 8.0);
        ctx.gfx.window().set_inner_size(LogicalSize::new(self.window_size.x * self.scale_factor, self.window_size.y * self.scale_factor));
    }

    // Turns an x y coordinate on the window to a scaled down mouse position
    pub fn scaled_mouse_pos(&self, x: f32, y: f32) -> Vec2 {
        Vec2 { x: x / self.scale_factor, y: y / self.scale_factor }
    }
    // Gets the scaled down mouse position
    pub fn mouse_pos(&self, ctx: &mut Context) -> Vec2 {
        let real_mouse_pos = ctx.mouse.position();
        Vec2 { x: real_mouse_pos.x / self.scale_factor, y: real_mouse_pos.y / self.scale_factor }
    }

    // Renders the whole frame
    pub fn render(&mut self, ctx: &mut Context, gui: &Gui, game: &Minesweeper, selected_tile: Option<usize>, selection_depressed: bool) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(192, 203, 220));
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, self.window_size.x, self.window_size.y));
        canvas.set_sampler(graphics::FilterMode::Nearest);

        self.render_game(ctx, &mut canvas, game, selected_tile, selection_depressed);
        self.render_gui(&mut canvas, gui);

        canvas.finish(ctx)
    }

    pub fn render_gui(&mut self, canvas: &mut Canvas, gui: &Gui) {
        // Render the menu bar
        // Draw the background/border of the menu bar (a tad wasteful as only the top, middle, and bottom are showing but meh)
        draw_nineslice(canvas, &mut self.spritesheet, Rect::new(55.0, 36.0, 3.0, 3.0), 1.0, Rect::new(-1.0, 0.0, self.window_size.x+2.0, self.menu_bar_height));

        // Render each button at the top
        for (i, menu_item) in gui.menu_bar.items.iter().enumerate() {
            // Shade the menu item depending on its state and whether or not it's the active one
            let (bg_col, text_col) = match (menu_item.button.b.state, gui.menu_bar.current_item.is_some_and(|c| c==i)) {
                (button::State::Disabled, _) => (LIGHT_GRAY, DARK_GRAY),
                (button::State::Idle, false) => (LIGHT_GRAY, BLACK),
                _ => (NAVY, WHITE)
            };
            canvas.draw(&graphics::Quad, DrawParam::new()
                .dest_rect(menu_item.button.b.rect)
                .color(bg_col));
            self.tr.draw_text(canvas, &menu_item.button.label, DrawParam::new().color(text_col).dest(menu_item.button.text_pos()));
        }

        // Render the dropdown menu, if there is one
        if let Some(current_item) = gui.menu_bar.current_item {
            let dropdown = &gui.menu_bar.items[current_item].dropdown;
            // Draw the shadow of it
            canvas.draw(&graphics::Quad, DrawParam::new().color(SHADOW)
                .dest_rect(Rect::new(dropdown.rect.x + 2.0, dropdown.rect.y + 2.0, dropdown.rect.w, dropdown.rect.h)));
            // Draw the border of it
            draw_nineslice(canvas, &mut self.spritesheet, Rect::new(55.0, 39.0, 3.0, 3.0), 1.0, dropdown.rect);
            // Draw each element of the dropdown menu
            for dropdown_item in dropdown.items.iter() {
                match dropdown_item {
                    gui::dropdown::DropdownItem::Divider(y_pos) => {
                        canvas.draw(&graphics::Quad, DrawParam::new().color(DARK_GRAY)
                            .dest_rect(Rect {x: dropdown.rect.x + 2.0, y: dropdown.rect.y+y_pos+1.0, w: dropdown.rect.w-4.0, h: 1.0 }));
                    }
                    gui::dropdown::DropdownItem::Button(button) => {
                        let (bg_col, text_col) = match button.b.state {
                            button::State::Disabled => (LIGHT_GRAY, DARK_GRAY),
                            button::State::Idle     => (LIGHT_GRAY, BLACK),
                            _                       => (NAVY, WHITE)
                        };
                        canvas.draw(&graphics::Quad, DrawParam::new().color(bg_col).dest_rect(button.b.rect));
                        self.tr.draw_text(canvas, &button.label, DrawParam::new().dest(button.text_pos()).color(text_col));
                    }
                }
            }
        }

    }

    // Draws the minefield, bomb counter, timer, etc
    pub fn render_game(&mut self, ctx: &mut Context, canvas: &mut Canvas, game: &Minesweeper, selected_tile: Option<usize>, selection_depressed: bool) {
        // Render the minefield if we must
        if self.redraw_minefield {
            let _ = self.render_minefield(ctx, game);
            self.redraw_minefield = false;
        }

        // Draw the minefield
        canvas.draw(&self.minefield_image, DrawParam::new().dest(self.minefield_pos));
        // Draw the selected tile and depressed tile if one is being held
        if let Some(selected_tile_index) = selected_tile {
            let selected_tile_pos = index_to_draw_coord(game, selected_tile_index) + self.minefield_pos + 2.0;

            if selection_depressed {
                canvas.draw(&self.spritesheet_image, DrawParam::new().dest(selected_tile_pos)
                    .src(normalize_rect(Rect::new( 9.0,  0.0,  9.0,  9.0), &self.spritesheet_image)));
            }
            canvas.draw(&self.spritesheet_image, DrawParam::new().dest(selected_tile_pos - 1.0)
                .src(normalize_rect(Rect::new(73.0, 28.0, 11.0, 11.0), &self.spritesheet_image)));
        }

    }

    pub fn redraw_minefield(&mut self) {
        self.redraw_minefield = true;
    }
    // Renders the minefield to self.minefield_image, only should be called when the minefield is updated for efficiency
    pub fn render_minefield(&mut self, ctx: &mut Context, game: &Minesweeper) -> GameResult {
        println!("redrew the minefield {:?}", rand::thread_rng().gen_range(10..100));

        let mut canvas = graphics::Canvas::from_image(ctx, self.minefield_image.clone(), LIGHT_GRAY);
        canvas.set_sampler(graphics::FilterMode::Nearest);

        // Draw the border
        draw_nineslice(&mut canvas, &mut self.spritesheet, Rect::new(36.0, 34.0, 5.0, 5.0), 2.0,
            Rect::new(0.0, 0.0, self.minefield_image.width() as f32, self.minefield_image.height() as f32));
        // Draw the tiles
        self.spritesheet.set(
            game.board
            .iter().enumerate().map(|(i, tile)| DrawParam::new().dest(index_to_draw_coord(&game, i))
            .src(normalize_rect(
                Rect::new(if *tile == minesweeper::TileType::Dug {18.0} else {0.0}, 0.0, 9.0, 9.0), &self.spritesheet_image))
            )
        );
        canvas.draw(&self.spritesheet, DrawParam::new().dest(Vec2::new(2.0, 2.0)));

        // Draw the flags
        self.spritesheet.set(
            game.board
            .iter().enumerate().filter_map(|(i, tile)| match *tile == minesweeper::TileType::Flag {
                false => None,
                true => Some(DrawParam::new().dest(index_to_draw_coord(&game, i))
                .src(normalize_rect(Rect::new(0.0, 27.0, 9.0, 9.0), &self.spritesheet_image)))
            } )
        );
        canvas.draw(&self.spritesheet, DrawParam::new().dest(Vec2::new(2.0, 2.0)));
        /*
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
        canvas.draw(&self.rendering.spritesheet_batch, DrawParam::new().dest(Vec2::new(2.0, 2.0))); */

        canvas.finish(ctx)
    }
}


// Draws a nine-slice texture, from a batch image, to a given canvas
pub fn draw_nineslice(canvas: &mut Canvas, batch_img: &mut InstanceArray, src: Rect, slice_size: f32, dest: Rect) {
    let s = slice_size; // Brevity
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
        parts.iter().map(|(s, d)| DrawParam::new()
            .src(normalize_rect(Rect::new(s.x + src.x,  s.y + src.y,  s.w, s.h), image))
            .dest_rect(Rect::new(d.x + dest.x, d.y + dest.y, d.w, d.h)))
    );
    canvas.draw(batch_img, DrawParam::new());
}

// Fits a rect to a given image (another function 'uv_rect' already exists but it doesn't return a rect! although... I could use uv_rect and ints rather than floats)
pub fn normalize_rect(rect: Rect, image: &Image) -> Rect {
    Rect::new(
        rect.x / image.width() as f32, rect.y / image.height() as f32,
        rect.w / image.width() as f32, rect.h / image.height() as f32,)
}

// Turns an index in the game to a draw coordinate
fn index_to_draw_coord(game: &Minesweeper, index: usize) -> Vec2 {
    Vec2::new(
        ((index % game.width) * 9) as f32,
        ((index / game.width) * 9) as f32,
    )
}