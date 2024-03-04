pub mod text_renderer;
pub mod button;
pub mod menu_bar;
pub mod dropdown;
use ggez::glam::Vec2;
pub use text_renderer::TextRenderer;
pub use button::Button;
pub use menu_bar::MenuBar;
pub use dropdown::Dropdown;

#[derive(PartialEq, Clone, Copy)]
pub enum MousePressMode {
    None, Down, Up,
}
pub struct Gui {
    pub menu_bar: MenuBar
}

impl Gui {
    pub fn new(menu_bar: MenuBar) -> Gui {
        Gui { menu_bar }
    }
    pub fn update(&mut self, mouse_pos: Vec2, mouse_mode: MousePressMode) {
        self.menu_bar.update(mouse_pos, mouse_mode);
    }
    pub fn hovering(&mut self, mouse_pos: Vec2) -> bool {
        // TODO: make better?
        self.menu_bar.hovering_over
    }
}