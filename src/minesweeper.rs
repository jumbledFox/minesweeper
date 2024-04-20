// A nice 'black box' game of minesweeper.
// Only handles minesweeper logic and is separate to any rendering or inputs and whatnot.

use macroquad::rand::ChooseRandom;

const NEIGHBOUR_OFFSETS: &[(isize, isize)] = &[
    (-1,  1), (0,  1), (1,  1),
    (-1,  0),          (1,  0),
    (-1, -1), (0, -1), (1, -1),
];
const MAX_WIDTH:  usize = 200;
const MAX_HEIGHT: usize = 100;
const MIN_WIDTH:  usize = 4;
const MIN_HEIGHT: usize = 4;

// TODO: MAYBE Make difficulty a struct and make easy, normal, and hard constants maybe ?
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
            Self::Easy   => (9, 9, 9),
            Self::Normal => (16, 16, 40),
            Self::Hard   => (30, 16, 100),
            Self::Custom {width, height, bomb_count} => {
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
pub enum Tile {
    Unopened,
    // TODO: Question,
    Flag,
    Dug,
    Numbered(u8),
}

pub enum Time {
    None,
    Some(f64),
    Frozen(f64)
}

impl Time {
    pub fn start() -> Self {
        Self::Some(macroquad::time::get_time())
    }
    pub fn freeze(&mut self) {
        *self = Self::Frozen(self.time_since());
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Time::None)
    }
    pub fn time_since(&self) -> f64 {
        match self {
            Self::None => 0.0,
            Self::Some(time) => macroquad::time::get_time()-time,
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

    state: GameState,
    start_time: Time,
    turns: usize,
}

impl Minesweeper {
    pub fn new(difficulty: Difficulty) -> Minesweeper {
        let (width, height, bomb_count) = difficulty.value();
        Minesweeper {
            width, height, bomb_count,
            board: vec![Tile::Unopened; width * height],
            // 'bombs' is only populated after the first move (to make sure the 3*3 area at the first dig is safe). For now it's empty
            bombs: Vec::with_capacity(bomb_count),
            state: GameState::Playing,
            start_time: Time::None,
            turns: 0,
        }
    }

    // Getters
    pub fn width(&self)      -> usize { self.width }
    pub fn height(&self)     -> usize { self.height }
    pub fn bomb_count(&self) -> usize { self.bomb_count }

    pub fn board(&self) -> &Vec<Tile>  { &self.board }
    pub fn bombs(&self) -> &Vec<usize> { &self.bombs }

    pub fn state(&self)      -> GameState  { self.state }
    pub fn start_time(&self) -> &Time      { &self.start_time }
    pub fn turns(&self)      -> usize      { self.turns }

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
    pub fn dig(&mut self, index: usize) -> bool {
        if !self.diggable(index) {
            return false;
        }
        if self.turns == 0 {
            self.populate_board(index);
            self.start_time = Time::start();
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
                    self.board[tile_index] = Tile::Numbered(neighbouring_bombs);
                } else {
                    self.board[tile_index] = Tile::Dug;
                    neighbours.extend(valid_neighbours);
                }
            }
            if neighbours.is_empty() {
                break;
            }
            // Remove all duplicates and non-diggable tiles
            neighbours.sort_unstable();
            neighbours.dedup();
            neighbours.retain(|n_i| self.board[*n_i] == Tile::Unopened);
            // Make tiles to dig for the next iteration the neighbours we found this time. this also clears neighbours
            tiles_to_dig = std::mem::take(&mut neighbours);
        }
        // For each diggable tile, see if there's a bomb under it. If there are any without bombs under them, the game hasn't been won.
        let game_won = !self.board
            .iter().enumerate()
            .filter(|&(_, t)| *t == Tile::Flag || *t == Tile::Unopened)
            .map(|(i, _)| self.bombs().contains(&i))
            .any(|has_bomb| !has_bomb);
        
        if game_won {
            self.state = GameState::Win;
            self.start_time.freeze();
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
                if *tile == Tile::Flag {
                    *tile = Tile::Unopened;
                    return true;
                }
            }
            false => {
                if *tile == Tile::Unopened {
                    *tile = Tile::Flag;
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
