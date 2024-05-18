// Makes the bombs explode in a nice circle, very efficient if I do say so myself :3

use indexmap::IndexMap;
use macroquad::time::get_frame_time;

use crate::{minesweeper::Minesweeper, ui::renderer::Renderer};

#[derive(Default)]
pub struct Exploder {
    map: IndexMap<usize, (f32, bool)>,
    map_skip: usize,
    
    radius: f32,
    radius_expansion: f32,
    
    effect_timer: f32
}

impl Exploder {
    pub fn contains(&self, key: &usize) -> bool {
        self.map.contains_key(key)
    }
    
    pub fn index_exploded(&self, key: &usize) -> Option<bool> {
        self.map.get(key).and_then(|(_, e)| Some(*e))
    }

    pub fn map_mut(&mut self) -> &mut IndexMap<usize, (f32, bool)> {
        &mut self.map
    }

    pub fn reset(&mut self) {
        self.map.clear();
        self.radius = 0.0;
        self.map_skip = 0;
        self.effect_timer = f32::MAX;
    }

    pub fn initialise(&mut self, start_index: usize, game: &mut Minesweeper) {
        self.reset();

        let index_to_coord = |index: usize| {(
            (index % game.width()) as f32,
            (index / game.width()) as f32,
        )};

        // Calculate all of the bombs squared distances to the center
        let (center_x, center_y) = index_to_coord(start_index);
        for bomb_index in game.bombs() {
            let (x, y) = index_to_coord(*bomb_index);
            // a^2 + b^2 = c^2, thanks Pythagoras
            let squared_distance = (center_x - x).powi(2) + (center_y - y).powi(2);
            self.map.insert(*bomb_index, (squared_distance, false));
        }

        // Sort the map by distance
        self.map.sort_unstable_by(|_, (a, _), _, (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less));
        
        // TODO: Make this better
        self.radius_expansion = (usize::max(game.width(), game.height()) as f32).sqrt() * 2.0;
    }

    pub fn update(&mut self, renderer: &mut Renderer) {
        // If we've exploded all of the bombs, we don't need to do anything more!!
        if self.map_skip == self.map.len() {
            return;
        }
        // This makes it so the first bomb explodes, then after 0.7 seconds the rest explode
        self.radius += get_frame_time() * match self.radius < 1.0 {
            true  => 1.0/0.7,
            false => self.radius_expansion,
        };

        let squared_radius = self.radius.powi(2);
        let prev_skip = self.map_skip;

        // Iterate from the first non-exploded bomb
        for (squared_distance, exploded) in self.map.values_mut().skip(self.map_skip) {
            match squared_radius > *squared_distance {
                // If this bomb is inside the circle, increase the skip index and explode it! 
                true  => { self.map_skip += 1; *exploded = true; }
                // Otherwise, it's outside, meaning all of the ones after this are outisde, so stop!!
                false => break
            }
        } 
        // If we've exploded at least one bomb (and we haven't done this too recently), shake the screen and play the explosion sound
        self.effect_timer += get_frame_time();
        if self.map_skip != prev_skip && self.effect_timer > 0.1 {
            self.effect_timer = 0.0;
            renderer.shake(1.0);
            renderer.sound_player().play_explosion();
        }
    }
}