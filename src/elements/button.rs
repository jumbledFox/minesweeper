use ggez::{graphics::Rect, mint::Point2};

#[derive(PartialEq)]
pub enum ButtonState {
    Idle, Hovered, Pressed, Disabled,
}

#[derive(PartialEq)]
pub enum PressMode {
    Press, Release,
}

pub struct Button {
    rect: Rect,
    state: ButtonState,
    press_mode: PressMode,
}

impl Button {
    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Point2<f32>) {
        // If the mouse has already been captured, or if the mouse isn't over the button, set the button to its idle state and return
        if !*mouse_free || !self.rect.contains(mouse_pos) {
            if self.state != ButtonState::Disabled {
                self.state = ButtonState::Idle;
            }
            return;
        }
        // If the mouse is over the button, capture it!
        if self.rect.contains(mouse_pos) { *mouse_free = true; }
        // If the button is disabled, don't do anything
        if self.state == ButtonState::Disabled { return; }

        
    }

    pub fn draw(&self) {

    }
}