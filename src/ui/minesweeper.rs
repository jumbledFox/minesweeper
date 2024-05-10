// A minesweeper ui element

use indexmap::IndexMap;
use macroquad::{input::MouseButton, math::{vec2, Rect, Vec2}};

use crate::minesweeper::{Difficulty, DifficultyValues, GameState, Minesweeper, SetFlagMode, Tile};

use super::{elements::{aligned_rect, button, Align}, hash_string, renderer::{DrawShape, Renderer}, spritesheet, state::{ButtonState, State}};

pub struct MinesweeperElement {
    game: Minesweeper,
    difficulty: Difficulty,
    timer: Option<f32>,
    flag_mode: Option<SetFlagMode>,

    // Stuff to do with animation
    // TODO: Maybe add tile breaking animation ? :/
    losing_bomb: Option<usize>,
    explosion_radius: f32,
    explosion_bombs: IndexMap<usize, (f32, bool)>,
    explosion_skip: usize,
    // Stores what the last custom input was for the popup
    custom_values: Option<DifficultyValues>,
    requesting_new_game: bool,
}

impl MinesweeperElement {
    pub fn new() -> MinesweeperElement {
        let difficulty = Difficulty::Easy;

        MinesweeperElement {
            game: Minesweeper::new(difficulty),
            difficulty,
            timer: None,
            flag_mode: None,

            losing_bomb: None,
            explosion_radius: 0.0,
            explosion_bombs:  IndexMap::new(),
            explosion_skip: 0,

            custom_values: None,
            requesting_new_game: false,
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }
    pub fn custom_values(&self) -> Option<DifficultyValues> {
        self.custom_values
    }
    pub fn game_in_progress(&self) -> bool {
        self.game.state() == GameState::Playing && self.game.turns() != 0
    }

    pub fn requesting_new_game(&mut self) -> bool {
        // If somethings checking that we're requesting a new game, reset our flag, as they'll deal with it
        match self.requesting_new_game {
            true => { self.requesting_new_game = false; true }
            false => false
        }
    }

    pub fn new_game(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.game = Minesweeper::new(difficulty);
        match difficulty {
            Difficulty::Custom(values) => self.custom_values = Some(values),
            _ => (),
        }
        self.losing_bomb = None;
        self.explosion_bombs.clear();
        self.explosion_radius = 0.0;
        self.explosion_skip = 0;
    }

    // Area is the area where the minefield and the ui bar at the top will be drawn, it's NOT CLIPPED!
    pub fn update(&mut self, area: Rect, state: &mut State, renderer: &mut Renderer) {
        // Update the timer
        self.timer = match (self.game.turns(), self.game.state()) {
            // If we haven't made a move yet, the time should be None
            (0, _) => None,
            // If the game is being played, increment the timer
            (_, GameState::Playing) => Some(self.timer.unwrap_or(0.0) + macroquad::time::get_frame_time()),
            // Otherwise (meaning we've won or lost) keep the timer frozen, on a valid value
            _ => Some(self.timer.unwrap_or(0.0)),
        };

        // The elements along the top
        if button(
            hash_string(&"hello if ur reading this :3".to_owned()),
            Align::Mid(area.x + area.w / 2.0), Align::Beg(area.y + 3.0),
            19.0, 19.0, false, state, renderer
        ) == ButtonState::Released {
            self.requesting_new_game = true;
        }

        let lower_x = area.x + area.w * (1.0 / 6.0);
        let upper_x = area.x + area.w * (5.0 / 6.0);
        self.bomb_counter(Align::Mid(lower_x), Align::Beg(area.y + 4.0), renderer);
        self.timer(       Align::Mid(upper_x), Align::Beg(area.y + 8.0), renderer);

        // Now do the actual minefield
        // Put it in the middle of the area, plus some vertical leeway for the stuff at the top, making sure it's at least at top_height
        self.minefield(Align::Mid(area.x + area.w / 2.0), Align::Mid(area.y + (area.h + self.top_height()-6.0)/2.0), area.y + self.top_height(), state, renderer)
    }

    // explode_bombs_begin() and explode_bombs() are my favourite functions in the whole game
    fn explode_bombs_begin(&mut self, start_index: usize) {
        self.losing_bomb = Some(start_index);
        let index_to_coord = |index: usize| {(
            (index % self.game.width()) as f32,
            (index / self.game.width()) as f32
        )};
        // Calculate all of the bombs distances to the center
        let (center_x, center_y) = index_to_coord(start_index);
        let mut bomb_distances: Vec<(usize, f32)> = Vec::with_capacity(self.game.bomb_count());
        for bomb_index in self.game.bombs() {
            let (x, y) = index_to_coord(*bomb_index);
            // a^2 + b^2 = c^2, thanks Pythagoras
            let squared_distance = (center_x - x).powi(2) + (center_y - y).powi(2);
            bomb_distances.push((*bomb_index, squared_distance));
        }
        // Sort them by distance
        bomb_distances.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less));
        // Add all of the sorted values to our ordered hashmap
        self.explosion_bombs = IndexMap::from_iter(
            bomb_distances.iter()
                .map(|(bomb_index, squared_distance)| (*bomb_index, (*squared_distance, false)))
        );
    }
    fn explode_bombs(&mut self) {
        // If we've exploded all of the bombs, we don't need to do anything more
        if self.explosion_skip == self.explosion_bombs.len() {
            return;
        }
        // TODO: Maybe move at a rate depending on the map size
        // This makes it so the first bomb explodes, then after 0.7 seconds the rest explode
        let multiplier = match self.explosion_radius < 1.0 {
            true  => 1.0 / 0.7,
            false => 20.0,
        };
        self.explosion_radius += macroquad::time::get_frame_time() * multiplier;
        let squared_radius = self.explosion_radius.powi(2);

        // Iterate from the first non-exploded bomb
        let range = self.explosion_bombs.values_mut().skip(self.explosion_skip);
        for (squared_distance, exploded) in range {
            match squared_radius > *squared_distance {
                // If this bomb is inside the circle, increase the skip amount, and explode it! 
                true  => { self.explosion_skip += 1; *exploded = true; },
                // Otherwise, it's outside, meaning all of the ones after this are outisde, so stop!!
                false => { break; },
            };
        }
    }

    pub fn minimum_area_size(&self) -> Vec2 {
        self.minefield_size() + vec2(4.0 + 8.0, 4.0 + 2.0 + self.top_height())
    }
    pub fn top_height(&self) -> f32 {
        26.0
    }
    pub fn minefield_size(&self) -> Vec2 {
        vec2(self.game.width() as f32 * 9.0, self.game.height() as f32 * 9.0)
    }

    fn minefield(&mut self, x: Align, y: Align, min_y: f32, state: &mut State, renderer: &mut Renderer) {
        let size = self.minefield_size();
        let rect = aligned_rect(x, y, size.x, size.y);
        let rect = Rect::new(rect.x, rect.y.max(min_y), rect.w, rect.h);

        if state.hot_item.assign_if_none_and(8008135, rect.contains(state.mouse_pos())) {
            let hovered_tile_coord = ((state.mouse_pos() - rect.point()) / 9.0).floor();
            let selected_tile = (hovered_tile_coord.y as usize * self.game.width() + hovered_tile_coord.x as usize).min(self.game.board().len()-1);
            let selected_diggable = self.game.diggable(selected_tile);

            // Draw the selector thingy
            let selector_pos = rect.point() + tile_pos(selected_tile, self.game.width()) - 1.0;
            renderer.draw(DrawShape::image(selector_pos.x, selector_pos.y, spritesheet::MINEFIELD_SELECTED, None));
            // And a depressed tile if the current tile is diggable and the mouse is down
            if state.mouse_down(MouseButton::Left) && selected_diggable {
                renderer.draw(DrawShape::image(selector_pos.x + 1.0, selector_pos.y + 1.0, spritesheet::minefield_tile(1), None));
            }

            // Digging
            if selected_diggable && state.mouse_released(MouseButton::Left) {
                self.game.dig(selected_tile);
                // If the game's state is now Lose, we dug a mine, so start the explosion flood fill here
                if self.game.state() == GameState::Lose {
                    self.explode_bombs_begin(selected_tile);
                }
            }
            
            // Flagging
            if state.mouse_pressed(MouseButton::Right) {
                self.flag_mode = match self.game.board().get(selected_tile) {
                    Some(t) if *t == Tile::Flag => Some(SetFlagMode::Remove),
                    _ => Some(SetFlagMode::Flag),
                }
            }
            if let Some(flag_mode) = self.flag_mode {
                self.game.set_flag(flag_mode, selected_tile);
            }
            // We only want to set flags once, and remove flags when holding the mouse down.
            if matches!(self.flag_mode, Some(SetFlagMode::Flag)) || state.mouse_released(MouseButton::Right) {
                self.flag_mode = None;
            }
        }
        // Explode bombs
        if self.game.state() == GameState::Lose {
            self.explode_bombs();
        }

        // Draw each tile
        for (i, t) in self.game.board().iter().enumerate() {
            let pos = rect.point() + tile_pos(i, self.game.width());
            // The icon on the tile
            match (t, self.explosion_bombs.get(&i).map(|(_, e)| *e), self.game.state()) {
                (Tile::Numbered(n), None,        _              ) => Some(*n as usize + 3), // Numbered
                (Tile::Flag,        None,        GameState::Lose) => Some(13), // Incorrect flag
                (Tile::Flag,        None,        _              ) => Some(12), // Flag
                (_,                 Some(false), _              ) => Some(14), // Unexploded
                (_,                 Some(true),  _              ) => Some(15), // Exploded
                _ => None,
            }.map(|id| renderer.draw(DrawShape::image(pos.x, pos.y, spritesheet::minefield_tile(id), None)));
            // The tile
            let tile = match t {
                _ if self.losing_bomb == Some(i)           => 3, // The losing dig
                Tile::Dug | Tile::Numbered(_)              => 2, // Dug
                _ if self.explosion_bombs.contains_key(&i) => 2, // An exploding bomb
                _                                          => 0, // Undug tile
            };
            renderer.draw(DrawShape::image(pos.x, pos.y, spritesheet::minefield_tile(tile), None));
        }
        // The border
        let border_rect = Rect::new(rect.x - 2.0, rect.y - 2.0, rect.w + 4.0, rect.h + 4.0);
        renderer.draw(DrawShape::nineslice(border_rect, spritesheet::MINEFIELD_BORDER));
    }

    fn bomb_counter(&self, x: Align, y: Align, renderer: &mut Renderer) {
        let value = self.game.flags_left();
        // Calculate the minimum number of digits needed to display the bomb count (always being 2 or larger for style purposes: 3)
        let digits = ((f32::log10(self.game.bomb_count() as f32) + 1.0).floor() as usize).max(2);

        let size = spritesheet::counter_size(digits);
        let rect = aligned_rect(x, y, size.x, size.y);

        let draw_shapes = (0..digits)
            // Work out the place value of the current digit
            .map(|i| (i, 10usize.saturating_pow(i as u32)))
            .map(|(i, power_of_ten)| (i,
                match value {
                    // If value is none, draw dashes for every character
                    None => spritesheet::CounterDigit::Dash,
                    // Otherwise draw a number! This doesn't draw leading zeros, however always renders the first digit, even if it's a zero!
                    Some(v) if power_of_ten <= v || i == 0 => spritesheet::CounterDigit::Digit((v / power_of_ten) % 10),
                    _ => spritesheet::CounterDigit::Empty,
                }
            ))
            .map(|(i, digit)| (i, spritesheet::counter_digit(digit)))
            // Render the digits in reverse order so they appear the right way around
            .map(|(i, digit_rect)| DrawShape::image(rect.x + 3.0 + (digit_rect.w + 2.0) * (digits - i - 1) as f32, rect.y + 2.0, digit_rect, None))
            // Last but not least draw the background
            .chain(std::iter::once(DrawShape::nineslice(rect, spritesheet::TIMER_BACKGROUND)));
        renderer.draw_iter(draw_shapes);
    }
    
    fn timer(&self, x: Align, y: Align, renderer: &mut Renderer) {
        let size = spritesheet::TIMER_SIZE;
        let rect = aligned_rect(x, y, size.x, size.y);

        let (digits, colon_lit): ([Option<usize>; 4], bool) = if let Some(time) = self.timer {
            let seconds = (time as usize).min(60*100-1).try_into().unwrap_or(usize::MAX);
            let digits = [
                // Don't display the last digit as a 0 
                if seconds < 60*10 { None } else { Some((seconds / 60) / 10) }, // Tens of minutes
                Some((seconds / 60) % 10),                                      // Minutes
                Some((seconds % 60) / 10),                                      // Tens
                Some(seconds % 10),                                             // Units 
            ];
            // Originally, the colon flashed every half-second, but I found it a bit distracting :/
            (digits, true)
        } else {
            ([None; 4], false)
        };

        let draw_shapes = digits.iter()
            .zip([2.0, 6.0, 12.0, 16.0])
            .map(|(&digit, along)| DrawShape::image(rect.x + along, rect.y + 2.0, spritesheet::timer_digit(digit), None))
            // Draw the colon and the background
            .chain(std::iter::once(DrawShape::image(rect.x + 10.0, rect.y + 2.0, spritesheet::timer_colon(colon_lit), None)))
            .chain(std::iter::once(DrawShape::nineslice(rect, spritesheet::TIMER_BACKGROUND)));
        renderer.draw_iter(draw_shapes);
    }
}

fn tile_pos(index: usize, width: usize) -> Vec2 {
    vec2(
        (index%width) as f32 * spritesheet::minefield_tile(0).w,
        (index/width) as f32 * spritesheet::minefield_tile(0).h,
    )
}