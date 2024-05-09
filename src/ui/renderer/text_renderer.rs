use std::collections::HashMap;

use macroquad::prelude::*;

// TODO: https://www.dafont.com/04b-03.font ?
fn default_char_map() -> HashMap<char, (f32, f32)> {
    HashMap::from([
        ('A', (3.0,   5.0)), ('a', (131.0, 5.0)), ('!', (287.0, 2.0)), ('0', (238.0, 5.0)),
        ('B', (8.0,   5.0)), ('b', (136.0, 4.0)), ('?', (289.0, 4.0)), ('1', (243.0, 4.0)),
        ('C', (13.0,  5.0)), ('c', (140.0, 4.0)), (':', (293.0, 2.0)), ('2', (247.0, 5.0)),
        ('D', (18.0,  5.0)), ('d', (144.0, 5.0)), (';', (295.0, 3.0)), ('3', (252.0, 5.0)),
        ('E', (23.0,  4.0)), ('e', (149.0, 5.0)), (',', (298.0, 3.0)), ('4', (257.0, 5.0)),
        ('F', (27.0,  4.0)), ('f', (154.0, 3.0)), ('.', (301.0, 2.0)), ('5', (262.0, 5.0)),
        ('G', (31.0,  5.0)), ('g', (157.0, 4.0)), ('*', (303.0, 4.0)), ('6', (267.0, 5.0)),
        ('H', (36.0,  5.0)), ('h', (161.0, 4.0)), ('#', (307.0, 6.0)), ('7', (272.0, 5.0)),
        ('I', (41.0,  4.0)), ('i', (165.0, 2.0)), ('"', (313.0, 4.0)), ('8', (277.0, 5.0)),
        ('J', (45.0,  4.0)), ('j', (167.0, 4.0)), ('\'',(317.0, 2.0)), ('9', (282.0, 5.0)),
        ('K', (49.0,  5.0)), ('k', (171.0, 4.0)), ('[', (319.0, 3.0)), ('\n',(  0.0, 0.0)),
        ('L', (54.0,  4.0)), ('l', (175.0, 2.0)), (']', (322.0, 3.0)), ('¬', (  0.0, 1.0)),
        ('M', (58.0,  6.0)), ('m', (177.0, 6.0)), ('(', (325.0, 3.0)), // ^ used for aligning text by 1 pixel.. a bit hacky but meh
        ('N', (64.0,  5.0)), ('n', (183.0, 4.0)), (')', (328.0, 3.0)), // (who tf is actually typing '¬'?!)
        ('O', (69.0,  5.0)), ('o', (187.0, 4.0)), ('{', (331.0, 4.0)),
        ('P', (74.0,  5.0)), ('p', (191.0, 4.0)), ('}', (335.0, 4.0)),
        ('Q', (79.0,  5.0)), ('q', (195.0, 4.0)), ('<', (339.0, 4.0)),
        ('R', (84.0,  5.0)), ('r', (199.0, 4.0)), ('>', (343.0, 4.0)),
        ('S', (89.0,  5.0)), ('s', (203.0, 4.0)), ('-', (347.0, 4.0)),
        ('T', (94.0,  4.0)), ('t', (207.0, 4.0)), ('+', (351.0, 4.0)),
        ('U', (98.0,  5.0)), ('u', (211.0, 4.0)), ('/', (355.0, 4.0)),
        ('V', (103.0, 6.0)), ('v', (215.0, 4.0)), ('\\',(357.0, 4.0)),
        ('W', (109.0, 6.0)), ('w', (219.0, 6.0)), ('=', (363.0, 4.0)),
        ('X', (115.0, 6.0)), ('x', (225.0, 4.0)), ('_', (367.0, 5.0)),
        ('Y', (121.0, 5.0)), ('y', (229.0, 4.0)), ('|', (372.0, 2.0)),
        ('Z', (126.0, 5.0)), ('z', (233.0, 5.0)), (' ', (  0.0, 3.0)),
    ])
}

pub struct TextRenderer {
    chars_texture: Texture2D,
    // For each character, where it starts and how wide it is
    char_map: HashMap<char, (f32, f32)>,
    error_char: char,
}


// Some of the code for carets and finding out where the user clicked is a bit messy, but it works nicely.
#[derive(Clone, Copy)]
pub struct Caret {
    pub index: usize,
    pub color: Color,
}

impl TextRenderer {
    pub fn new() -> TextRenderer {
        let chars_texture = Texture2D::from_file_with_format(include_bytes!("../../../resources/chars.png"), None);
        chars_texture.set_filter(FilterMode::Nearest);
        TextRenderer { 
            chars_texture,
            char_map: default_char_map(),
            error_char: '?',
        }
    }

    pub fn line_gap(&self, line_gap: Option<f32>) -> f32 {
        self.chars_texture.height() + line_gap.unwrap_or(1.0)
    }

    // Gets the start and width of a char
    pub fn character_values(&self, c: char) -> (f32, f32) {
        match self.char_map.get(&c) {
            Some(c) => *c,
            None => *self.char_map.get(&self.error_char).unwrap_or(&(0.0, 0.0))
        }
    }

    // TODO: Think about how and when I use AsRef<str> in this and other parts of the code.
    // https://www.reddit.com/r/learnrust/comments/14s0k5x/using_asrefstr_as_ref_and_to_owned/
    pub fn draw_text(&self, text: &String, caret: Option<Caret>, click_pos: Option<Vec2>, x: f32, y: f32, color: Color, line_gap: Option<f32>) -> Option<usize> {
        let mut caret_pos = None;
        let mut pos = Vec2::new(x, y);
        let mut last_pos;
        let mut clicked = None;
        for (i, c) in text.chars().enumerate() {
            last_pos = pos;
            if c == '\n' {
                pos = vec2(x, pos.y + self.line_gap(line_gap));
                // continue;
            }
            if caret.is_some_and(|c| c.index == i) {
                caret_pos = Some(pos);
            }
            let (start, width) = self.character_values(c);
            let params = DrawTextureParams {
                source: Some(Rect::new(start, 0.0, width, 6.0)),
                ..Default::default()
            };
            draw_texture_ex(&self.chars_texture, pos.x, pos.y, color, params);
            pos.x += width;
            match (click_pos, clicked) {
                (Some(click_pos), None) => {
                    if click_pos.x < pos.x {
                        clicked = match pos.x - click_pos.x - 1.5 < click_pos.x - last_pos.x {
                            true  => Some(i+1),
                            false => Some(i),
                        };
                    }
                }
                _ => ()
            }
        }
        if clicked.is_none() && click_pos.is_some() {
            clicked = Some(text.len());
        }
        if caret.is_some_and(|c| c.index == text.len()) {
            caret_pos = Some(pos);
        }

        if let Some(caret_pos) = caret_pos {
            let (start, width) = self.character_values('|');
            let params = DrawTextureParams {
                source: Some(Rect::new(start, 0.0, width, 6.0)),
                ..Default::default()
            };
            draw_texture_ex(&self.chars_texture, (caret_pos.x-1.0).max(x), caret_pos.y, caret.unwrap().color, params);
        }

        clicked
    }

    pub fn text_size(&self, text: &String, line_gap: Option<f32>) -> Vec2 {
        let line_breaks: Vec<usize> = text
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '\n' => Some(i),
                _ => None,
            })
            .collect();
        let largest_line_len: f32 =
            split_vector_by_indexes(&text.chars().collect::<Vec<char>>(), &line_breaks)
                .iter()
                .map(|v| v.iter().map(|c| self.character_values(*c).1).sum())
                .fold(0.0, f32::max);

        let height = (line_breaks.len() + 1) as f32
            * (self.chars_texture.height() + line_gap.unwrap_or(1.0))
            - line_gap.unwrap_or(1.0);

        Vec2::new(largest_line_len, height)
    }
}

// TODO: I think there's a better way to do this with slices and fold, but this works for now
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
