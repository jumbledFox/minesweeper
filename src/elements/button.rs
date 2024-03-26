use ggez::{graphics::{self, Canvas, Color, DrawParam, Rect}, mint::Point2};

use super::text_renderer::TextRenderer;

#[derive(PartialEq)]
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
}

impl Button {
    // Makes a new button without a label
    pub fn new(rect: Rect, press_mode: PressMode, disabled: bool) -> Button {
        Button {
            label: String::new(),
            label_pos: Point2 { x: 0.0, y: 0.0 },
            rect,
            state: if disabled { ButtonState::Disabled } else { ButtonState::Idle },
            press_mode
        }
    }

    // Makes a new button given some text, that fits the text plus some padding
    pub fn new_labelled(text_renderer: TextRenderer, label: String, position: Point2<f32>, padding: (f32, f32, f32, f32), press_mode: PressMode, disabled: bool) -> Button {
        let label_size = text_renderer.text_size_padded(&label, padding);
        Button {
            label,
            label_pos: Point2 { x: padding.2, y: padding.1 },
            rect: Rect::new(position.x, position.y, label_size.x, label_size.y),
            state: if disabled { ButtonState::Disabled } else { ButtonState::Idle },
            press_mode
        }
    }

    pub fn update(&mut self, mouse_free: &mut bool, mouse_pos: Point2<f32>) {
        // If the mouse isn't free, make the button idle and return
        if !*mouse_free {
            self.idle();
            return;
        }
        // If the mouse isn't over the button, make it idle and return
        if !self.rect.contains(mouse_pos) {
            self.idle();
            return;
        }
        // At this point, the button is guaranteed to capture the mouse and run its normal logic 
        *mouse_free = false;
        // If the button is disabled, don't do anything
        if self.state == ButtonState::Disabled { return; }

        // TODO: rest of the button logic
    }

    // Makes the button idle, unless it's disabled
    pub fn idle(&mut self) {
        if self.state != ButtonState::Disabled {
            self.state = ButtonState::Idle;
        }
    }
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
    text_renderer.draw_text(canvas, &button.label, DrawParam::new().dest(button.label_pos).color(text_col));
}

// Draws a button with a fancier border
pub fn draw_button_fancy(button: &Button) {

}