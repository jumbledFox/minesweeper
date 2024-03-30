use ggez::{glam::Vec2, graphics::{Canvas, Rect}, mint::{Point2, Vector2}};

use super::{button::{self, Button}, text_renderer::TextRenderer, MouseAction};

pub struct MenuBar {
    buttons: Vec<Button>
}

impl MenuBar {
    pub fn new(text_renderer: &TextRenderer) -> MenuBar {
        let mut buttons: Vec<Button> = Vec::with_capacity(25);
        buttons.push(Button::new_labelled_auto(text_renderer, Rect::new(50.0, 10.0, 50.0, 10.0), String::from("Start"), button::PressMode::Release, false));
        buttons.push(Button::new_labelled_auto(text_renderer, Rect::new(50.0, 25.0, 50.0, 10.0), String::from("Exit"), button::PressMode::Release, false));
        MenuBar { buttons }
    }

    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Vec2, mouse_action: &(MouseAction, MouseAction)) {
        for b in self.buttons.iter_mut().rev() {
            b.update(mouse_free, mouse_pos, &mouse_action.0);
            // if b.pressed() {
            //     println!("{:?} pressed", b.label())
            // }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, text_renderer: &mut TextRenderer) {
        for b in &self.buttons {
            button::draw_button_flat(&b, canvas, text_renderer);
        }
    }

    pub fn pressed(&mut self, id: usize) -> bool {
        if let Some(button) = self.buttons.get_mut(id) {
            button.pressed()
        } else {
            false
        }
    }
}