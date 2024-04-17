use std::{cell::RefCell, rc::Rc};

use macroquad::math::{vec2, Rect, Vec2};

use crate::{minesweeper::{Difficulty, Minesweeper, TileType}, ui::DrawShape};

use super::{hash_string, spritesheet, ButtonState, UIState};

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
    selected_cell: Option<usize>,
}

impl MinesweeperUI {
    pub fn new(ui: Rc<RefCell<UIState>>, difficulty: Difficulty) -> MinesweeperUI {
        MinesweeperUI { ui, game: Minesweeper::new(difficulty), selected_cell: None }
    }

    // Renders the minefield ui element
    // TODO: Make this a bit neater...
    pub fn minefield(&mut self, middle_x: f32, y: f32, min_y: f32) {
        let size = vec2((self.game.width()*9) as f32, (self.game.height()*9) as f32);
        let pos = vec2(middle_x - size.x/2.0, min_y.max(y - size.y/2.0));
        let mut ui = self.ui.borrow_mut();

        let rect = Rect::new(pos.x, pos.y, size.x, size.y);
        let id = hash_string(&String::from("MINEFIELD!!! jumbledfox is so cool"));
        let mouse_in_rect = ui.mouse_in_rect(rect);
        let state = ui.button_state(id, mouse_in_rect, true);

        if mouse_in_rect && state != ButtonState::Idle {
            let selected_cell_pos = ((ui.mouse_pos - rect.point()) / 9.0).floor();
            // According to logic, this shouldn't need the min, but I like things to always be safe, just in case!
            let selected_cell = (selected_cell_pos.x as usize + selected_cell_pos.y as usize * self.game.width())
                .min(self.game.board().len().saturating_sub(1));
            self.selected_cell = Some(selected_cell);

            ui.draw_queue().push(DrawShape::image(selected_cell_pos.x * 9.0 + rect.x - 1.0, selected_cell_pos.y * 9.0 + rect.y - 1.0, spritesheet::MINEFIELD_SELECTED));
            if state == ButtonState::Held {
                ui.draw_queue().push(DrawShape::image(
                    rect.x + (selected_cell%self.game.width()) as f32 * 9.0,
                    rect.y + (selected_cell/self.game.width()) as f32 * 9.0,
                    spritesheet::minefield_tile(1)
                ));
            }
            if state == ButtonState::Released {
                self.game.dig(selected_cell);
            }
        } else {
            self.selected_cell = None;
        }

        // Draw the tiles
        // TODO: Make some kind of DrawShape::Minefield, as this shit is probably inefficient as hell
        for (i, tile) in self.game.board().iter().enumerate() {
            let t = match *tile {
                TileType::Unopened => 0,
                TileType::Dug => 2,
                TileType::Flag => {
                    ui.draw_queue().push(DrawShape::image(
                        rect.x + (i%self.game.width()) as f32 * 9.0,
                        rect.y + (i/self.game.width()) as f32 * 9.0,
                        spritesheet::minefield_tile(12)
                    ));
                    0
                },
                TileType::Numbered(n) => {
                    ui.draw_queue().push(DrawShape::image(
                        rect.x + (i%self.game.width()) as f32 * 9.0,
                        rect.y + (i/self.game.width()) as f32 * 9.0,
                        spritesheet::minefield_tile((3+n).into())
                    ));
                    2
                },
            };
            ui.draw_queue().push(DrawShape::image(
                rect.x + (i%self.game.width()) as f32 * 9.0,
                rect.y + (i/self.game.width()) as f32 * 9.0,
                spritesheet::minefield_tile(t)
            ));
        }

        // Draw the border
        ui.draw_queue().push(DrawShape::nineslice(Rect::new(pos.x - 2.0, pos.y - 2.0, size.x + 4.0, size.y + 4.0), spritesheet::MINEFIELD_BORDER));
    }
}