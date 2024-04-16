use std::{cell::RefCell, rc::Rc};

use macroquad::{math::{vec2, Rect, Vec2}, texture::Image};

use crate::{minesweeper::Minesweeper, ui::DrawShape};

use super::{spritesheet, UIState};

// GRRRRRRRRR
pub fn bomb_counter(ui: &mut UIState, middle_pos: Vec2, size: Vec2, digit_count: u32, value: usize) {
    // ui.draw_queue().push(DrawShape::Image { x: middle_x - 4.5, y, source: spritesheet::minefield_tile(value) });
    ui.draw_queue().push(DrawShape::image_rect(Rect::new(middle_pos.x - size.x/2.0, middle_pos.y - size.y/2.0, size.x, size.y), spritesheet::minefield_tile(value)));
    // const DIGIT_PAD: f32 = 2.0;

    // let width = DIGIT_PAD + (ui.style().counter_digit_0.w + DIGIT_PAD) * digit_count as f32;
    // // let height = ui.style().counter_digit_0.h + 4.0;
    // let x = f32::max(0.0, middle_x - width/2.0);
    
    // let padding_amount: usize = match usize::try_from(digit_count.saturating_sub(value.checked_ilog10().unwrap_or(0) + 1)) {
    //     Ok(p) => p,
    //     Err(..) => return,
    // };
    // // Make a vector of all of the digits, 10 representing a blank segment
    // let mut digits = vec![10 ;padding_amount];
    // digits.append(&mut value.to_string().chars().filter_map(|d| d.to_digit(10)).collect());

    // // Draw them
    // let mut digit_x = x + DIGIT_PAD;
    // for d in digits {
    //     let source = get_counter_digit_source(&ui, d);
    //     ui.draw_queue().push(DrawShape::image(digit_x, y + 2.0, source));
    //     digit_x += source.w + DIGIT_PAD;
    // }

    // // Draw the background
    // let dest = Rect::new(x, y, width, height);
    // let source = ui.style().counter_bg_source;
    // ui.draw_queue().push(super::DrawShape::nineslice(dest, source));
}

// fn get_counter_digit_source(ui: &UIState, digit: u32) -> Rect {
//     let digit_size = ui.style().counter_digit_0.size();
//     match digit {
//         0..=9 => ui.style().counter_digit_0.offset(vec2(digit_size.x*(digit%5) as f32, digit_size.y*(digit/5) as f32)),
//         _ => Rect::default(),
//     }
//     // Rect::new(36.0, 0.0, 8.0, 14.0)
// }

pub fn timer(ui: &mut UIState, ) {
    
}

pub struct MinesweeperUI {
    ui: Rc<RefCell<UIState>>,
    game: Minesweeper,
}

impl MinesweeperUI {
    pub fn new(ui: Rc<RefCell<UIState>>) -> MinesweeperUI {
        MinesweeperUI { ui, game: Minesweeper::new(crate::minesweeper::Difficulty::Normal) }
    }

    // Renders the minefield ui element and returns how large it is
    pub fn minefield(&mut self, middle_x: f32, y: f32) -> Vec2 {
        let size = vec2((self.game.width()*9) as f32, (self.game.height()*9) as f32) + 4.0;
        let mut ui = self.ui.borrow_mut();
        ui.draw_queue().push(DrawShape::nineslice(
            Rect::new(middle_x - size.x/2.0, y - size.y/2.0, size.x, size.y),
            spritesheet::MINEFIELD_BORDER
        ));
        size
    }
}

// Draw the minefield, does the logic, and returns how big it was (?)
// TODO: Maybe make a minefield struct that handles all of the does this kind of stuff
pub fn minefield(ui: &mut UIState, middle_x: f32, y: f32) -> Vec2 {
    let size = vec2(9.0 * 15.0 + 4.0, 9.0 * 13.0 + 4.0);
    ui.draw_queue().push(DrawShape::nineslice(
        Rect::new(middle_x - size.x/2.0, y - size.y/2.0, size.x, size.y),
        spritesheet::MINEFIELD_BORDER
    ));
    size
}