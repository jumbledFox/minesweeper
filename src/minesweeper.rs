use ggez::glam::Vec2;
use rand::prelude::*;

#[derive(PartialEq)]
pub enum TileType {
    Covered,
    Dug,
    Flag,
}

impl rand::distributions::Distribution<TileType> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0..=2) { // rand 0.8
            0 => TileType::Covered,
            1 => TileType::Dug,
            _ => TileType::Flag,
        }
    }
}

pub struct Minesweeper {
    pub width: usize, pub height: usize,
    pub board: Vec<TileType>,
    pub neighbour_count: Vec<u8>,
    pub bombs: Vec<usize>,
    pub selected_tile: Option<usize>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize) -> Minesweeper {
        let size = width*height;
        let mut board = Vec::with_capacity(size);
        for _ in 0..size {
            board.push(rand::random());
        }
        let mut neighbour_count = Vec::with_capacity(size);
        for _ in 0..size {
            neighbour_count.push(thread_rng().gen_range(0..=8));
        }
        Minesweeper { width, height, board, neighbour_count, bombs: vec![0, 1, width, 2*width + 3], selected_tile: Some(width + 1) }
    }

    pub fn get_minefield_canvas_size(&self) -> (u32, u32) {
        (self.width as u32 * 9 + 4, self.height as u32 * 9 + 4)
    }

    pub fn index_to_draw_coord(&self, index: usize) -> Vec2 {
        Vec2::new(
            ((index % self.width) * 9) as f32,
            ((index / self.width) * 9) as f32,
        )
    }
}