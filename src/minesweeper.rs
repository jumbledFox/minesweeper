// A nice 'black box' game of minesweeper.
// Only handles minesweeper logic and is separate to any rendering or inputs and whatnot.

use macroquad::{math::bool, rand::ChooseRandom};

const NEIGHBOUR_OFFSETS: &[(isize, isize)] = &[
    (-1,  1), (0,  1), (1,  1),
    (-1,  0),          (1,  0),
    (-1, -1), (0, -1), (1, -1),
];
const MAX_WIDTH:  usize = 200;
const MAX_HEIGHT: usize = 100;
const MIN_WIDTH:  usize = 4;
const MIN_HEIGHT: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy, Normal, Hard,
    Custom(DifficultyValues),
}
#[derive(Debug, Clone, Copy)]
pub struct DifficultyValues {
    width: usize,
    height: usize,
    bomb_count: usize,
}

impl Difficulty {
    pub fn custom(width: usize, height: usize, bomb_count: usize) -> Self {
        // Ensure the fields match the (somewhat arbitrary) limits.
        let width  = width .clamp(MIN_WIDTH,  MAX_WIDTH);
        let height = height.clamp(MIN_HEIGHT, MAX_HEIGHT);
        let bomb_count = bomb_count.min((width - 1) * (height - 1));
        Self::Custom(DifficultyValues { width, height, bomb_count })
    }

    pub fn values(&self) -> DifficultyValues {
        let (width, height, bomb_count) = match *self {
            Self::Easy   => (9, 9, 9),
            Self::Normal => (16, 16, 40),
            Self::Hard   => (30, 16, 100),
            Self::Custom (difficulty_values) => return difficulty_values,
        };
        DifficultyValues { width, height, bomb_count }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Playing,
    Win,
    Lose,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Unopened,
    Flag,
    Dug,
    Numbered(u8),
}

pub enum Timer {
    None,
    Timing(f64),
    Frozen(f64)
}

impl Timer {
    pub fn start(&mut self) {
        *self = Self::Timing(macroquad::time::get_time())
    }
    pub fn freeze(&mut self) {
        if let Self::Timing(_) = self {
            *self = Self::Frozen(self.time_since());
        }
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
    pub fn time_since(&self) -> f64 {
        match self {
            Self::None => 0.0,
            Self::Timing(time) => macroquad::time::get_time()-time,
            Self::Frozen(time) => *time,
        }
    }
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    bomb_count: usize,

    board: Vec<Tile>,
    bombs: Vec<usize>,

    floodfill_current: Vec<usize>,
    floodfill_next:    Vec<usize>,

    state: GameState,
    timer: Timer,
    turns: usize,
}

impl Minesweeper {
    pub fn new(difficulty: Difficulty) -> Minesweeper {
        // Holy shit rust is the best fucking language ever made for allowing this
        let DifficultyValues { width, height, bomb_count } = difficulty.values();
        Minesweeper {
            width, height, bomb_count,

            board: vec![Tile::Unopened; width * height],
            // 'bombs' is only populated after the first move (to make sure the 3*3 area at the first dig is safe). For now it's empty
            bombs: Vec::with_capacity(bomb_count),

            floodfill_current: Vec::with_capacity(width * height),
            floodfill_next:    Vec::with_capacity(width * height),

            state: GameState::Playing,
            timer: Timer::None,
            turns: 0,
        }
    }

    // Getters
    pub fn width(&self)      -> usize { self.width }
    pub fn height(&self)     -> usize { self.height }
    pub fn bomb_count(&self) -> usize { self.bomb_count }

    pub fn board(&self) -> &Vec<Tile>  { &self.board }
    pub fn bombs(&self) -> &Vec<usize> { &self.bombs }

    pub fn state(&self) -> GameState  { self.state }
    pub fn timer(&self) -> &Timer     { &self.timer }
    pub fn turns(&self) -> usize      { self.turns }

    // How many flags the player needs to have flagged all the bombs
    pub fn flags_left(&self) -> usize {
        let flags_count = self.board.iter().filter(|&t| *t == Tile::Flag).count();
        self.bomb_count.saturating_sub(flags_count)
    }

    // Populates the minefield with bombs, making sure there are no bombs in/neighbouring safe_index
    fn populate_board(&mut self, safe_index: usize) {
        let safe_positions: Vec<usize> = NEIGHBOUR_OFFSETS
            .iter()
            .flat_map(|(x, y)| get_index_from_offset(safe_index, *x, *y, self.width, self.height))
            .chain(std::iter::once(safe_index))
            .collect();
        // TODO: Make sure there are no more than 4 mines in the 5x5 are, to decrease the liklihood of annoying spawns
        let mut possible_positions: Vec<usize> = (0..self.board.len())
            .filter(|&i| !safe_positions.contains(&i))
            .collect();
        possible_positions.shuffle();

        self.bombs = possible_positions[..self.bomb_count()].to_vec();
    }

    pub fn diggable(&mut self, index: usize) -> bool {
        self.state == GameState::Playing
        && self.board.get(index).is_some_and(|t| *t == Tile::Unopened)
    }

    // Digs at a position, returns true if something happened
    // TODO: Chording !!
    pub fn dig(&mut self, index: usize) -> bool {
        if !self.diggable(index) {
            return false;
        }
        if self.turns == 0 {
            self.populate_board(index);
            self.timer.start();
        }
        self.turns += 1;

        // We dug a bomb! lose the game and return :c
        if self.bombs.contains(&index) {
            self.state = GameState::Lose;
            self.timer.freeze();
            return true;
        }
        // Floodfill digging algorithm
        self.floodfill_current.clear();
        self.floodfill_current.push(index);
        self.floodfill_next.clear();

        for _ in 0..self.board.len() {
            for &tile_index in &self.floodfill_current {
                let valid_neighbours = NEIGHBOUR_OFFSETS.iter().flat_map(|(x, y)| {
                    get_index_from_offset(tile_index, *x, *y, self.width, self.height)
                });
                let neighbouring_bombs: u8 = valid_neighbours
                    .clone()
                    .filter(|i| self.bombs.contains(&i))
                    .count() as u8;

                if neighbouring_bombs != 0 {
                    self.board[tile_index] = Tile::Numbered(neighbouring_bombs);
                } else {
                    self.board[tile_index] = Tile::Dug;
                    self.floodfill_next.extend(valid_neighbours);
                }
            }
            if self.floodfill_next.is_empty() {
                break;
            }
            // Remove all duplicates and non-diggable tiles
            self.floodfill_next.sort_unstable();
            self.floodfill_next.dedup();
            self.floodfill_next.retain(|n_i| self.board[*n_i] == Tile::Unopened);
            // Make tiles to dig for the next iteration the neighbours we found this time. this also clears neighbours
            self.floodfill_current = std::mem::take(&mut self.floodfill_next);
        }

        // For each diggable tile, see if there's a bomb under it. If there aren't any without bombs under them, the game has been won!
        let game_won = !self.board
            .iter().enumerate()
            .filter(|&(_, t)| *t == Tile::Flag || *t == Tile::Unopened)
            .map(|(i, _)| self.bombs().contains(&i))
            .any(|has_bomb| !has_bomb);

        if game_won {
            self.state = GameState::Win;
            self.timer.freeze();
        }
        true
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
        match *tile {
            Tile::Unopened if !erasing_flags => *tile = Tile::Flag,
            Tile::Flag     if  erasing_flags => *tile = Tile::Unopened,
            _ => return false,
        }
        true
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
    y.checked_mul(width).and_then(|f| f.checked_add(x))
}
