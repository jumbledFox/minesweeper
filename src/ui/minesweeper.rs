// // A minesweeper ui element

// use std::collections::HashSet;

// use indexmap::IndexMap;
// use macroquad::{audio::{play_sound, PlaySoundParams, Sound}, camera::{set_camera, Camera2D}, color::WHITE, input::MouseButton, math::{vec2, Rect, Vec2}, rand::gen_range, texture::{draw_texture_ex, render_target, DrawTextureParams, RenderTarget}};

// use crate::minesweeper::{get_index_from_offset, Difficulty, DifficultyValues, GameState, Minesweeper, SetFlagMode, Tile, NEIGHBOUR_OFFSETS};

// use super::{elements::{aligned_rect, Align}, renderer::{DrawShape, Renderer}, spritesheet::{self, FoxFace}, state::{ButtonState, State}};

// pub struct MinesweeperElement {
//     game: Minesweeper,
//     difficulty: Difficulty,
//     timer: Option<f32>,
//     flag_mode: Option<SetFlagMode>,
//     chording: bool,
//     chorded: bool,
//     prev_state: GameState,

//     minefield: RenderTarget,
//     // TODO: Flag sound
//     // TODO: Popup textbox fix
//     // TODO: Dark mode
//     // Stuff to do with animation
//     // Face
//     holding: bool,
//     fox_blink_timer: f32,
//     fox_blink_next: f32,

//     // Explosion
//     losing_bomb: Option<usize>,
//     explosion_radius: f32,
//     explosion_radius_grow_rate: f32,
//     explosion_bombs: IndexMap<usize, (f32, bool)>,
//     explosion_skip: usize,
    
//     explosion_effect_timeout: f32,
//     explosion_sound: Option<Sound>,
//     win_sound: Option<Sound>,
//     // Stores what the last custom input was for the popup
//     custom_values: Option<DifficultyValues>,
//     requesting_new_game: bool,
// }

// impl MinesweeperElement {
//     pub async fn new() -> MinesweeperElement {
//         let difficulty = Difficulty::Easy;
//         let explosion_sound = macroquad::audio::load_sound_from_bytes(include_bytes!("../../resources/explosion.ogg")).await.ok();
//         let win_sound       = macroquad::audio::load_sound_from_bytes(include_bytes!("../../resources/congrats.ogg")) .await.ok();

//         let minefield =render_target(difficulty.values().width() as u32 * 9, difficulty.values().height() as u32 * 9);
//         minefield.texture.set_filter(macroquad::texture::FilterMode::Nearest);

//         MinesweeperElement {
//             game: Minesweeper::new(difficulty),
//             difficulty,
//             timer: None,
//             flag_mode: None,
//             chording: false,
//             chorded: false,
//             prev_state: GameState::Playing,
//             minefield,
            
//             holding: false,
//             fox_blink_timer: 0.0,
//             fox_blink_next: 0.0,

//             losing_bomb: None,
//             explosion_radius: 0.0,
//             explosion_radius_grow_rate: 0.0,
//             explosion_bombs:  IndexMap::new(),
//             explosion_skip: 0,
//             explosion_effect_timeout: f32::MAX,
//             explosion_sound,
//             win_sound,
            
//             custom_values: None,
//             requesting_new_game: false,
//         }
//     }

//     pub fn difficulty(&self) -> Difficulty {
//         self.difficulty
//     }
//     pub fn custom_values(&self) -> Option<DifficultyValues> {
//         self.custom_values
//     }
//     pub fn game_in_progress(&self) -> bool {
//         self.game.state() == GameState::Playing && self.game.turns() != 0
//     }

//     pub fn requesting_new_game(&mut self) -> bool {
//         // If somethings checking that we're requesting a new game, reset our flag, as they'll deal with it
//         match self.requesting_new_game {
//             true => { self.requesting_new_game = false; true }
//             false => false
//         }
//     }

//     pub fn new_game(&mut self, difficulty: Difficulty, renderer: &mut Renderer) {
//         renderer.shake_stop();
//         self.difficulty = difficulty;
//         self.game = Minesweeper::new(difficulty);
//         match difficulty {
//             Difficulty::Custom(values) => self.custom_values = Some(values),
//             _ => (),
//         }
//         self.minefield = render_target(difficulty.values().width() as u32 * 9, difficulty.values().height() as u32 * 9);
//         self.minefield.texture.set_filter(macroquad::texture::FilterMode::Nearest);

//         self.losing_bomb = None;
//         self.explosion_bombs.clear();
//         self.explosion_radius = 0.0;
//         self.explosion_skip = 0;
//         self.explosion_effect_timeout = f32::MAX;
//     }

//     // Area is the area where the minefield and the ui bar at the top will be drawn, it's NOT CLIPPED!
//     pub fn update(&mut self, area: Rect, state: &mut State, renderer: &mut Renderer) {
//         // Update the timer
//         self.timer = match (self.game.turns(), self.game.state()) {
//             // If we haven't made a move yet, the time should be None
//             (0, _) => None,
//             // If the game is being played, increment the timer
//             (_, GameState::Playing) => Some(self.timer.unwrap_or(0.0) + macroquad::time::get_frame_time()),
//             // Otherwise (meaning we've won or lost) keep the timer frozen, on a valid value
//             _ => Some(self.timer.unwrap_or(0.0)),
//         };

//         // The elements along the top
//         self.button(Align::Mid(area.x + area.w / 2.0), Align::Beg(area.y + 3.0), state, renderer);

//         let lower_x = area.x + area.w * (1.0 / 6.0);
//         let upper_x = area.x + area.w * (5.0 / 6.0);
//         self.bomb_counter(Align::Mid(lower_x), Align::Beg(area.y + 4.0), renderer);
//         self.timer(       Align::Mid(upper_x), Align::Beg(area.y + 8.0), renderer);

//         // Now do the actual minefield
//         // Put it in the middle of the area, plus some vertical leeway for the stuff at the top, making sure it's at least at top_height
//         self.minefield(Align::Mid(area.x + area.w / 2.0), Align::Mid(area.y + (area.h + self.top_height()-6.0)/2.0), area.y + self.top_height(), state, renderer)
//     }

//     // explode_bombs_begin() and explode_bombs() are my favourite functions in the whole game
//     fn explode_bombs_begin(&mut self, start_index: usize) {
//         let index_to_coord = |index: usize| {(
//             (index % self.game.width()) as f32,
//             (index / self.game.width()) as f32
//         )};
//         // Calculate all of the bombs distances to the center
//         let (center_x, center_y) = index_to_coord(start_index);
//         let mut bomb_distances: Vec<(usize, f32)> = Vec::with_capacity(self.game.bomb_count());
        
//         for bomb_index in self.game.bombs() {
//             let (x, y) = index_to_coord(*bomb_index);
//             // a^2 + b^2 = c^2, thanks Pythagoras
//             let squared_distance = (center_x - x).powi(2) + (center_y - y).powi(2);
//             bomb_distances.push((*bomb_index, squared_distance));
//         }
//         // Sort them by distance
//         bomb_distances.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less));
//         // Add all of the sorted values to our ordered hashmap
//         self.explosion_bombs = IndexMap::from_iter(
//             bomb_distances
//                 .iter()
//                 .map(|(bomb_index, squared_distance)| (*bomb_index, (*squared_distance, false)))
//         );
//         self.losing_bomb = Some(*self.explosion_bombs.keys().next().unwrap_or(&0));
//         // Expand at a rate depending on map size, this isn't the best equation but it works nicely with what I've tested
//         self.explosion_radius_grow_rate = (usize::max(self.game.width(), self.game.height()) as f32).sqrt() * 2.0;
//     }
//     fn explode_bombs(&mut self, renderer: &mut Renderer) {
//         // Don't do anything if explode_bombs_begin has been called for some reason.
//         if self.losing_bomb.is_none() {
//             return;
//         }
//         // If we've exploded all of the bombs, we don't need to do anything more
//         if self.explosion_skip == self.explosion_bombs.len() {
//             return;
//         }
//         // This makes it so the first bomb explodes, then after 0.7 seconds the rest explode
//         let multiplier = match self.explosion_radius < 1.0 {
//             true  => 1.0 / 0.7,
//             false => self.explosion_radius_grow_rate,
//         };
//         self.explosion_radius += macroquad::time::get_frame_time() * multiplier;
//         let squared_radius = self.explosion_radius.powi(2);

//         let prev_skip = self.explosion_skip;
//         // Iterate from the first non-exploded bomb
//         let range = self.explosion_bombs.values_mut().skip(self.explosion_skip);
//         for (squared_distance, exploded) in range {
//             match squared_radius > *squared_distance {
//                 // If this bomb is inside the circle, increase the skip amount, and explode it! 
//                 true  => { self.explosion_skip += 1; *exploded = true; },
//                 // Otherwise, it's outside, meaning all of the ones after this are outisde, so stop!!
//                 false => { break; },
//             };
//         }
//         // If we've exploded at least one bomb, play the explosion sound!
//         self.explosion_effect_timeout += macroquad::time::get_frame_time();
//         if prev_skip != self.explosion_skip && self.explosion_effect_timeout > 0.1 {
//             self.explosion_effect_timeout = 0.0;
//             if let Some(explosion_sound) = &self.explosion_sound {
//                 play_sound(&explosion_sound, PlaySoundParams {
//                     looped: false,
//                     volume: 0.3,
//                 });
//                 renderer.shake(1.0);
//             }
//         }
//     }

//     pub fn minimum_area_size(&self) -> Vec2 {
//         self.minefield_size() + vec2(4.0 + 8.0, 4.0 + 2.0 + self.top_height())
//     }
//     pub fn top_height(&self) -> f32 {
//         26.0
//     }
//     pub fn minefield_size(&self) -> Vec2 {
//         vec2(self.game.width() as f32 * 9.0, self.game.height() as f32 * 9.0)
//     }

//     pub fn won_this_frame(&self) -> bool {
//         self.prev_state == GameState::Playing && self.game.state() == GameState::Win
//     }

//     pub fn win_sound(&self) {
//         if let Some(win_sound) = &self.win_sound {
//             play_sound(&win_sound, PlaySoundParams {
//                 looped: false,
//                 volume: 0.5,
//             })
//         }
//     }

//     fn minefield(&mut self, x: Align, y: Align, min_y: f32, state: &mut State, renderer: &mut Renderer) {
//         let width = self.game.width();
//         let index_to_coord = |index: usize| {(
//             (index%width) as f32 * spritesheet::minefield_tile(0).w,
//             (index/width) as f32 * spritesheet::minefield_tile(0).h,
//         )};

//         let size = self.minefield_size();
//         let rect = aligned_rect(x, y, size.x, size.y);
//         let rect = Rect::new(rect.x, rect.y.max(min_y), rect.w, rect.h);

//         self.prev_state = self.game.state();
//         self.holding = false;
//         let mut chorded_tiles: HashSet<usize> = HashSet::with_capacity(9);

//         if state.hot_item.assign_if_none_and(8008135, rect.contains(state.mouse_pos())) {
//             let hovered_tile_coord = ((state.mouse_pos() - rect.point()) / 9.0).floor();
//             let selected_tile = (hovered_tile_coord.y as usize * self.game.width() + hovered_tile_coord.x as usize).min(self.game.board().len()-1);

//             // Chording
//             // If we were chording and now we've released a mouse button, chord! And also set the 'chorded' flag to true
//             let bomb_index = match self.chording && (state.mouse_released(MouseButton::Middle) || state.mouse_released(MouseButton::Left) || state.mouse_released(MouseButton::Right)) {
//                 true  => {self.chorded = true; self.game.chord(selected_tile)},
//                 false => None,
//             };
//             // Chording should only be true if right and left are held, or if middle is held
//             self.chording = (state.mouse_down(MouseButton::Left) && state.mouse_down(MouseButton::Right)) || state.mouse_down(MouseButton::Middle);
            
//             self.holding = self.chording || state.mouse_down(MouseButton::Left);

//             // Draw the chorded tiles
//             if self.chording && self.game.state() == GameState::Playing {
//                 chorded_tiles.extend(NEIGHBOUR_OFFSETS
//                     .iter()
//                     .chain(std::iter::once(&(0, 0)))
//                     .flat_map(|(x, y)| get_index_from_offset(selected_tile, *x, *y, self.game.width(), self.game.height()))
//                     .filter(|i| self.game.board().get(*i).is_some_and(|t| *t == Tile::Unopened))
//                 );
//             }

//             // Draw the selector thingy
//             let (sx, sy) = index_to_coord(selected_tile);
//             let selector_pos = rect.point() + vec2(sx, sy) - 1.0;
//             renderer.draw(DrawShape::image(selector_pos.x, selector_pos.y, spritesheet::MINEFIELD_SELECTED, None));

//             // Only dig and flag if we're not chording
//             if !self.chording && !self.chorded {
//                 // Digging
//                 if state.mouse_released(MouseButton::Left) {
//                     self.game.dig(selected_tile);
//                 }
//                 // Draw a depressed tile if the current tile is diggable and the mouse is held down
//                 if state.mouse_down(MouseButton::Left) && self.game.diggable(selected_tile) {
//                     renderer.draw(DrawShape::image(selector_pos.x + 1.0, selector_pos.y + 1.0, spritesheet::minefield_tile(1), None));
//                 }
                
//                 // Flagging
//                 if state.mouse_pressed(MouseButton::Right) {
//                     self.flag_mode = match self.game.board().get(selected_tile) {
//                         Some(t) if *t == Tile::Flag => Some(SetFlagMode::Remove),
//                         _ => Some(SetFlagMode::Flag),
//                     }
//                 }
//                 if let Some(flag_mode) = self.flag_mode {
//                     self.game.set_flag(flag_mode, selected_tile);
//                 }
//                 // We only want to set flags once, and remove flags when holding the mouse down.
//                 if matches!(self.flag_mode, Some(SetFlagMode::Flag)) || state.mouse_released(MouseButton::Right) {
//                     self.flag_mode = None;
//                 }
//             } else {
//                 // We can only remove flags when not chording too!
//                 self.flag_mode = None;
//             }

//             // If the game has been lost this frame start exploding bombs at the bomb that caused it
//             if self.game.state() == GameState::Lose && self.prev_state == GameState::Playing {
//                 self.explode_bombs_begin(bomb_index.unwrap_or(selected_tile))
//             }

//             // Chorded should only be set to false when none of the mouse buttons are pressed
//             // This is done at the end of the selected tile part to make sure we don't dig on the same frame we release from chording
//             self.chorded = self.chorded && (state.mouse_down(MouseButton::Left) || state.mouse_down(MouseButton::Right) || state.mouse_down(MouseButton::Middle));
//         }
//         // Explode bombs
//         if self.game.state() == GameState::Lose {
//             self.explode_bombs(renderer);
//         }

//         // Draw the minefield to a texture rather than a million billion trillion renderer.draw(DrawShape)s
//         set_camera(&Camera2D {
//             zoom: 2.0 / self.minefield.texture.size(),
//             target: self.minefield.texture.size() / 2.0,
//             render_target: Some(self.minefield.clone()),
//             ..Default::default()
//         });
//         let texture = &renderer.texture();
//         let draw_tile = |index: usize, id: u32| {
//             let pos = index_to_coord(index);
//             draw_texture_ex(texture, pos.0, pos.1, WHITE, DrawTextureParams {
//                 source: Some(spritesheet::minefield_tile(id)),
//                 ..Default::default()
//             })
//         };
//         for (i, t) in self.game.board().iter().enumerate() {
//             // Draw the base of the tile
//             draw_tile(i, match t {
//                 _ if self.losing_bomb == Some(i)                                         => 4, // The losing tile
//                 Tile::Dug | Tile::Numbered(_)                                            => 1, // A dug tile
//                 Tile::Flag if !self.explosion_bombs.get(&i).is_some_and(|(_, e)| *e)     => 0, // A flag if there's not an exploded bomb below it 
//                 _ if chorded_tiles.contains(&i) || self.explosion_bombs.contains_key(&i) => 1, // A bomb is here or it's being chorded
//                 _                                                                        => 0, // Unopened
//             });
//             // Draw the icon on top of the tile
//             match (t, self.explosion_bombs.get(&i).map(|(_, e)| *e)) {
//                 (Tile::Flag, None) if self.game.state() == GameState::Lose => Some(6), // Incorrect flag
//                 (Tile::Flag, Some(true)) => Some(7), // Exploded flag
//                 (Tile::Flag, _)          => Some(5), // Flag
//                 (_, Some(false))         => Some(2), // Unexploded bomb
//                 (_, Some(true) )         => Some(3), // Exploded bomb
//                 (Tile::Numbered(n), _)   => Some(*n as u32+7), // Number
//                 _ => None
//             }.map(|id| draw_tile(i, id));
//         }

//         // Draw the texture and the border with the renderer
//         renderer.draw(DrawShape::texture(rect.x, rect.y, self.minefield.texture.clone()));
//         let border_rect = Rect::new(rect.x - 2.0, rect.y - 2.0, rect.w + 4.0, rect.h + 4.0);
//         renderer.draw(DrawShape::nineslice(border_rect, spritesheet::MINEFIELD_BORDER));
//     }

//     // i KNOW some of this is basically just copied from element.rs.. but it's different enough to warrant this and i really don't wanna faff about
//     fn button(&mut self, x: Align, y: Align, state: &mut State, renderer: &mut Renderer) {
//         let rect = aligned_rect(x, y, 19.0, 19.0);
//         let button_state = state.button_state(0xB00B135, state.mouse_in_rect(rect), false, true);

//         let (offset, source) = match button_state {
//             ButtonState::Disabled                    => (0.0, spritesheet::BUTTON_DISABLED),
//             ButtonState::Held | ButtonState::Clicked => (1.0, spritesheet::BUTTON_DOWN),
//             _                                        => (0.0, spritesheet::BUTTON_IDLE),
//         };

//         self.fox_blink_timer += macroquad::time::get_frame_time();
//         if self.fox_blink_timer > self.fox_blink_next {
//             self.fox_blink_timer = 0.0;
//             self.fox_blink_next = gen_range(1.0, 10.0); 
//         }

//         // const BLINK_TIME: f32 = 0.1;
//         let face = match &button_state {
//             _ if self.game.state() == GameState::Lose                    => FoxFace::Dead,
//             _ if self.game.state() == GameState::Win                     => FoxFace::Happy,
//             _ if self.holding                                            => FoxFace::Eek,
//             // ButtonState::Held | ButtonState::Clicked                     => FoxFace::Blink,
//             // _ if self.fox_blink_timer > self.fox_blink_next - BLINK_TIME => FoxFace::Blink,
//             _                                                            => FoxFace::Normal
//         };

//         let rect = rect.offset(Vec2::splat(offset));
//         renderer.draw(DrawShape::image(rect.x+1.0, rect.y+1.0, spritesheet::fox_face(face), None));
//         renderer.draw(DrawShape::nineslice(rect, source));

//         if button_state.released() {
//             self.requesting_new_game = true;
//         }
//     }

//     fn bomb_counter(&self, x: Align, y: Align, renderer: &mut Renderer) {
//         let value = self.game.flags_left();
//         // Calculate the minimum number of digits needed to display the bomb count (always being 2 or larger for style purposes: 3)
//         let digits = ((f32::log10(self.game.bomb_count() as f32) + 1.0).floor() as usize).max(2);

//         let size = spritesheet::counter_size(digits);
//         let rect = aligned_rect(x, y, size.x, size.y);

//         let draw_shapes = (0..digits)
//             // Work out the place value of the current digit
//             .map(|i| (i, 10usize.saturating_pow(i as u32)))
//             .map(|(i, power_of_ten)| (i,
//                 match value {
//                     // If value is none, draw dashes for every character
//                     None => spritesheet::CounterDigit::Dash,
//                     // Otherwise draw a number! This doesn't draw leading zeros, however always renders the first digit, even if it's a zero!
//                     Some(v) if power_of_ten <= v || i == 0 => spritesheet::CounterDigit::Digit((v / power_of_ten) % 10),
//                     _ => spritesheet::CounterDigit::Empty,
//                 }
//             ))
//             .map(|(i, digit)| (i, spritesheet::counter_digit(digit)))
//             // Render the digits in reverse order so they appear the right way around
//             .map(|(i, digit_rect)| DrawShape::image(rect.x + 3.0 + (digit_rect.w + 2.0) * (digits - i - 1) as f32, rect.y + 2.0, digit_rect, None))
//             // Last but not least draw the background
//             .chain(std::iter::once(DrawShape::nineslice(rect, spritesheet::TIMER_BACKGROUND)));
//         renderer.draw_iter(draw_shapes);
//     }
    
//     fn timer(&self, x: Align, y: Align, renderer: &mut Renderer) {
//         let size = spritesheet::TIMER_SIZE;
//         let rect = aligned_rect(x, y, size.x, size.y);

//         let (digits, colon_lit): ([Option<usize>; 4], bool) = if let Some(time) = self.timer {
//             let seconds = (time as usize).min(60*100-1).try_into().unwrap_or(usize::MAX);
//             let digits = [
//                 // Don't display the last digit as a 0 
//                 if seconds < 60*10 { None } else { Some((seconds / 60) / 10) }, // Tens of minutes
//                 Some((seconds / 60) % 10),                                      // Minutes
//                 Some((seconds % 60) / 10),                                      // Tens
//                 Some(seconds % 10),                                             // Units 
//             ];
//             // Originally, the colon flashed every half-second, but I found it a bit distracting :/
//             (digits, true)
//         } else {
//             ([None; 4], false)
//         };

//         let draw_shapes = digits.iter()
//             .zip([2.0, 6.0, 12.0, 16.0])
//             .map(|(&digit, along)| DrawShape::image(rect.x + along, rect.y + 2.0, spritesheet::timer_digit(digit), None))
//             // Draw the colon and the background
//             .chain(std::iter::once(DrawShape::image(rect.x + 10.0, rect.y + 2.0, spritesheet::timer_colon(colon_lit), None)))
//             .chain(std::iter::once(DrawShape::nineslice(rect, spritesheet::TIMER_BACKGROUND)));
//         renderer.draw_iter(draw_shapes);
//     }
// }