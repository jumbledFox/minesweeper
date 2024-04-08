use macroquad::prelude::*;

use ::phf::{phf_map, Map};

// For each character, where it starts and how wide it is
static CHAR_MAP: Map<char, (f32, f32)> = phf_map! {
    'A' => (3.0,   5.0), 'a' => (131.0, 5.0), '!' => (286.0, 2.0), '0' => (237.0, 5.0),
    'B' => (8.0,   5.0), 'b' => (136.0, 4.0), '?' => (288.0, 4.0), '1' => (242.0, 4.0),
    'C' => (13.0,  5.0), 'c' => (140.0, 4.0), ':' => (292.0, 2.0), '2' => (246.0, 5.0),
    'D' => (18.0,  5.0), 'd' => (144.0, 5.0), ';' => (294.0, 3.0), '3' => (251.0, 5.0),
    'E' => (23.0,  4.0), 'e' => (149.0, 4.0), ',' => (297.0, 3.0), '4' => (256.0, 5.0),
    'F' => (27.0,  4.0), 'f' => (153.0, 3.0), '.' => (300.0, 2.0), '5' => (261.0, 5.0),
    'G' => (31.0,  5.0), 'g' => (156.0, 4.0), '*' => (302.0, 4.0), '6' => (266.0, 5.0),
    'H' => (36.0,  5.0), 'h' => (160.0, 4.0), '#' => (306.0, 6.0), '7' => (271.0, 5.0),
    'I' => (41.0,  4.0), 'i' => (164.0, 2.0), '"' => (312.0, 4.0), '8' => (276.0, 5.0),
    'J' => (45.0,  4.0), 'j' => (166.0, 4.0), '\''=> (316.0, 2.0), '9' => (281.0, 5.0),
    'K' => (49.0,  5.0), 'k' => (170.0, 4.0), '[' => (318.0, 3.0), '\n'=> (  0.0, 0.0),
    'L' => (54.0,  4.0), 'l' => (174.0, 2.0), ']' => (321.0, 3.0), '¬' => (  0.0, 1.0), // used for aligning text by 1 pixel.. a bit hacky but meh
    'M' => (58.0,  6.0), 'm' => (176.0, 6.0), '(' => (324.0, 3.0),                      // (who tf is actually typing '¬'?!)
    'N' => (64.0,  5.0), 'n' => (182.0, 4.0), ')' => (327.0, 3.0),
    'O' => (69.0,  5.0), 'o' => (186.0, 4.0), '{' => (330.0, 4.0),
    'P' => (74.0,  5.0), 'p' => (190.0, 4.0), '}' => (334.0, 4.0),
    'Q' => (79.0,  5.0), 'q' => (194.0, 4.0), '<' => (338.0, 4.0),
    'R' => (84.0,  5.0), 'r' => (198.0, 4.0), '>' => (342.0, 4.0),
    'S' => (89.0,  5.0), 's' => (202.0, 4.0), '-' => (346.0, 4.0),
    'T' => (94.0,  4.0), 't' => (206.0, 4.0), '+' => (350.0, 4.0),
    'U' => (98.0,  5.0), 'u' => (210.0, 4.0), '/' => (354.0, 4.0),
    'V' => (103.0, 6.0), 'v' => (214.0, 4.0), '\\'=> (358.0, 4.0),
    'W' => (109.0, 6.0), 'w' => (218.0, 6.0), '=' => (362.0, 4.0),
    'X' => (115.0, 6.0), 'x' => (224.0, 4.0), '_' => (366.0, 5.0),
    'Y' => (121.0, 5.0), 'y' => (228.0, 4.0), ' ' => (  0.0, 3.0),
    'Z' => (126.0, 5.0), 'z' => (232.0, 5.0),
};
const ERROR_CHAR: char = '?';

pub struct TextRenderer {
    chars_texture: Texture2D,
}

impl TextRenderer {
    pub fn new() -> TextRenderer {
        let chars_texture =
            Texture2D::from_file_with_format(include_bytes!("../../resources/chars.png"), None);
        chars_texture.set_filter(FilterMode::Nearest);

        TextRenderer { chars_texture }
    }

    // Gets the start and width of a c (or the error character if 'c' isn't in the CHAR_MAP)
    pub fn character_values(c: char) -> (f32, f32) {
        *CHAR_MAP
            .get(if CHAR_MAP.contains_key(&c) {
                &c
            } else {
                &ERROR_CHAR
            })
            .expect("ERROR_CHAR not a key in CHAR_MAP!")
    }

    pub fn draw_text(&self, text: &String, x: f32, y: f32, color: Color, line_gap: Option<f32>) {
        let mut x_pos = 0.0;
        let mut y_pos = 0.0;
        for c in text.chars() {
            if c == '\n' {
                x_pos = 0.0;
                y_pos += self.chars_texture.height() + line_gap.unwrap_or(1.0);
                continue;
            }

            let (start, width) = TextRenderer::character_values(c);

            draw_texture_ex(
                &self.chars_texture,
                x + x_pos,
                y + y_pos,
                color,
                DrawTextureParams {
                    source: Some(Rect::new(start, 0.0, width, 6.0)),
                    ..Default::default()
                },
            );
            x_pos += width;
        }
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
                .map(|v| v.iter().map(|c| TextRenderer::character_values(*c).1).sum())
                .fold(f32::NEG_INFINITY, f32::max);

        let height = (line_breaks.len() + 1) as f32
            * (self.chars_texture.height() + line_gap.unwrap_or(1.0))
            - line_gap.unwrap_or(1.0);

        Vec2::new(largest_line_len, height)
    }
}

// I think there's a better way to do this with slices and fold..
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
