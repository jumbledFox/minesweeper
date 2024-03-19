use std::time::Instant;
use rand::seq::SliceRandom;

const MAX_WIDTH : usize = 200;
const MAX_HEIGHT: usize = 100;

#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy, Normal, Hard,
    Custom{width: usize, height: usize, bomb_count: usize},
}

#[derive(PartialEq)]
pub enum GameState {
    Prelude, Playing, Win, Lose
}
#[derive(PartialEq, Clone)]
pub enum TileType {
    Unopened, Dug, Flag,
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
        let (width, height, bomb_count) = difficulty_values(difficulty);
        let size = width*height;
        
        let board = vec![TileType::Unopened; size];

        // 'bombs' and 'neighbour_count' are empty for now
        // They're only populated after the first move to make sure the 3*3 area at the first dig doesn't have any bombs
        let bombs = vec![0; bomb_count];
        let neighbour_count = vec![0; size];

        Minesweeper {
            width, height, bomb_count,
            board, bombs, neighbour_count,
            state: GameState::Prelude, start_time: Instant::now(),
        }
    }

    // Populates the bombs and neighbour_count vecs with valid values, making sure there are no bombs in the 3x3 area centered at safe_index
    fn populate_board(&mut self, safe_index: usize) {
        // Find out all of the possible positions for bombs, taking into account safe_index
        let mut possible_positions: Vec<usize> = (0..self.board.len())
            .filter(|&i| i != safe_index)
            .collect();
        // Shuffle the positions
        possible_positions.shuffle(&mut rand::thread_rng());

        let bomb_count = self.bombs.len();
        // Logically, bomb_count < possible_positions.len() is always true, so we don't have to worry about a possible panic here
        self.bombs.copy_from_slice(&possible_positions[..bomb_count]);
        // Work out all of the neighbour_counts
        for (tile_index, tile) in self.neighbour_count.iter_mut().enumerate() {
            *tile = 0;
            // For each of the neighbours
            for n in [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)] {
                // Get the neighbour index (if it exists), if there's a bomb there add one to the number!
                match get_index_from_offset(tile_index, n.0, n.1, self.width, self.height) {
                    Some(n) if self.bombs.contains(&n)=> *tile += 1,
                    _ => continue,
                };
            }
        }
    }

    // TODO: this function might be lame
    pub fn playing_state(&self) -> bool {
        match self.state {
            GameState::Prelude | GameState::Playing => true,
            _ => false,
        }
    }

    // Digs at a checked position, returns true if something changed
    pub fn dig(&mut self, index: usize) -> bool {
        // Get the tile from the board, making sure it's valid
        let tile = match self.board.get_mut(index) {
            None => { println!("index invalid!! tried to dig at {index}!! wtf?!?!?!?!"); return false; }
            Some(t) => t,
        };
        if *tile != TileType::Unopened { return false; }
        // If this is the first tile being opened, generate bombs and stuff, and set the state to playing
        if self.state == GameState::Prelude {
            self.populate_board(index);
            self.state = GameState::Playing;
        }
        // We can only dig when playing
        if self.state != GameState::Playing { return false; }
        // We dug a bomb! lose the game and return
        if self.bombs.contains(&index) {
            self.state = GameState::Lose;
            return true;
        }
        // Floodfill digging algorithm
        let mut tiles_to_dig = vec![index];
        let mut neighbours: Vec<usize> = vec![];
        
        for _ in 0..self.board.len() {
            // Loop through each of the tiles we want to dig up
            for &tile_index in &tiles_to_dig {
                // Dig up the tile
                match self.board.get_mut(tile_index) {
                    None => continue,
                    Some(t) if *t != TileType::Unopened => continue,
                    Some(t) => *t = TileType::Dug,
                };
                // Check each neighbour, if it's 
                for neighbour in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    if let Some(n) = get_index_from_offset(tile_index, neighbour.0, neighbour.1, self.width, self.height) {
                        
                    }
                }
                // Look over each neighbour
                // If the index is valid
                // If the tile 

                // Add all of the neighbours
                // up, down, left, right
                // let neighbour_offsets = [tile_index.checked_sub(self.width), tile_index.checked_add(self.width), tile_index.checked_sub(1), tile_index.checked_add(1)];
                // for (i, n) in neighbour_offsets.iter().enumerate() {
                //     if match i {
                //         /* Up    */ 0 => { tile_index % (self.width-1) == 0 }
                //         /* Down  */ 1 => { tile_index % (self.width-1) == 0 }
                //         /* Left  */ 2 => { tile_index }
                //         /* Right */ _ => { tile_index }
                //     } { continue; }
                // }
                // match tile_index.checked_sub(self.width) {
                //     Some(s) if s
                //     None => _
                // }
            }
            // If there aren't any neighbours, we've finished the flood fill
            if neighbours.is_empty() { break; }
            // Remove any duplicate agents
            neighbours.sort_unstable();
            neighbours.dedup();
            // Make the new tiles to dig the neighbours, and clear the neighbours
            std::mem::swap(&mut tiles_to_dig, &mut neighbours);
            neighbours.clear();
        }
        true
    }
    // Toggles a flag at a checked position, returns true if something changed
    pub fn set_flag(&mut self, erasing_flags: bool, index: usize) -> bool {
        // Get the tile from the board, making sure it's valid
        let tile = match self.board.get_mut(index) {
            None => { println!("index invalid!! tried to set flag at {index}!! wtf?!"); return false; }
            Some(t) => t,
        };
        // Add or remove a flag, depending on 'erasing_flags'
        match erasing_flags {
            true  => if *tile == TileType::Flag { *tile = TileType::Unopened; return true; },
            false => if *tile == TileType::Unopened { *tile = TileType::Flag; return true; },
        }
        return false;
    }
}

// Returns the width, heigth, and bomb_count given a difficulty. If the difficulty is custom, it's made to match the limits.
pub fn difficulty_values(difficulty: Difficulty) -> (usize, usize, usize) {
    match difficulty {
        Difficulty::Easy   => (10, 10,  9),
        Difficulty::Normal => (15, 13, 40),
        Difficulty::Hard   => (30, 16, 99),
        Difficulty::Custom{width, height, bomb_count} => {
            // Ensure the fields match the (somewhat arbitrary) limits.
            let (w, h) = (width.min(MAX_WIDTH), height.min(MAX_HEIGHT));
            let b = bomb_count.min((w-1)*(h-1));
            (w, h, b)
        },
    }
}

// Used for indexing the board, takes in an index and x and y offsets and calculates the new index, or None if it was invalid
pub fn get_index_from_offset(index: usize, x_offset: isize, y_offset: isize, width: usize, height: usize) -> Option<usize> {
    // Add on the offsets, if either of these are invalid then we know the new index is going to be invalid
    // The X offset is simple
    let x = match index.checked_add_signed(x_offset) {
        None => return None,
        Some(x) => x,
    };

    // However the Y offset needs to be multiplied by the width of the board so we move up / down
    // TODO: Make this better
    let mut y_offset_actual: isize = 0;
    for _ in 0..width {
        match y_offset_actual.checked_add(y_offset) {
            Some(yi) => y_offset_actual = yi,
            None => return None,
        }
    }
    let y = match index.checked_add_signed(y_offset_actual) {
        None => return None,
        Some(y) => y,
    };

    if x >= width || y >= height { return None }
    Some(y * width + x)
}
