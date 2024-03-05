use ggez::conf::{WindowMode, WindowSetup};
use ggez::ContextBuilder;
use ggez::event;

pub mod minesweeper;
pub mod mainstate;
pub mod gui;
pub mod rendering;
pub mod game;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Minesweeper", "jumbledFox")
        .window_mode(WindowMode {
            resizable: false,
            visible: false,
            ..Default::default()
        })
        .window_setup(WindowSetup {
            title: String::from("jumbledFox's Minesweeper"),
            ..Default::default()
        })
        .build()
        .expect("Couldn't create GGEZ context!!! wtf!!");

    // Run!!
    let main_state = game::MainState::new(&mut ctx);
    event::run(ctx, event_loop, main_state);
}