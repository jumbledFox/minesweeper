use ggez::mint::Point2;

use super::button::Button;

pub struct MenuBar {
    buttons: Vec<Button>
}

impl MenuBar {
    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Point2<f32>) {
        for b in &mut self.buttons {
            b.update(mouse_free, mouse_pos);
        }
    }

    pub fn draw(&self) {
        for b in &self.buttons {
            b.draw();
        }
    }
}