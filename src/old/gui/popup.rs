use ggez::{glam::Vec2, graphics::Rect};

pub enum PopupKind {
    Exit,
}

pub struct Popup {
    pub rect: Rect,
    pub kind: PopupKind,
    pub hovering_over: bool,
}

impl Popup {
    pub fn new(kind: PopupKind, window_middle: Vec2) -> Popup {
        let size = match kind {
            PopupKind::Exit => Vec2::new(40.0, 20.0),
        };
        let pos = (window_middle - size / 2.0).round();
        Popup {
            rect: Rect::new(pos.x, pos.y, size.x, size.y),
            kind,
            hovering_over: false,
        }
    }
    pub fn update(&mut self, mouse_pos: Vec2) {
        self.hovering_over = self.rect.contains(mouse_pos)
    }
}
