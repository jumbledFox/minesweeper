use std::collections::HashSet;

use macroquad::{audio::{load_sound_from_bytes, PlaySoundParams, Sound}, camera::{set_camera, Camera2D}, color::WHITE, input::MouseButton, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, render_target, DrawTextureParams, RenderTarget}};

use crate::{minesweeper::{get_index_from_offset, Difficulty, GameState, Minesweeper, SetFlagMode, Tile, NEIGHBOUR_OFFSETS}, ui::{elements::{aligned_rect, Align}, hash_string, renderer::{DrawShape, Renderer}, spritesheet, state::{Id, State}}};

use super::exploder::Exploder;

pub struct Minefield {
    flag_mode:   Option<SetFlagMode>,
    losing_tile: Option<usize>,
    chording:     bool,
    chorded:      bool,
    about_to_dig: bool,

    id: Id,
    render_target: RenderTarget,

    sound_flag:      Option<Sound>,
    sound_explosion: Option<Sound>,
    sound_win:       Option<Sound>,
}

impl Minefield {
    pub async fn new(difficulty: Difficulty) -> Minefield {
        Minefield {
            flag_mode:   None,
            losing_tile: None,
            chording:     false,
            chorded:      false,
            about_to_dig: false,
            id:  hash_string(&format!("if you're reading this... say hi :3 {:?}", macroquad::rand::rand())),
            render_target: Minefield::render_target(difficulty),
            sound_flag:      load_sound_from_bytes(include_bytes!("../../../resources/congrats.ogg" )).await.ok(),
            sound_explosion: load_sound_from_bytes(include_bytes!("../../../resources/explosion.ogg")).await.ok(),
            sound_win:       load_sound_from_bytes(include_bytes!("../../../resources/congrats.ogg" )).await.ok(),
        }
    }

    pub fn about_to_dig(&self) -> bool { self.about_to_dig }
    pub fn id(&self)           -> Id   { self.id }
    pub fn size(&self, game: &Minesweeper) -> Vec2 {
        vec2(game.width() as f32, game.height() as f32) * 9.0
    }

    pub fn min_size(&self, game: &Minesweeper) -> Vec2 {
        self.size(game) + 10.0
    }

    pub fn update(&mut self, area: Rect, game: &mut Minesweeper, exploder: &mut Exploder, state: &mut State, renderer: &mut Renderer) {
        let size = self.size(game);
        let rect = aligned_rect(Align::Mid(area.x + area.w / 2.0), Align::Mid(area.y + area.h / 2.0), size.x, size.y);
        // Make sure it doesn't go above the area (plus some more leeway :3)
        let rect = Rect::new(rect.x, f32::max(rect.y, area.y + 2.0 + 2.0).floor(), rect.w, rect.h);

        self.about_to_dig = false;

        let mut chorded_tiles: HashSet<usize> = HashSet::new();

        if state.hot_item.assign_if_none_and(self.id, state.mouse_in_rect(rect)) {
            let hovered_tile_coord = ((state.mouse_pos() - rect.point()) / 9.0).floor();
            let selected_tile = hovered_tile_coord.y as usize * game.width() + hovered_tile_coord.x as usize;

            // Draw the selector thingy
            let selector_pos = rect.point() + hovered_tile_coord * 9.0 - 1.0;
            renderer.draw(DrawShape::image(selector_pos.x, selector_pos.y, spritesheet::MINEFIELD_SELECTED, None));

            // Interacting
            let any_mouse_down = state.mouse_down(MouseButton::Left) || state.mouse_down(MouseButton::Middle) || state.mouse_down(MouseButton::Right);
            state.active_item.assign_if_none_and(self.id, any_mouse_down);

            let prev_game_state = game.state();
            let bomb_index = self.interact(state.active_item == self.id, selected_tile, rect.point(), &mut chorded_tiles, game, state, renderer);

            // If we've lost on this frame, start exploding bombs!
            if prev_game_state.is_playing() && game.state().is_lose() {
                let bomb_index = bomb_index.unwrap_or(selected_tile);
                self.losing_tile = Some(bomb_index);
                exploder.initialise(bomb_index, game);
            }
            // If we've won on this frame, play the win sound etc
            if prev_game_state.is_playing() && game.state().is_win() {
                play_sound(&self.sound_win, 1.0);
                // TODO: Other winning stuff
            }
        }
        // Explode :3
        if game.state() == GameState::Lose {
            exploder.update(&self.sound_explosion, renderer);
        }

        // Rendering the board
        // Drawing the minefield to a texture is much better than a million billion trillion renderer.draw(DrawShape)s
        set_camera(&Camera2D {
            zoom: 2.0 / self.render_target.texture.size(),
            target: self.render_target.texture.size() / 2.0,
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });
        let texture = &renderer.texture();
        let draw_tile = |index: usize, id: usize| {
            let pos = 9.0 * Vec2::new(
                (index % game.width()) as f32,
                (index / game.width()) as f32
            );
            draw_texture_ex(texture, pos.x, pos.y, WHITE, DrawTextureParams {
                source: Some(spritesheet::minefield_tile(id)),
                ..Default::default()
            });
        };
        // Draw each of the tiles
        for (i, t) in game.board().iter().enumerate() {
            // Draw the background of the tile
            draw_tile(i, match t {
                _ if self.losing_tile == Some(i)                         => 4, // The losing tile
                Tile::Dug | Tile::Numbered(_)                            => 1, // A dug tile
                Tile::Flag if exploder.index_exploded(&i) != Some(true)  => 0, // A flag that's not got an exploded bomb below it
                _ if exploder.contains(&i) || chorded_tiles.contains(&i) => 1, // A bomb or a tile being chorded
                _                                                        => 0, // Unopened
            });
            // Draw the icon on top of the tile
            match (t, exploder.index_exploded(&i)) {
                (Tile::Flag, None) if game.state().is_lose() => Some(6), // Incorrect flag
                (Tile::Flag, Some(true))                     => Some(7), // Exploded flag
                (Tile::Flag, _)                              => Some(5), // Flag
                (_, Some(false))                             => Some(2), // Unexploded bomb
                (_, Some(true))                              => Some(3), // Exploded bomb
                (Tile::Numbered(n), _)                       => Some(*n as usize + 7), // Number
                _ => None,
            }.map(|id| draw_tile(i, id));
        }
        // Draw the texture and the border with the renderer
        let border_rect = Rect::new(rect.x - 2.0, rect.y - 2.0, rect.w + 4.0, rect.h + 4.0);
        renderer.draw(DrawShape::texture(rect.x, rect.y, self.render_target.texture.clone()));
        renderer.draw(DrawShape::nineslice(border_rect, spritesheet::MINEFIELD_BORDER));
    }

    // Handles mouse interaction with the minefield
    pub fn interact(
        &mut self,
        is_active: bool,
        selected_tile: usize,
        pos: Vec2,
        chorded_tiles: &mut HashSet<usize>,
        game:     &mut Minesweeper,
        state:    &mut State,
        renderer: &mut Renderer
    ) -> Option<usize> {
        // Chording
        // If we were trying to chord and any of the mouse buttons have been released, chord!
        let any_mouse_released = state.mouse_released(MouseButton::Middle) || state.mouse_released(MouseButton::Left) || state.mouse_released(MouseButton::Right);
        if self.chording && any_mouse_released {
            (self.chording, self.chorded) = (false, true);
            return game.chord(selected_tile);
        };

        // We only want to be chording if the minefield is active and we're holding the right button(s) 
        self.chording = is_active && ((state.mouse_down(MouseButton::Middle) || (state.mouse_down(MouseButton::Left) && state.mouse_down(MouseButton::Right))));
        // Draw the chorded tiles
        if self.chording {
            self.about_to_dig = true;
            // TODO: Maybe just renderer.draw_iter this
            chorded_tiles.extend(NEIGHBOUR_OFFSETS
                .iter()
                .chain(std::iter::once(&(0, 0)))
                .flat_map(|(x, y)| get_index_from_offset(selected_tile, *x, *y, game.width(), game.height()))
                .filter(|i| game.board().get(*i).is_some_and(|t| *t == Tile::Unopened))
            );
        }
        
        // We only want to stop being chorded if none of the mouse buttons are down
        if self.chorded && !state.mouse_down(MouseButton::Middle) && !state.mouse_down(MouseButton::Left) && !state.mouse_down(MouseButton::Right) {
            self.chorded = false;
            return None;
        }
        // We don't want to dig or flag if we're chording, or we've chorded and we haven't let go
        if self.chording || self.chorded {
            return None;
        }

        // Digging
        if is_active && state.mouse_released(MouseButton::Left) {
            game.dig(selected_tile);
            return Some(selected_tile);
        }
        // If about to dig, draw a tile being dug
        if is_active && state.mouse_down(MouseButton::Left) && game.diggable(selected_tile) {
            self.about_to_dig = true;
            // TODO: Maybe add a nice indented dig sprite
            renderer.draw(DrawShape::image(
                pos.x + (selected_tile%game.width()) as f32 * 9.0,
                pos.y + (selected_tile/game.width()) as f32 * 9.0,
                spritesheet::minefield_tile(1),
                None
            ));
        }

        // Flagging
        if matches!(self.flag_mode, Some(SetFlagMode::Flag)) || !state.mouse_down(MouseButton::Right) {
            self.flag_mode = None;
        }
        if is_active && state.mouse_pressed(MouseButton::Right) {
            self.flag_mode = match game.board().get(selected_tile).is_some_and(|t| *t != Tile::Flag) {
                true  => Some(SetFlagMode::Flag),
                false => Some(SetFlagMode::Remove),
            }
        }
        if let Some(flag_mode) = self.flag_mode {
            if game.set_flag(flag_mode, selected_tile) {
                play_sound(&self.sound_flag, 0.0);
            }
        }

        None
    }

    pub fn new_game(&mut self, difficulty: Difficulty) {
        self.flag_mode     = None;
        self.losing_tile   = None;
        self.render_target = Minefield::render_target(difficulty);
    }

    fn render_target(difficulty: Difficulty) -> RenderTarget {
        let r = render_target(difficulty.values().width() as u32 * 9, difficulty.values().height() as u32 * 9);
        r.texture.set_filter(macroquad::texture::FilterMode::Nearest);
        r
    }
}

fn play_sound(sound: &Option<Sound>, volume: f32) {
    if let Some(sound) = sound {
        macroquad::audio::play_sound(sound, PlaySoundParams { looped: false, volume })
    }
}