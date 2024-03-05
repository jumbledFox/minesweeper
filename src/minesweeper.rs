use std::time::Instant;

use rand::prelude::*;

const MAX_WIDTH : usize = 200;
const MAX_HEIGHT: usize = 100;

#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy, Normal, Hard, Custom(usize, usize, usize)
}

#[derive(PartialEq)]
pub enum GameState {
    Prelude, Playing, Win, Lose
}
#[derive(PartialEq, Clone)]
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

// TODO: Maybe make this struct only hold game logic related things and be pure of rendering
pub struct Minesweeper {
    // Game logic
    pub width : usize,
    pub height: usize,
    pub bomb_count: usize,

    pub board: Vec<TileType>,
    pub bombs: Vec<usize>,
    pub neighbour_count: Vec<u8>,

    pub state: GameState,
    pub start_time: Instant,
}

impl Minesweeper {
    pub fn new(difficulty: Difficulty) -> Minesweeper {
        let (width, height, bomb_count) = Minesweeper::difficulty_values(difficulty);
        let size = width*height;

        let bomb_count = bomb_count.min(size - 9);

        let mut board = Vec::with_capacity(size);
        for _ in 0..size {
            // board.push(rand::random());
            board.push(TileType::Unopened);
        }
        // The bombs and neighbour_count values are only populated properly after the first move.
        // This is because we want the cell at the user's first dig, and all of it's neighbours, to never be a bomb.
        // So before that they're empty / filled with dummy values.

        let bombs = (0..bomb_count).collect();

        let mut neighbour_count = Vec::with_capacity(size);
        for _ in 0..size {
            neighbour_count.push(thread_rng().gen_range(0..=8));
        }

        Minesweeper { width, height, bomb_count,
            board, bombs, neighbour_count,
            state: GameState::Prelude, start_time: Instant::now(),
        }
    }

    pub fn difficulty_values(difficulty: Difficulty) -> (usize, usize, usize) {
        match difficulty {
            Difficulty::Easy   => (10, 10,  9),
            Difficulty::Normal => (15, 13, 40),
            Difficulty::Hard   => (30, 16, 99),
            Difficulty::Custom(w, h, b) => {
                // Ensure it matches the (somewhat arbitrary) limit.
                let (width, height) = (w.min(MAX_WIDTH), h.min(MAX_HEIGHT));
                let bomb_count = b.min((width-1)*(height-1));
                (width, height, bomb_count)
            },
        }
    }

    pub fn playing_state(&self) -> bool {
        match self.state {
            GameState::Prelude | GameState::Playing => true,
            _ => false,
        }
    }

    pub fn dig(&mut self, index: usize) -> bool {
        // If the tile is valid and diggable...
        if let Some(tile) = self.board.get_mut(index) {
            if *tile == TileType::Unopened {
                if self.state == GameState::Prelude {
                    // Generate the board, bombs and stuff
        
                    // Start the timer
                    self.start_time = Instant::now();
                    self.state = GameState::Playing;
                }
                *tile = TileType::Dug;
                return true;
            }
        }
        false
    }

    // Toggle a flag at a position, checks if the index is valid, as well as if flagging that tile is a valid move
    // Returns if the operation was successful
    pub fn set_flag(&mut self, erasing_flags: bool, index: usize) -> bool {
        // If the index is valid
        if let Some(tile) = self.board.get_mut(index) {
            // Add or remove a flag, depending on 'erasing_flags'
            match erasing_flags {
                true  => if *tile == TileType::Flag { *tile = TileType::Unopened; return true; },
                false => if *tile == TileType::Unopened { *tile = TileType::Flag; return true; },
            }
        }
        false
    }
}