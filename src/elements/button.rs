use ggez::{glam::Vec2, graphics::{self, Canvas, Color, DrawParam, Rect}, mint::Point2};

use super::{text_renderer::TextRenderer, MouseAction};

#[derive(PartialEq, Debug)]
pub enum ButtonState {
    Idle, Hovered, Pressed, Disabled,
}

#[derive(PartialEq)]
pub enum PressMode {
    Press, Release,
}

pub struct Button {
    label: String,
    label_pos: Point2<f32>,

    rect: Rect,
    state: ButtonState,
    press_mode: PressMode,
    pressed: bool,
}

impl Button {
    // Makes a new button without a label
    pub fn new(rect: Rect, press_mode: PressMode, disabled: bool) -> Button {
        Button {
            label: String::new(),
            label_pos: Point2 { x: 0.0, y: 0.0 },
            rect,
            state: if disabled { ButtonState::Disabled } else { ButtonState::Idle },
            press_mode,
            pressed: false,
        }
    }

    // Makes a new button given some text, that fits the text plus some padding
    pub fn new_labelled(text_renderer: &TextRenderer, label: String, position: Vec2, padding: (f32, f32, f32, f32), press_mode: PressMode, disabled: bool) -> Button {
        let label_size = text_renderer.text_size_padded(&label, padding);
        Button {
            label,
            label_pos: Point2 { x: padding.2, y: padding.1 },
            rect: Rect::new(position.x, position.y, label_size.x, label_size.y),
            state: if disabled { ButtonState::Disabled } else { ButtonState::Idle },
            press_mode,
            pressed: false,
        }
    }

    // Makes a new button given some text and a rect, puts the text in the middle of the rect
    pub fn new_labelled_auto(text_renderer: &TextRenderer, rect: Rect, label: String, press_mode: PressMode, disabled: bool) -> Button {
        let text_size = text_renderer.text_size(&label);
        let label_pos = Point2 { x: (rect.w - text_size.x) / 2.0, y: (rect.h - text_size.y) / 2.0 };
        Button {
            label,
            label_pos,
            rect,
            state: if disabled { ButtonState::Disabled } else { ButtonState::Idle },
            press_mode,
            pressed: false,
        }
    }

    // Updates the button
    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Vec2, left_mouse_action: &MouseAction) {
        // If the mouse isn't free, or if it's not over the button, make the button idle and return
        if !*mouse_free || !self.rect.contains(mouse_pos) {
            if self.state != ButtonState::Disabled {
                self.state = ButtonState::Idle;
            }
            return;
        }
        // At this point, the button is guaranteed to capture the mouse and run its normal logic
        *mouse_free = false;
        // If the button is disabled, don't do anything
        if self.state == ButtonState::Disabled { return; }
        // Update the button depending on the action of the left mouse button
        self.state = match left_mouse_action {
            // If the mouse has been pressed, make the button pressed
            MouseAction::Press => {
                if self.press_mode == PressMode::Press { self.pressed = true; }
                ButtonState::Pressed
            }
            // If the mouse has been released, make the button hovered
            MouseAction::Release => {
                if self.press_mode == PressMode::Release { self.pressed = true; }
                ButtonState::Hovered
            }
            // If the button is pressed down, keep it pressed down, otherwise make it hovered
            _ => {
                if self.state == ButtonState::Pressed {
                    ButtonState::Pressed
                } else {
                    ButtonState::Hovered
                }
            }
        };
    }

    // Returns whether the button was pressed and resets the pressed flag
    pub fn pressed(&mut self) -> bool {
        let return_value = self.pressed;
        self.pressed = false;
        return return_value;
    }

    // Getters
    pub fn label(&self) -> &String { &self.label }
}

// Draws a button with no fancy border, just normal and flat
pub fn draw_button_flat(button: &Button, canvas: &mut Canvas, text_renderer: &mut TextRenderer) {
    let (text_col, rect_col) = match button.state {
        ButtonState::Idle     => (super::TEXT_IDLE,     super::BACKGROUND_IDLE),
        ButtonState::Disabled => (super::TEXT_DISABLED, super::BACKGROUND_IDLE),
        ButtonState::Hovered | ButtonState::Pressed => (Color::WHITE, super::BACKGROUND_HOVERED),
    };
    // Draw the background
    canvas.draw(&graphics::Quad, DrawParam::new().dest_rect(button.rect).color(rect_col));
    // Draw the text
    let text_pos = Point2 { x: button.rect.x + button.label_pos.x, y: button.rect.y + button.label_pos.y };
    text_renderer.draw_text(canvas, &button.label, DrawParam::new().dest(text_pos).color(text_col));
}

// Draws a button with a fancier border
pub fn draw_button_fancy(button: &Button) {

}