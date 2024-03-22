use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::ContextBuilder;
use rand::Rng;

pub mod game;
pub mod gui;
pub mod mainstate;
pub mod minesweeper;
pub mod rendering;

pub mod gui_ideas;

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
    // let main_state = game::MainState::new(&mut ctx);
    // event::run(ctx, event_loop, main_state);

    // let mut g = minesweeper::Minesweeper::new(minesweeper::Difficulty::Custom {
    //     width: 200,
    //     height: 100,
    //     bomb_count: 50,
    // });
    let mut g = minesweeper::Minesweeper::new(minesweeper::Difficulty::Custom { width: 30, height: 30, bomb_count: 2 });
    draw_minefield(&g);
    for _ in 0..99999 {
        let index = rand::thread_rng().gen_range(0..g.board().len());
        if match rand::thread_rng().gen_bool(0.0) {
            true  => g.dig(index),
            false => g.set_flag(false, index),
        } == false { continue; }
        draw_minefield(&g);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn draw_minefield(game: &minesweeper::Minesweeper) {
    println!("turns: {:?}", game.turns());
    for (i, b) in game.board().iter().enumerate() {
        if i % game.width() == 0 { println!(""); }
        print!(
            "{}", match b {
                minesweeper::TileType::Unopened => String::from("[ ]"),
                minesweeper::TileType::Flag => String::from(" F "),
                minesweeper::TileType::Numbered(n) => format!(" {:?} ", n),
                minesweeper::TileType::Dug => {
                    if game.bombs().contains(&i) { String::from(" * ") }
                    else {  String::from(" . ") }
                }
            }
        );
    }
    println!("");
}
