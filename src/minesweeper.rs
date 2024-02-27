use std::time::Instant;

use rand::prelude::*;

#[derive(PartialEq)]
pub enum GameState {
    Prelude, Playing, Win, Lose
}
#[derive(PartialEq)]
pub enum TileType {
    Unopened, Dug, Flag,
}

impl rand::distributions::Distribution<TileType> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0..=2) { // rand 0.8
            0 => TileType::Unopened,
            1 => TileType::Dug,
            _ => TileType::Dug,
        }
    }
}

pub struct Minesweeper {
    pub width : usize,
    pub height: usize,
    pub bomb_count: usize,

    pub board: Vec<TileType>,
    pub bombs: Vec<usize>,
    pub neighbour_count: Vec<u8>,

    pub selected_tile: Option<usize>,
    pub state: GameState,
    pub start_time: Instant,

    // Used for the lose animation, holds the indexes of all of the bombs that should be drawn as an explosion rather than a bomb.
    pub exploded_bombs: Vec<usize>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, bomb_count: usize) -> Minesweeper {
        let size = width*height;

        if bomb_count > size - 9 {
            println!("Bomb count is bigger than max! you silly goose :P");
        }
        let bomb_count = bomb_count.min(size - 9);

        let mut board = Vec::with_capacity(size);
        for _ in 0..size {
            board.push(rand::random());
        }
        // The bombs and neighbour_count values are only populated properly after the first move.
        // This is because we want the cell at the user's first dig, and all of it's neighbours, to never be a bomb.
        // So before that they're empty / filled with dummy values.

        let mut bombs = Vec::with_capacity(bomb_count);
        bombs.append(&mut vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let mut neighbour_count = Vec::with_capacity(size);
        for _ in 0..size {
            neighbour_count.push(thread_rng().gen_range(0..=8));
        }

        Minesweeper { width, height, bomb_count,
            board, bombs, neighbour_count,
            selected_tile: Some(width + 1), state: GameState::Prelude, start_time: Instant::now(),
            exploded_bombs: vec![]
        }
    }

    pub fn dig(&mut self) {
        if self.state == GameState::Prelude {
            self.state = GameState::Playing;
            self.start_time = Instant::now();
        }
    }
}