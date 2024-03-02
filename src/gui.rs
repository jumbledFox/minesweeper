pub mod text_renderer;
pub mod button;
pub mod menu_bar;
pub mod dropdown;
pub use text_renderer::TextRenderer;
pub use button::Button;
pub use menu_bar::MenuBar;
pub use dropdown::Dropdown;

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