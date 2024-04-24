use macroquad::{audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound}, input::MouseButton, math::{vec2, Rect, Vec2}};

use crate::{minesweeper::{Difficulty, GameState, Minesweeper, Tile}, ui::DrawShape};

use super::{hash_string, spritesheet, ButtonState, RectFeatures, UIState};

const EXPLOSION_SOUND_BYTES: &[u8] = include_bytes!("../../resources/explosion.ogg");

pub struct MinesweeperUI {
    game: Minesweeper,
    selected_cell: Option<usize>,
    erasing_flags: bool,

    timer: Option<f32>,
    explosion_sound: Option<Sound>, 
    exploded_bombs: Vec<usize>,
    next_explosion: f64,
}

impl MinesweeperUI {
    pub async fn new(difficulty: Difficulty) -> MinesweeperUI {
        let explosion_sound = match load_sound_from_bytes(EXPLOSION_SOUND_BYTES).await {
            Ok(s) => Some(s),
            Err(e) => { println!("Error {:?} loading explosion sound!!11!", e); None }
        };

        MinesweeperUI {
            game: Minesweeper::new(difficulty),
            selected_cell: None,
            erasing_flags: false,

            timer: None,
            explosion_sound,
            exploded_bombs: vec![],
            next_explosion: 0.0,
        }
    }

    pub fn new_game(&mut self, difficulty: Difficulty) {
        self.game = Minesweeper::new(difficulty);
        self.exploded_bombs.clear();
        self.selected_cell = None;
    }

    // Renders the bomb counter, timer, and button. Returns whether the button was released
    pub fn game_ui(&mut self, ui: &mut UIState, y: f32) -> bool {
        let screen_size = ui.screen_size();
        let lower_x = screen_size.x / 5.0;
        let y = y + self.game_ui_height() / 2.0;

        // Make sure the button is displayed on top of the others, just in case!
        let button_released = self.button(ui, screen_size.x / 2.0, y);
        self.bomb_counter(ui, lower_x, y);
        self.timer(ui, screen_size.x - lower_x, y);

        if button_released {
            // TODO: ? self.new_game(self.game.difficulty)
        }
        button_released
    }

    // The button with the little face on it :3
    fn button(&mut self, ui: &mut UIState, x: f32, y: f32) -> bool {
        let rect = Rect::centered(x, y, 19.0, 19.0).round();
        let hovered = ui.mouse_in_rect(rect);
        
        let state = ui.button_state(hash_string(&" - MINESWEEPER BUTTON!! - ".to_owned()), hovered, true);

        let (source, offset) = match state {
            ButtonState::Clicked | ButtonState::Held => (spritesheet::BUTTON_DOWN, 1.0),
            _ => (spritesheet::BUTTON_IDLE, 0.0),
        };

        ui.draw_queue().push(DrawShape::nineslice(rect.offset(Vec2::splat(offset)), source));
        state == ButtonState::Released
    }
    
    fn bomb_counter(&mut self, ui: &mut UIState, x: f32, y: f32) {
        let value = self.game.flags_left();
        // Calculate the minimum number of digits needed to display the bomb count (always being 2 or larger for style purposes: 3)
        let digits = ((f32::log10(self.game.bomb_count() as f32) + 1.0).floor() as usize).max(2);
        
        let size = spritesheet::counter_size(digits);
        let rect = Rect::centered(x, y, size.x, size.y).round();

        // TODO: Display dashes if value is None
        let value = value.unwrap();
        let draw_shapes = (0..digits)
            // Work out the place value of the current digit
            .map(|i| (i, 10usize.saturating_pow(i as u32)))
            .map(|(i, power_of_ten)| (i,
                // Don't draw leading zeros, however always render the first digit, even if it's a zero!
                match power_of_ten <= value || i == 0 {
                    true  => Some((value / power_of_ten) % 10),
                    false => None,
                }
            ))
            .map(|(i, digit)| (i, spritesheet::counter_digit(digit)))
            // Render the digits in reverse order so they appear the right way around
            .map(|(i, digit_rect)| DrawShape::image(rect.x + 3.0 + (digit_rect.w + 2.0) * (digits - i - 1) as f32, rect.y + 2.0, digit_rect));

        ui.draw_queue().extend(draw_shapes);
        ui.draw_queue().push(DrawShape::nineslice(rect, spritesheet::TIMER_BACKGROUND));
    }

    fn timer(&mut self, ui: &mut UIState, x: f32, y: f32) {
        let size = spritesheet::TIMER_SIZE;
        let rect = Rect::centered(x, y, size.x, size.y).round();

        let (digits, colon_lit): ([Option<usize>; 4], bool) = if let Some(time) = self.timer {
            // I could do 'into.unwrap()', but like staying safe :3
            let seconds = (time as usize).min(60*100-1).try_into().unwrap_or(usize::MAX);
            (
                [
                    // Don't display the last digit as a 0 
                    if seconds < 60*10 { None } else { Some((seconds / 60) / 10) }, // Tens of minutes
                    Some((seconds / 60) % 10),                                      // Minutes
                    Some((seconds % 60) / 10),                                      // Tens
                    Some(seconds % 10),                                             // Units 
                ],
                // Originally, the colon flashed, but I found it a bit distracting :/
                true,
            )
        } else {
            ([None; 4], false)
        };
        
        let draw_shapes = digits.iter()
            .zip([2.0, 6.0, 12.0, 16.0])
            .map(|(&digit, along)| DrawShape::image(rect.x + along, rect.y + 2.0, spritesheet::timer_digit(digit)))
            .chain(std::iter::once(DrawShape::image(rect.x + 10.0, rect.y + 2.0, spritesheet::timer_colon(colon_lit))));

        ui.draw_queue().extend(draw_shapes);
        ui.draw_queue().push(DrawShape::nineslice(rect, spritesheet::TIMER_BACKGROUND));
    }

    pub fn game_ui_height(&self) -> f32 {
        24.0
    }

    // Renders the minefield ui element
    // TODO: Make this a bit neater...
    // TODO: Panning with middle mouse maybe?? For when the scale is too large to fit the game
    pub fn minefield(&mut self, ui: &mut UIState, middle_x: f32, y: f32, min_y: f32) {

        // Update the timer
        match (self.game.turns(), self.game.state()) {
            // If we haven't made a turn yet, the timer should be None
            (0, _)                  => self.timer = None,
            // If we have made a turn, and we're playing, increment the timer
            (_, GameState::Playing) => self.timer = Some(self.timer.unwrap_or(0.0) + macroquad::time::get_frame_time()),
            // Otherwise do nothing, keeping it frozen
            _ => (),
        };

        // TODO: Make bombs explode
        if self.game.state() == GameState::Lose
        && self.exploded_bombs.len() < self.game.bombs().len()
        && macroquad::time::get_time() >= self.next_explosion {
            let mut next_bomb = self.game.bombs().iter().filter(|b| !self.exploded_bombs.contains(b));
            self.explode_bomb(*next_bomb.next().unwrap());
        }


        let size = vec2((self.game.width()*9) as f32, (self.game.height()*9) as f32);
        let pos = vec2(middle_x - size.x/2.0, min_y.max(y - size.y/2.0) + 2.0);

        let rect = Rect::new(pos.x, pos.y, size.x, size.y);
        let id = hash_string(&String::from("MINEFIELD!!! jumbledfox is so cool 🦊🦊"));
        let mouse_in_rect = ui.mouse_in_rect(rect);
        let state = ui.button_state(id, mouse_in_rect, true);

        // TODO: When resizing the window, where the mouse last was means you can select a tile
        if mouse_in_rect && state != ButtonState::Idle {
            let selected_cell_pos = ((ui.mouse_pos - rect.point()) / 9.0).floor();
            // According to logic, this shouldn't need the min, but I like things to always be safe, just in case!
            let selected_cell = (selected_cell_pos.x as usize + selected_cell_pos.y as usize * self.game.width())
                .min(self.game.board().len().saturating_sub(1));
            self.selected_cell = Some(selected_cell);

            // Draw the selection outline 
            ui.draw_queue().push(DrawShape::image(
                selected_cell_pos.x * 9.0 + rect.x - 1.0,
                selected_cell_pos.y * 9.0 + rect.y - 1.0,
                spritesheet::MINEFIELD_SELECTED
            ));
            
            // I might as well use the button state to check if you're about to / trying to dig, rather than `ui.mouse_pressed()` :> 
            if state == ButtonState::Held && self.game.diggable(selected_cell) {
                ui.draw_queue().push(DrawShape::image(
                    rect.x + (selected_cell%self.game.width()) as f32 * 9.0,
                    rect.y + (selected_cell/self.game.width()) as f32 * 9.0,
                    spritesheet::minefield_tile(1)
                ));
            }

            // Digging
            if state == ButtonState::Released {
                let previously_lose = self.game.state() == GameState::Lose;
                self.game.dig(selected_cell);
                // If we've JUST lost (we weren't before this dig) then explode the initial bomb and start the chain reaction of explosions!
                if self.game.state() == GameState::Lose && !previously_lose {
                    self.explode_bomb(selected_cell);
                }
            }

            // Flagging
            if ui.mouse_pressed(MouseButton::Right) {
                self.erasing_flags = self.game.board().get(selected_cell).is_some_and(|t| *t == Tile::Flag);
            }
            if ui.mouse_pressed(MouseButton::Right) || (ui.mouse_down(MouseButton::Right) && self.erasing_flags) {
                self.game.set_flag(self.erasing_flags, selected_cell);
            }

        } else {
            self.selected_cell = None;
        }

        // Draw the tiles
        // TODO: Make some kind of DrawShape::Minefield, as this shit is probably inefficient as hell

        // Draw bombs if we've lots
        if self.game.state() == GameState::Lose {
            for i in self.game.bombs().iter() {
                let sprite = if self.exploded_bombs.contains(i) { 15 } else { 14 };
                ui.draw_queue().push(DrawShape::image(
                    rect.x + (i%self.game.width()) as f32 * 9.0,
                    rect.y + (i/self.game.width()) as f32 * 9.0,
                    spritesheet::minefield_tile(sprite)
                ));
            }
        }

        for (i, tile) in self.game.board().iter().enumerate() {
            let t = match *tile {
                Tile::Unopened => 0,
                Tile::Dug => 2,
                Tile::Flag => {
                    ui.draw_queue().push(DrawShape::image(
                        rect.x + (i%self.game.width()) as f32 * 9.0,
                        rect.y + (i/self.game.width()) as f32 * 9.0,
                        spritesheet::minefield_tile(12)
                    ));
                    0
                },
                Tile::Numbered(n) => {
                    ui.draw_queue().push(DrawShape::image(
                        rect.x + (i%self.game.width()) as f32 * 9.0,
                        rect.y + (i/self.game.width()) as f32 * 9.0,
                        spritesheet::minefield_tile((3+n).into())
                    ));
                    2
                },
            };
            ui.draw_queue().push(DrawShape::image(
                rect.x + (i%self.game.width()) as f32 * 9.0,
                rect.y + (i/self.game.width()) as f32 * 9.0,
                spritesheet::minefield_tile(t)
            ));
        }

        // Draw the border
        ui.draw_queue().push(DrawShape::nineslice(Rect::new(pos.x - 2.0, pos.y - 2.0, size.x + 4.0, size.y + 4.0), spritesheet::MINEFIELD_BORDER));
    }

    // TODO: Explode bombs in a circular pattern (flood fill?!)
    fn explode_bomb(&mut self, index: usize) {
        if self.exploded_bombs.contains(&index)
        || !self.game.bombs().contains(&index) {
            return;
        }
        self.next_explosion = macroquad::time::get_time() + (macroquad::rand::gen_range(3.0, 5.0) / self.game.bomb_count() as f64).max(0.15);
        if let Some(explosion_sound) = &self.explosion_sound {
            play_sound(explosion_sound, PlaySoundParams {
                volume: 0.2,
                looped: false,
            });
        }
        self.exploded_bombs.push(index);
    }
}