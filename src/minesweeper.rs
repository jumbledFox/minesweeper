// A nice 'black box' game of minesweeper.
// Only handles minesweeper logic and is separate to any rendering or inputs and whatnot.

use rand::seq::SliceRandom;
use std::time::Instant;

const NEIGHBOUR_OFFSETS: &[(isize, isize)] = &[(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];
const MAX_WIDTH : usize = 200;
const MAX_HEIGHT: usize = 100;
const MIN_WIDTH : usize = 4;
const MIN_HEIGHT: usize = 4;

#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy, Normal, Hard,
    Custom {width: usize, height: usize, bomb_count: usize},
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    Playing, Win, Lose,
}
#[derive(PartialEq, Clone)]
pub enum TileType {
    Unopened,
    Flag,
    Dug,
    Numbered(u8),
}

// All of these are public as im unsure about 
pub struct Minesweeper {
    // Game logic
    width: usize,
    height: usize,
    bomb_count: usize,

    board: Vec<TileType>,
    bombs: Vec<usize>,

    state: GameState,
    start_time: Option<Instant>,
    turns: usize,
}

impl Minesweeper {
    pub fn new(difficulty: Difficulty) -> Minesweeper {
        let (width, height, bomb_count) = difficulty_values(difficulty);
        let size = width * height;

        let board = vec![TileType::Unopened; size];

        // 'bombs' is only populated after the first move (to make sure the 3*3 area at the first dig is safe), so for now it's filled with zero
        let bombs = vec![0; bomb_count];

        Minesweeper {
            width, height, bomb_count,
            board, bombs,
            state: GameState::Playing,
            start_time: None,
            turns: 0,
        }
    }

    // Getters
    pub fn width(&self)      -> usize { self.width  }
    pub fn height(&self)     -> usize { self.height }
    pub fn bomb_count(&self) -> usize { self.bomb_count }

    pub fn board(&self) -> &Vec<TileType> { &self.board }
    pub fn bombs(&self) -> &Vec<usize>    { &self.bombs }

    pub fn state(&self)      -> &GameState      { &self.state }
    pub fn start_time(&self) -> Option<Instant> { self.start_time }
    pub fn turns(&self)      -> usize           { self.turns }

    // Populates the bombs and neighbour_count vecs with valid values, making sure there are no bombs in the 3x3 area centered at safe_index
    fn populate_board(&mut self, safe_index: usize) {
        // List of positions a bomb can't be (the safe_index and all of it's neighbours)
        let safe_positions: Vec<usize> = NEIGHBOUR_OFFSETS
            .iter()
            .flat_map(|(x, y)| get_index_from_offset(safe_index, *x, *y, self.width, self.height))
            .chain(std::iter::once(safe_index))
            .collect();
        // Find out all of the possible positions for bombs
        let mut possible_positions: Vec<usize> = (0..self.board.len())
            .filter(|&i| !safe_positions.contains(&i))
            .collect();
        // Shuffle the positions
        possible_positions.shuffle(&mut rand::thread_rng());

        let bomb_count = self.bombs.len();
        // Logically, bomb_count is ALWAYS less than possible_positions.len(), so we don't have to worry about a possible panic here
        self.bombs.copy_from_slice(&possible_positions[..bomb_count]);
    }

    // Digs at a checked position, returns true if something changed
    pub fn dig(&mut self, index: usize) -> bool {
        // Get the tile from the board, making sure it's valid
        let tile = match self.board.get_mut(index) {
            None => {
                println!("index invalid!! tried to dig at {index}!! wtf?!?!?!?!");
                return false;
            }
            Some(t) => t,
        };
        // You can only dig unopened cells
        if *tile != TileType::Unopened { return false; }
        // If this is the first tile being opened, start the game and generate bombs
        if self.turns == 0 {
            self.populate_board(index);
            self.start_time = Some(Instant::now());
            self.state = GameState::Playing;
        }
        // If we're not playing, don't dig!
        if self.state != GameState::Playing { return false; }
        // Increase the turns
        self.turns += 1;

        // We dug a bomb! lose the game and return :c
        if self.bombs.contains(&index) {
            self.board[index] = TileType::Dug;
            self.state = GameState::Lose;
            return true;
        }
        
        // Floodfill digging algorithm
        let mut tiles_to_dig = vec![index];
        let mut neighbours: Vec<usize> = Vec::with_capacity(8);
        for _ in 0..self.board.len() {
            // Loop through each of the tiles we're gonna dig up
            for &tile_index in &tiles_to_dig {
                // Find out all of the valid neighbours of this tile
                let valid_neighbours = NEIGHBOUR_OFFSETS.iter().flat_map(|(x, y)| {
                    get_index_from_offset(tile_index, *x, *y, self.width, self.height)
                });
                // Work out how many neighbours of this tile are bombs
                let neighbouring_bombs: u8 = valid_neighbours
                    .clone()
                    .filter(|i| self.bombs.contains(&i))
                    .count() as u8;
                // If this tile has any bombs adjacent to it, make it numbered and don't flood any further from it.
                if neighbouring_bombs != 0 {
                    self.board[tile_index] = TileType::Numbered(neighbouring_bombs);
                    continue;
                }
                // Otherwise, dig it up normally
                self.board[tile_index] = TileType::Dug;
                // and add each valid neighbour of this tile to the neighbours vec!
                neighbours.extend(valid_neighbours);
            }
            // If there aren't any neighbours, we've finished the flood fill!
            if neighbours.is_empty() { break; }
            // Otherwise, Remove any duplicate neighbours, as well as any ones that aren't diggable
            neighbours.sort_unstable();
            neighbours.dedup();
            // I THINK doing this here is faster than filtering when defining valid_neighbours... (key word, think)
            neighbours.retain(|n_i| self.board[*n_i] == TileType::Unopened);
            // Make it so the new tiles we've got to dig are the neighbours, and clear the neighbours
            tiles_to_dig = std::mem::take(&mut neighbours);
        }
        true
    }

    // Toggles a flag at a checked position, returns true if something changed
    pub fn set_flag(&mut self, erasing_flags: bool, index: usize) -> bool {
        // If we're not playing, don't flag!
        if self.state != GameState::Playing { return false; }
        // Get the tile from the board, making sure it's valid
        let tile = match self.board.get_mut(index) {
            None => {
                println!("index invalid!! tried to set flag at {index}!! you dope!!");
                return false;
            }
            Some(t) => t,
        };
        // Add or remove a flag, depending on 'erasing_flags'
        match erasing_flags {
            true  => if *tile == TileType::Flag { *tile = TileType::Unopened; return true; }
            false => if *tile == TileType::Unopened { *tile = TileType::Flag; return true; }
        }
        return false;
    }
}

// Returns the width, heigth, and bomb_count given a difficulty. If the difficulty is custom, it's made to match the limits.
fn difficulty_values(difficulty: Difficulty) -> (usize, usize, usize) {
    match difficulty {
        Difficulty::Easy => (10, 10, 9),
        Difficulty::Normal => (15, 13, 40),
        Difficulty::Hard => (30, 16, 99),
        Difficulty::Custom {width, height, bomb_count} => {
            // Ensure the fields match the (somewhat arbitrary) limits.
            let (w, h) = (width.clamp(MIN_WIDTH, MAX_WIDTH), height.clamp(MIN_HEIGHT, MAX_HEIGHT));
            let b = bomb_count.min((w - 1) * (h - 1));
            (w, h, b)
        }
    }
}

// Used for indexing the board, takes in an index and x and y offsets and calculates the new index, or None if it was invalid
fn get_index_from_offset(index: usize, x_offset: isize, y_offset: isize, width: usize, height: usize) -> Option<usize> {
    // Get the coordinates of the new position, making sure it's valid
    let x = match (index % width).checked_add_signed(x_offset) {
        Some(x) if x < width => x,
        _ => return None,
    };
    let y = match (index / width).checked_add_signed(-y_offset) {
        Some(y) if y < height => y,
        _ => return None,
    };
    Some(y * width + x)
}
