use std::any::Any;

use ggez::{glam::Vec2, graphics::{Canvas, Color, DrawParam, Rect}};


pub struct Renderer {

}
impl Renderer {
    pub fn draw_text(&mut self, text: &String, canvas: &mut Canvas, draw_param: DrawParam) {

    }
}

// Allows easily repositioning rects
trait Translate {
    fn translated(&self, pos: Vec2) -> Rect;
}
impl Translate for Rect {
    fn translated(&self, pos: Vec2) -> Rect {
        Rect::new(
            self.x + pos.x,
            self.y + pos.y,
            self.w,
            self.h,
        )
    } 
}

#[derive(PartialEq)]
pub enum MouseAction {
    None, Down, Up,
}

pub trait GuiElement {
    // Updates the GUI element with the mouse. returns if the mouse was captured by this element (as in hovering, etc)
    fn update(&mut self, _mouse_pos: Vec2, _mouse_action: MouseAction, _parent_pos: Vec2) -> bool { false }
    // Sets the element to it's default state, should be called if update wasn't called
    fn default(&mut self) { }
    fn mouse_over(&self, _mouse_pos: Vec2, _parent_pos: Vec2) -> bool { false }
    fn draw(&self, _canvas: &mut Canvas, _renderer: &mut Renderer, _parent_pos: Vec2) { }

    fn as_any(&mut self) -> &mut dyn Any;
}


pub struct Label {
    pos: Vec2,
    text: String,
    color: Color,
}

impl GuiElement for Label {
    fn as_any(&mut self) -> &mut dyn Any { self }

    fn draw(&self, canvas: &mut Canvas, renderer: &mut Renderer, parent_pos: Vec2) {
        renderer.draw_text(&self.text, canvas, DrawParam::new().color(self.color).dest(parent_pos + self.pos));
    }
}

#[derive(PartialEq)]
pub enum ButtonState {
    Idle, Hovered, Pressed, Disabled,
}
#[derive(PartialEq)]
pub enum ButtonTrigger {
    Press, Release,
}
pub struct Button {
    rect: Rect,
    trigger: ButtonTrigger,
    state: ButtonState,
    pressed: bool,
    // draw_type: (nineslice, blank background, etc)
}
impl Button {
    pub fn new(rect: Rect, trigger: ButtonTrigger, disabled: bool) -> Button {
        Button {
            rect, trigger,
            state: if disabled { ButtonState::Disabled } else { ButtonState::Idle },
            pressed: false
        }
    }
    // Returns if the button was pressed or not and resets the pressed flag if so. This means we will never miss a press!
    pub fn pressed(&mut self) -> bool {
        match self.pressed {
            true  => { self.pressed = false; true }
            false => false
        }
    }
}

impl GuiElement for Button {
    fn as_any(&mut self) -> &mut dyn Any { self }

    fn update(&mut self, mouse_pos: Vec2, mouse_action: MouseAction, parent_pos: Vec2) -> bool {
        // If the mouse isn't over the button, we don't want to update it, and we don't CARE!!!
        if !self.mouse_over(mouse_pos, parent_pos) {
            self.default();
            return false;
        }
        // Do logic for pressing, releasing, hovering, etc
        match self.state {
            ButtonState::Pressed if mouse_action == MouseAction::Up => {
                if self.trigger == ButtonTrigger::Release { self.pressed = true; }
                self.state = ButtonState::Hovered;
            }
            ButtonState::Idle | ButtonState::Hovered if mouse_action == MouseAction::Down => {
                if self.trigger == ButtonTrigger::Press   { self.pressed = true; }
                self.state = ButtonState::Pressed;
            }
            ButtonState::Idle => self.state = ButtonState::Hovered,
            _ => {}
        }
        // By this point we've already established the button is being hovered over so we return true after doing this logic
        true
    }
    fn default(&mut self) {
        // Make the button IDLE, unless it's disabled, in which case it'll STAY disabled
        if self.state != ButtonState::Disabled { self.state = ButtonState::Idle; }
    }
    fn mouse_over(&self, mouse_pos: Vec2, parent_pos: Vec2) -> bool {
        self.rect.translated(parent_pos).contains(mouse_pos)
    }
    fn draw(&self, canvas: &mut Canvas, _renderer: &mut Renderer, parent_pos: Vec2) {
        let col = match self.state {
            ButtonState::Idle     => Color::from_rgb(196, 196, 196),
            ButtonState::Hovered  => Color::from_rgb(240, 240, 240),
            ButtonState::Pressed  => Color::from_rgb(100, 100, 100),
            ButtonState::Disabled => Color::from_rgb( 64,   0,   0),
        };
        canvas.draw(&ggez::graphics::Quad, DrawParam::new()
            .dest_rect(self.rect.translated(parent_pos))
            .color(col));
    }
}

pub struct LabeledButton {
    pos: Vec2,
    button: Button,
    label: Label,
}

impl LabeledButton {
    // pub fn new(text: String, pos: Vec2, trigger: ButtonTrigger, disabled: bool) -> LabeledButton {
        
    // }
}

impl GuiElement for LabeledButton {
    fn as_any(&mut self) -> &mut dyn Any { self }

    fn update(&mut self, mouse_pos: Vec2, mouse_action: MouseAction, parent_pos: Vec2) -> bool {
        self.button.update(mouse_pos, mouse_action, self.pos + parent_pos)
    }
    fn default(&mut self) {
        self.button.default();
    }
    fn mouse_over(&self, mouse_pos: Vec2, parent_pos: Vec2) -> bool {
        self.button.mouse_over(mouse_pos, parent_pos)
    }
    fn draw(&self, canvas: &mut Canvas, renderer: &mut Renderer, parent_pos: Vec2) {
        self.button.draw(canvas, renderer, self.pos + parent_pos);
        self.label.draw(canvas, renderer, self.pos + parent_pos);
    }
}

/*
update() {
    loop through GuiElements until one update returns true, that means it's used the mouse
    update through the rest and call default()
}
draw() {
    loop through GuiElements and call draw();
}
*/
/*
// menubar -> holds vec of buttons generated from string vec

// // A container holds a bunch of gui elements
// trait Container {
//     pub fn update(&mut self, mouse_pos) -> bool;
// }


// // basically, the gui is made up of a bunch of containers
// hover(mouse_pos) {

// }

// mouse_action(mouse_pos, UpOrDown)

// enum GuiElement {
//     Label({text: String}),
//     Button({rect: pressed})
// }
*/