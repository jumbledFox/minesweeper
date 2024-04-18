// A nice 'black box' game of minesweeper.
// Only handles minesweeper logic and is separate to any rendering or inputs and whatnot.

use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

const NEIGHBOUR_OFFSETS: &[(isize, isize)] = &[
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const MAX_WIDTH: usize = 200;
const MAX_HEIGHT: usize = 100;
const MIN_WIDTH: usize = 4;
const MIN_HEIGHT: usize = 4;

// TODO: Make difficulty a struct and make easy, normal, and hard constants maybe ?
// TODO: Question marks

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy, Normal, Hard,
    Custom {
        width: usize,
        height: usize,
        bomb_count: usize,
    },
}

impl Difficulty {
    pub fn value(&self) -> (usize, usize, usize) {
        match *self {
            Difficulty::Easy => (10, 10, 9),
            Difficulty::Normal => (15, 13, 40),
            Difficulty::Hard => (30, 16, 100),
            Difficulty::Custom {width, height, bomb_count} => {
                // Ensure the fields match the (somewhat arbitrary) limits.
                let (w, h) = (
                    width.clamp(MIN_WIDTH, MAX_WIDTH),
                    height.clamp(MIN_HEIGHT, MAX_HEIGHT),
                );
                let b = bomb_count.min((w - 1) * (h - 1));
                (w, h, b)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Playing,
    Win,
    Lose,
}
#[derive(Debug, PartialEq, Clone)]
pub enum TileType {
    Unopened,
    Flag,
    Dug,
    Numbered(u8),
}

pub enum TimeValue {
    None,
    Some(Instant),
    Frozen(Duration)
}

impl TimeValue {
    pub fn freeze(&mut self) {
        if let TimeValue::Some(instant) = self {
            *self = TimeValue::Frozen(instant.elapsed())
        }
    }

    pub fn duration(&self) -> Duration {
        match self {
            TimeValue::None => Duration::new(0, 0),
            TimeValue::Some(i) => i.elapsed(),
            TimeValue::Frozen(d) => *d,
        }
    }
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    bomb_count: usize,

    board: Vec<TileType>,
    bombs: Vec<usize>,

    state: GameState,
    start_time: TimeValue,
    turns: usize,
}

impl Minesweeper {
    pub fn new(difficulty: Difficulty) -> Minesweeper {
        let (width, height, bomb_count) = difficulty.value();
        let size = width * height;

        let board = vec![TileType::Unopened; size];

        // 'bombs' is only populated after the first move (to make sure the 3*3 area at the first dig is safe). For now it's empty
        let bombs = Vec::with_capacity(bomb_count);

        Minesweeper {
            width,
            height,
            bomb_count,
            board,
            bombs,
            state: GameState::Playing,
            start_time: TimeValue::None,
            turns: 0,
        }
    }

    // Getters
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn bomb_count(&self) -> usize {
        self.bomb_count
    }

    pub fn board(&self) -> &Vec<TileType> {
        &self.board
    }
    pub fn bombs(&self) -> &Vec<usize> {
        &self.bombs
    }

    pub fn state(&self) -> GameState {
        self.state
    }
    pub fn start_time(&self) -> &TimeValue {
        &self.start_time
    }
    pub fn turns(&self) -> usize {
        self.turns
    }

    // How many flags the player needs to have flagged all the bombs
    pub fn flags_left(&self) -> usize {
        let flags_count = self.board.iter().filter(|&t| *t == TileType::Flag).count();
        self.bomb_count.saturating_sub(flags_count)
    }

    // Populates the minefield with bombs, making sure there are no bombs in/neighbouring safe_index
    fn populate_board(&mut self, safe_index: usize) {
        let safe_positions: Vec<usize> = NEIGHBOUR_OFFSETS
            .iter()
            .flat_map(|(x, y)| get_index_from_offset(safe_index, *x, *y, self.width, self.height))
            .chain(std::iter::once(safe_index))
            .collect();
        let mut possible_positions: Vec<usize> = (0..self.board.len())
            .filter(|&i| !safe_positions.contains(&i))
            .collect();
        possible_positions.shuffle(&mut rand::thread_rng());

        self.bombs = possible_positions[..self.bomb_count()].to_vec();
    }

    // Digs at a position, returns true if something happened
    pub fn dig(&mut self, index: usize) -> bool {
        if self.state != GameState::Playing {
            return false;
        }
        if !self.board.get(index).is_some_and(|t| *t == TileType::Unopened) {
            return false;
        }
        if self.turns == 0 {
            self.populate_board(index);
            self.start_time = TimeValue::Some(Instant::now());
        }
        self.turns += 1;

        // We dug a bomb! lose the game and return :c
        if self.bombs.contains(&index) {
            self.state = GameState::Lose;
            self.start_time.freeze();
            return true;
        }
        // Floodfill digging algorithm
        let mut tiles_to_dig = vec![index];
        let mut neighbours: Vec<usize> = Vec::with_capacity(8);
        for _ in 0..self.board.len() {
            for &tile_index in &tiles_to_dig {
                let valid_neighbours = NEIGHBOUR_OFFSETS.iter().flat_map(|(x, y)| {
                    get_index_from_offset(tile_index, *x, *y, self.width, self.height)
                });
                let neighbouring_bombs: u8 = valid_neighbours
                    .clone()
                    .filter(|i| self.bombs.contains(&i))
                    .count() as u8;
                if neighbouring_bombs != 0 {
                    self.board[tile_index] = TileType::Numbered(neighbouring_bombs);
                } else {
                    self.board[tile_index] = TileType::Dug;
                    neighbours.extend(valid_neighbours);
                }
            }
            if neighbours.is_empty() {
                break;
            }
            // Remove all duplicates and non-diggable tiles
            neighbours.sort_unstable();
            neighbours.dedup();
            neighbours.retain(|n_i| self.board[*n_i] == TileType::Unopened);
            // Make tiles to dig for the next iteration the neighbours we found this time. this also clears neighbours
            tiles_to_dig = std::mem::take(&mut neighbours);
        }
        return true;
    }

    // Flags / unflags, returns true if something happened
    pub fn set_flag(&mut self, erasing_flags: bool, index: usize) -> bool {
        if self.state != GameState::Playing {
            return false;
        }
        let tile = match self.board.get_mut(index) {
            None => return false,
            Some(t) => t,
        };
        match erasing_flags {
            true => {
                if *tile == TileType::Flag {
                    *tile = TileType::Unopened;
                    return true;
                }
            }
            false => {
                if *tile == TileType::Unopened {
                    *tile = TileType::Flag;
                    return true;
                }
            }
        }
        return false;
    }
}

fn get_index_from_offset(index: usize, x_offset: isize, y_offset: isize,  width: usize, height: usize) -> Option<usize> {
    let x = match (index % width).checked_add_signed(x_offset) {
        Some(x) if x < width => x,
        _ => return None,
    };
    let y = match (index / width).checked_add_signed(-y_offset) {
        Some(y) if y < height => y,
        _ => return None,
    };
    // Safe way of doing (y * width + x)
    match y.checked_mul(width) {
        Some(i) => i.checked_add(x),
        None => None,
    }
}
