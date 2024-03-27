use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};
use mainstate::MainState;
use rand::Rng;

pub mod minesweeper;
pub mod mainstate;
pub mod elements;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Minesweeper", "jumbledFox")
        .window_mode(WindowMode {
            resizable: false,
            // visible: false,
            ..Default::default()
        })
        .window_setup(WindowSetup {
            title: String::from("jumbledFox's Minesweeper"),
            ..Default::default()
        })
        .build()
        .expect("Couldn't create GGEZ context!!! wtf!!");

    // Run!!
    // let main_state = game::MainState::new(&mut ctx);
    // event::run(ctx, event_loop, main_state);

    // let mut g = minesweeper::Minesweeper::new(minesweeper::Difficulty::Custom {
    //     width: 200,
    //     height: 100,
    //     bomb_count: 50,
    // });
    // let mut g = minesweeper::Minesweeper::new(minesweeper::Difficulty::Hard);
    let main_state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, main_state);
}