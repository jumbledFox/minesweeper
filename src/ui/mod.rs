use std::hash::{DefaultHasher, Hash, Hasher};

use self::{menubar::Menubar, popups::Popups, renderer::Renderer, state::State};

pub fn hash_string(input: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

pub mod state;
pub mod menubar;
pub mod popups;
pub mod elements;
pub mod renderer;
pub mod spritesheet;

pub struct Ui {
    pub state: State,
    pub menubar: Menubar,
    pub popups: Popups,
    pub renderer: Renderer,
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            state:    State   ::new(),
            menubar:  Menubar ::default(),
            popups:   Popups  ::default(),
            renderer: Renderer::new(),
        }
    }

    pub fn begin(&mut self) {
        self.state.begin();
        self.renderer.begin();
    }

    pub fn finish(&mut self) {
        self.state.finish();
        self.renderer.finish();
    }
}
