// Generic UI stuff

use std::hash::{DefaultHasher, Hash, Hasher};

use macroquad::math::Rect;

use self::{menubar::Menubar, minesweeper_element::MinesweeperElement, popups::Popups, renderer::Renderer, state::State};

pub fn hash_string(input: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

pub trait Round {
    fn round(self) -> Self;
}
impl Round for Rect {
    fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.w.round(), self.h.round())
    }
}

pub mod state;
pub mod menubar;
pub mod popups;
pub mod minesweeper;
pub mod minesweeper_element;
pub mod elements;
pub mod renderer;
pub mod spritesheet;

pub struct Ui {
    pub state: State,
    pub menubar: Menubar,
    pub popups: Popups,
    pub minesweeper_element: MinesweeperElement,
    pub renderer: Renderer,
}

impl Ui {
    pub async fn new() -> Ui {
        Ui {
            state:   State  ::new(),
            menubar: Menubar::default(),
            popups:  Popups ::default(),
            minesweeper_element: MinesweeperElement::new().await, 
            renderer: Renderer::new(),
        }
    }

    pub fn begin(&mut self) {
        self.state.begin(&self.menubar);//, &self.minesweeper_element);
        self.renderer.begin(&self.state);
    }

    pub fn finish(&mut self) {
        self.state.finish();
        self.renderer.finish(&self.state, &self.menubar);
    }
}
