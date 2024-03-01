// use ggez::{glam::Vec2, graphics::{Image, Rect}, input::keyboard::KeyCode, Context};

use std::collections::HashMap;

use ggez::{glam::Vec2, graphics::{Canvas, DrawParam, Image, InstanceArray, Rect}, Context};

pub struct TextRenderer {
    map: HashMap<char, (f32, f32, f32)>, // Map of where each char starts and ends in the texture, and it's real length in pixels
    // image: Image,
    batch: InstanceArray,
}

impl TextRenderer {
    pub fn new(ctx: &mut Context, image: Image, character_map: HashMap<char, (f32, f32)>) -> TextRenderer {
        // Normalise all of the positions in the character map
        let mut map: HashMap<char, (f32, f32, f32)> = HashMap::new();
        for (c, (start, length)) in character_map {
            map.insert(c, (start / image.width() as f32, length / image.width() as f32, length));
        }
        let batch = InstanceArray::new(ctx, image);
        TextRenderer { map, batch }
    }
    pub fn char_len(&self, c: char) -> f32 {
        if let Some((_, _, length)) = self.map.get(&c) { *length } else { 0.0 }
    }
    pub fn text_len(&self, text: String) -> f32 {
        text.chars().map(|c| self.char_len(c)).sum()
    }
    pub fn draw_text(&mut self, canvas: &mut Canvas, text: String, draw_param: DrawParam) {
        // Work out the x positions of each character
        let chars: Vec<char> = text.chars().collect();
        let mut char_x_positions: Vec<f32> = Vec::with_capacity(text.len());
        for i in 0..chars.len() {
            // If we're at the first character, start at 0, otherwise set the position to be the previous position plus the length of the previous character
            char_x_positions.push(if i == 0 { 0.0 } else { char_x_positions[i-1] + self.char_len(chars[i-1]) });
        }
        self.batch.set(
            text.chars()
            .map(|character| self.map.get(&character)).flatten().enumerate()
            .map(|(i, char_bounds)| DrawParam::new().src(Rect::new(char_bounds.0, 0.0, char_bounds.1, 1.0)).dest(Vec2::new(char_x_positions[i], 0.0)))
        );
        canvas.draw(&self.batch, draw_param);
    }
}

// // Drawable image
// pub struct DrawableImage {
//     pub middle: Vec2,
//     pub dest_rect: Rect,
//     pub img: Image,
// }

// impl GuiElement {
//     pub fn new(img: Image) -> GuiElement {
//         GuiElement { 
//             middle: Vec2::new(img.width() as f32 / 2.0, img.height() as f32 / 2.0),
//             dest_rect: Rect::one(),
//             img,
//         }
//     }
//     pub fn goto(&mut self, pos: Vec2, scale_factor: f32) {
//         self.dest_rect = Rect::new(pos.x.floor(), pos.y.floor(), scale_factor, scale_factor);
//     }
// }

// pub struct NumberInput {
//     pub value: Option<usize>,
//     pub min: usize,
//     pub max: usize,
//     pub max_length: usize,
//     pub valid: bool,
// }

// impl NumberInput {
//     pub fn new(min: usize, max: usize, max_length: usize) -> NumberInput {
//         NumberInput { value: None, min, max, max_length, valid: false }
//     }
//     pub fn add(&mut self, keycode: KeyCode) {
//         let num_to_push;
//         match keycode {
//             KeyCode::Key0 | KeyCode::Numpad0 => { num_to_push = 0; }
//             KeyCode::Key1 | KeyCode::Numpad1 => { num_to_push = 1; }
//             KeyCode::Key2 | KeyCode::Numpad2 => { num_to_push = 2; }
//             KeyCode::Key3 | KeyCode::Numpad3 => { num_to_push = 3; }
//             KeyCode::Key4 | KeyCode::Numpad4 => { num_to_push = 4; }
//             KeyCode::Key5 | KeyCode::Numpad5 => { num_to_push = 5; }
//             KeyCode::Key6 | KeyCode::Numpad6 => { num_to_push = 6; }
//             KeyCode::Key7 | KeyCode::Numpad7 => { num_to_push = 7; }
//             KeyCode::Key8 | KeyCode::Numpad8 => { num_to_push = 8; }
//             KeyCode::Key9 | KeyCode::Numpad9 => { num_to_push = 9; }
//             // If we press backspace, divide the number by 10
//             KeyCode::Back => { self.value = if self.value.is_some_and(|v| v>=10) { Some(self.value.unwrap()/10) } else { None }; return; }
//             _ => {return;}
//         }
//         let new_value = if let Some(v) = self.value {
//             Some(v*10+num_to_push)
//         } else {
//             Some(num_to_push)
//         };
//         if self.length_valid(new_value) {
//             self.value = new_value
//         }
//     }
//     pub fn length_valid(&self, value: Option<usize>) -> bool {
//         if let Some(v) = value { v.checked_ilog10().unwrap_or(0)as usize+1 <= self.max_length } else { true }
//     }
//     pub fn validity(&mut self) -> bool {
//         // If it's more than or equal to min, less than or equal to max, and the amount of digits is less than or equal to the maximum length
//         self.valid = self.value.is_some_and(|v| v >= self.min && v <= self.max) && self.length_valid(self.value);
//         self.valid
//     }
// }

// pub struct Menu {
//     pub active: bool,
//     pub buttons: Vec<Rect>,
//     pub number_inputs: Vec<NumberInput>,
//     pub gui_element: GuiElement,
// }

// impl Menu {
//     pub fn new(ctx: &Context, width: usize, height: usize, buttons: Vec<Rect>, number_inputs: Vec<NumberInput>) -> Menu {
//         let img = Image::new_canvas_image(ctx, ctx.gfx.surface_format(), 19, 19, 1);
//         let gui_element = GuiElement::new(img);
//         Menu {active: false, buttons, number_inputs, gui_element }
//     }
//     pub fn hovering_button(&self, ctx: &Context) -> Option<usize> {
//         let r = self.gui_element.dest_rect;
//         for (i, b) in self.buttons.iter().enumerate() {
//             let b_t = Rect::new(b.x + r.x, b.y + r.y, b.w * r.w, b.h * r.w);
//             if b_t.contains(ctx.mouse.position()) { return Some(i); }
//         }
//         None
//     }
// }