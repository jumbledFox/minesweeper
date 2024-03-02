use std::collections::HashMap;

use ggez::{glam::Vec2, graphics::{Canvas, DrawParam, Image, InstanceArray, Rect}, Context};

pub struct TextRenderer {
    map: HashMap<char, (f32, f32, f32)>, // Map of where each char starts and ends in the texture, and it's real length in pixels
    batch: InstanceArray,
    pub height: f32, // The height of each character
}

impl TextRenderer {
    pub fn new(ctx: &mut Context, image: Image, character_map: HashMap<char, (f32, f32)>) -> TextRenderer {
        // Normalise all of the positions in the character map
        let mut map: HashMap<char, (f32, f32, f32)> = HashMap::new();
        for (c, (start, length)) in character_map {
            map.insert(c, (start / image.width() as f32, length / image.width() as f32, length));
        }
        let batch = InstanceArray::new(ctx, image.clone());
        TextRenderer { map, batch, height: image.height() as f32 }
    }
    pub fn char_len(&self, c: char) -> f32 {
        if let Some((_, _, length)) = self.map.get(&c) { *length } else { 0.0 }
    }
    pub fn text_size(&self, text: &String) -> Vec2 {
        let line_breaks: Vec<usize> = text.char_indices().filter_map(|(i, c)| if c == '\n' { Some(i) } else { None }).collect();
        let largest_line_len: f32 = split_vector_by_indexes(&text.chars().collect::<Vec<char>>(), &line_breaks)
            .iter().map(|v| v.iter()
            .map(|c| self.char_len(*c)).sum())
            .fold(f32::NEG_INFINITY, f32::max);
        // 1.0 is for the 1 pixel gap between lines
        Vec2::new(largest_line_len, self.height + (self.height + 1.0) * (line_breaks.len()) as f32)
    }
    pub fn draw_text(&mut self, canvas: &mut Canvas, text: &String, draw_param: DrawParam) {
        // Work out the positions of each character
        let chars: Vec<char> = text.chars().collect();
        let mut char_positions: Vec<Vec2> = Vec::with_capacity(text.len());
        for i in 0..chars.len() {
            // If we're at the first character, start at 0
            // If the previous character is a line-break, move down to the next line
            // otherwise set the position to be the previous position plus the length of the previous character
            char_positions.push(if i == 0 { Vec2::new(0.0, 0.0) }
                else if chars[i-1] == '\n' { Vec2::new(0.0, char_positions[i-1].y + self.height + 1.0) }
                else { char_positions[i-1] + Vec2::new(self.char_len(chars[i-1]), 0.0) });
        }
        self.batch.set(
            text.chars()
            .map(|character| self.map.get(&character)).flatten().enumerate()
            .map(|(i, char_bounds)| DrawParam::new().src(Rect::new(char_bounds.0, 0.0, char_bounds.1, 1.0)).dest(char_positions[i]))
        );
        canvas.draw(&self.batch, draw_param);
    }
}

fn split_vector_by_indexes<T: Clone>(vector: &[T], indexes: &[usize]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut start_index = 0;

    for &end_index in indexes {
        if end_index <= vector.len() {
            result.push(vector[start_index..end_index].to_vec());
            start_index = end_index;
        }
    }

    // Push the remaining elements after the last index
    if start_index < vector.len() {
        result.push(vector[start_index..].to_vec());
    }
    result
}