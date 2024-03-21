use std::collections::HashMap;

use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Image, InstanceArray, Rect},
    Context,
};

const LINE_GAP: f32 = 1.0;
pub const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../../resources/chars.png");
pub fn default_char_map() -> HashMap<char, (f32, f32)> {
    HashMap::from([
        ('A', (3.0, 5.0)),
        ('a', (131.0, 5.0)),
        ('!', (286.0, 2.0)),
        ('0', (237.0, 5.0)),
        ('B', (8.0, 5.0)),
        ('b', (136.0, 4.0)),
        ('?', (288.0, 4.0)),
        ('1', (242.0, 4.0)),
        ('C', (13.0, 5.0)),
        ('c', (140.0, 4.0)),
        (':', (292.0, 2.0)),
        ('2', (246.0, 5.0)),
        ('D', (18.0, 5.0)),
        ('d', (144.0, 5.0)),
        (';', (294.0, 3.0)),
        ('3', (251.0, 5.0)),
        ('E', (23.0, 4.0)),
        ('e', (149.0, 4.0)),
        (',', (297.0, 3.0)),
        ('4', (256.0, 5.0)),
        ('F', (27.0, 4.0)),
        ('f', (153.0, 3.0)),
        ('.', (300.0, 2.0)),
        ('5', (261.0, 5.0)),
        ('G', (31.0, 5.0)),
        ('g', (156.0, 4.0)),
        ('*', (302.0, 4.0)),
        ('6', (266.0, 5.0)),
        ('H', (36.0, 5.0)),
        ('h', (160.0, 4.0)),
        ('#', (306.0, 6.0)),
        ('7', (271.0, 5.0)),
        ('I', (41.0, 4.0)),
        ('i', (164.0, 2.0)),
        ('"', (312.0, 4.0)),
        ('8', (276.0, 5.0)),
        ('J', (45.0, 4.0)),
        ('j', (166.0, 4.0)),
        ('\'', (316.0, 2.0)),
        ('9', (281.0, 5.0)),
        ('K', (49.0, 5.0)),
        ('k', (170.0, 4.0)),
        ('[', (318.0, 3.0)),
        ('L', (54.0, 4.0)),
        ('l', (174.0, 2.0)),
        (']', (321.0, 3.0)),
        ('M', (58.0, 6.0)),
        ('m', (176.0, 6.0)),
        ('(', (324.0, 3.0)),
        ('N', (64.0, 5.0)),
        ('n', (182.0, 4.0)),
        (')', (327.0, 3.0)),
        ('O', (69.0, 5.0)),
        ('o', (186.0, 4.0)),
        ('{', (330.0, 4.0)),
        ('P', (74.0, 5.0)),
        ('p', (190.0, 4.0)),
        ('}', (334.0, 4.0)),
        ('Q', (79.0, 5.0)),
        ('q', (194.0, 4.0)),
        ('<', (338.0, 4.0)),
        ('R', (84.0, 5.0)),
        ('r', (198.0, 4.0)),
        ('>', (342.0, 4.0)),
        ('S', (89.0, 5.0)),
        ('s', (202.0, 4.0)),
        ('-', (346.0, 4.0)),
        ('T', (94.0, 4.0)),
        ('t', (206.0, 4.0)),
        ('+', (350.0, 4.0)),
        ('U', (98.0, 5.0)),
        ('u', (210.0, 4.0)),
        ('/', (354.0, 4.0)),
        ('V', (103.0, 6.0)),
        ('v', (214.0, 4.0)),
        ('=', (358.0, 4.0)),
        ('W', (109.0, 6.0)),
        ('w', (218.0, 6.0)),
        ('_', (362.0, 5.0)),
        ('X', (115.0, 6.0)),
        ('x', (224.0, 4.0)),
        (' ', (0.0, 3.0)),
        ('Y', (121.0, 5.0)),
        ('y', (228.0, 4.0)),
        ('\n', (0.0, 0.0)),
        ('Z', (126.0, 5.0)),
        ('z', (232.0, 5.0)),
        ('¬', (0.0, 1.0)), // used for aligning text by 1 pixel.. a bit hacky but meh
    ]) // (who tf is actually typing '¬'?!)
}

pub struct TextRenderer {
    map: HashMap<char, (f32, f32, f32)>, // Map of where each char starts and ends in the texture, and it's real length in pixels
    batch: InstanceArray,
    height: f32, // The height of each character
}

impl TextRenderer {
    pub fn new(
        ctx: &mut Context,
        image: Image,
        character_map: HashMap<char, (f32, f32)>,
    ) -> TextRenderer {
        // Normalise all of the positions in the character map
        let mut map: HashMap<char, (f32, f32, f32)> = HashMap::new();
        for (c, (start, length)) in character_map {
            map.insert(
                c,
                (
                    start / image.width() as f32,
                    length / image.width() as f32,
                    length,
                ),
            );
        }
        let batch = InstanceArray::new(ctx, image.clone());
        TextRenderer {
            map,
            batch,
            height: image.height() as f32,
        }
    }

    pub fn char_len(&self, c: char) -> f32 {
        if let Some((_, _, length)) = self.map.get(&c) {
            *length
        } else {
            0.0
        }
    }

    pub fn text_size(&self, text: &String) -> Vec2 {
        let line_breaks: Vec<usize> = text
            .char_indices()
            .filter_map(|(i, c)| if c == '\n' { Some(i) } else { None })
            .collect();
        let largest_line_len: f32 =
            split_vector_by_indexes(&text.chars().collect::<Vec<char>>(), &line_breaks)
                .iter()
                .map(|v| v.iter().map(|c| self.char_len(*c)).sum())
                .fold(f32::NEG_INFINITY, f32::max);
        Vec2::new(
            largest_line_len,
            self.height + (self.height + LINE_GAP) * (line_breaks.len()) as f32,
        )
    }
    pub fn text_size_padded(&self, text: &String, padding: (f32, f32, f32, f32)) -> Vec2 {
        self.text_size(text) + Vec2::new(padding.2 + padding.3, padding.0 + padding.1)
    }

    // TODO!! text is drawn the wrong colours for some reason! probably to do with blend modes or srgba or some other shit
    pub fn draw_text(&mut self, canvas: &mut Canvas, text: &String, draw_param: DrawParam) {
        // Work out the positions of each character
        let chars: Vec<char> = text.chars().collect();
        let mut char_positions: Vec<Vec2> = Vec::with_capacity(text.len());
        for i in 0..chars.len() {
            // If we're at the first character, start at 0
            // If the previous character is a line-break, move down to the next line
            // otherwise set the position to be the previous position plus the length of the previous character
            char_positions.push(if i == 0 {
                Vec2::new(0.0, 0.0)
            } else if chars[i - 1] == '\n' {
                Vec2::new(0.0, char_positions[i - 1].y + self.height + LINE_GAP)
            } else {
                char_positions[i - 1] + Vec2::new(self.char_len(chars[i - 1]), 0.0)
            });
        }
        self.batch.set(
            text.chars()
                .map(|character| self.map.get(&character))
                .flatten()
                .enumerate()
                .map(|(i, char_bounds)| {
                    DrawParam::new()
                        .src(Rect::new(char_bounds.0, 0.0, char_bounds.1, 1.0))
                        .dest(char_positions[i])
                }),
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
