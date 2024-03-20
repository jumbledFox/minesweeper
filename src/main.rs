use ggez::conf::{WindowMode, WindowSetup};
use ggez::ContextBuilder;
use ggez::event;

pub mod minesweeper;
pub mod mainstate;
pub mod gui;
pub mod rendering;
pub mod game;

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

    let mut g = minesweeper::Minesweeper::new(minesweeper::Difficulty::Easy);
    draw_minefield(&g);
    g.dig(16);
    draw_neighbours(&g);
    draw_minefield(&g);
    g.dig(40);
    draw_minefield(&g);
}

fn draw_minefield(game: &minesweeper::Minesweeper) {
    for (i, b) in game.board.iter().enumerate() {
        if i % game.width == 0 { println!(""); }
        print!("{}", match b {
            minesweeper::TileType::Unopened => String::from("[ ]"),
            minesweeper::TileType::Flag => String::from(" F "),
            minesweeper::TileType::Dug => match game.neighbour_count[i] {
                0 => String::from(" . "),
                _ => format!(" {} ", game.neighbour_count[i]),
            },
        });
    }
    println!("");
}

fn draw_neighbours(game: &minesweeper::Minesweeper) {
    for (i, b) in game.board.iter().enumerate() {
        if i % game.width == 0 { println!(""); }
        if game.bombs.contains(&i) {
            print!(" * ");
        } else {
            print!(" {} ", match game.neighbour_count[i] {
                0 => String::from("."),
                _ => game.neighbour_count[i].to_string(),
            });            
        }
    }
    println!("");
}