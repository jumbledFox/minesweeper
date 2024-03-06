use ggez::{glam::Vec2, graphics::{Canvas, Color, DrawParam, Rect}};


pub enum Renderer {

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

trait GuiElement {
    // Returns if the mouse was captured by this element (as in hovering, etc)
    fn update(&mut self, _mouse_pos: Vec2, _mouse_action: MouseAction, _parent_pos: Vec2) -> bool { false }
    fn draw(&self, _parent_pos: Vec2, _canvas: &mut Canvas, _renderer: &mut Renderer) { }
}


pub struct Label {
    pos: Vec2,
    text: String,
    color: Color,
}

impl GuiElement for Label {
    fn draw(&self, parent_pos: Vec2, canvas: &mut Canvas, renderer: &mut Renderer) {
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
    fn update(&mut self, mouse_pos: Vec2, mouse_action: MouseAction, parent_pos: Vec2) -> bool {
        // If the mouse isn't over the button, we don't want to update it, and we don't CARE!!!
        if !self.rect.translated(parent_pos).contains(mouse_pos) {
            // Make the button IDLE, unless it's disabled, in which case it'll STAY disabled
            if self.state != ButtonState::Disabled { self.state = ButtonState::Idle; }
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
    fn draw(&self, parent_pos: Vec2, canvas: &mut Canvas, renderer: &mut Renderer) {

    }
}

/*
update() {
    loop through GuiElements until one update returns true, that means it's used the mouse
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