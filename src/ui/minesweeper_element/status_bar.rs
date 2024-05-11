use macroquad::{rand::gen_range, time::get_frame_time};

use crate::{minesweeper::Minesweeper, ui::{elements::{aligned_rect, Align}, renderer::{DrawShape, Renderer}, spritesheet, state::{ButtonState, State}}};

#[derive(Default)]
pub struct StatusBar {
    blink_timer: f32,
    blink_next: f32,
}

impl StatusBar {
    pub fn update(&mut self, game: &Minesweeper, timer: Option<f32>, state: &mut State, renderer: &mut Renderer) {
        // StatusBar::bomb_counter(game, x, y, renderer);
        // StatusBar::timer(timer, x, y, renderer);
    }

    fn button(&mut self, x: Align, y: Align, state: &mut State, renderer: &mut Renderer) {
        let rect = aligned_rect(x, y, 19.0, 19.0);
        let button_state = state.button_state(0xB00B135, state.mouse_in_rect(rect), false, true);

        let (offset, source) = match button_state {
            ButtonState::Held | ButtonState::Clicked => (1.0, spritesheet::BUTTON_DOWN),
            _                                        => (0.0, spritesheet::BUTTON_IDLE),
        };

        self.blink_timer += get_frame_time();
        if self.blink_timer > self.blink_next {
            self.blink_timer = 0.0;
            self.blink_next = gen_range(1.0, 10.0);
        }
    }

    fn bomb_counter(game: &Minesweeper, x: Align, y: Align, renderer: &mut Renderer) {
        let value = game.flags_left();
        // Calculate the minimum number of digits needed to display the bomb count (always being 2 or larger for style purposes: 3)
        let digits = ((f32::log10(game.bomb_count() as f32) + 1.0).floor() as usize).max(2);

        let size = spritesheet::counter_size(digits);
        let rect = aligned_rect(x, y, size.x, size.y);

        let draw_shapes = (0..digits)
            // Work out the place value of the current digit
            .map(|i| (i, 10usize.saturating_pow(i as u32)))
            .map(|(i, power_of_ten)| (i,
                match value {
                    // If value is none, draw dashes for every character
                    None => spritesheet::CounterDigit::Dash,
                    // Otherwise draw a number! This doesn't draw leading zeros, however always renders the first digit, even if it's a zero!
                    Some(v) if power_of_ten <= v || i == 0 => spritesheet::CounterDigit::Digit((v / power_of_ten) % 10),
                    _ => spritesheet::CounterDigit::Empty,
                }
            ))
            .map(|(i, digit)| (i, spritesheet::counter_digit(digit)))
            // Render the digits in reverse order so they appear the right way around
            .map(|(i, digit_rect)| DrawShape::image(rect.x + 3.0 + (digit_rect.w + 2.0) * (digits - i - 1) as f32, rect.y + 2.0, digit_rect, None))
            // Last but not least draw the background
            .chain(std::iter::once(DrawShape::nineslice(rect, spritesheet::TIMER_BACKGROUND)));
        renderer.draw_iter(draw_shapes);
    }
    
    fn timer(timer: Option<f32>, x: Align, y: Align, renderer: &mut Renderer) {
        let size = spritesheet::TIMER_SIZE;
        let rect = aligned_rect(x, y, size.x, size.y);

        let (digits, colon_lit): ([Option<usize>; 4], bool) = if let Some(time) = timer {
            let seconds = (time as usize).min(60*100-1).try_into().unwrap_or(usize::MAX);
            let digits = [
                // Don't display the last digit as a 0 
                if seconds < 60*10 { None } else { Some((seconds / 60) / 10) }, // Tens of minutes
                Some((seconds / 60) % 10),                                      // Minutes
                Some((seconds % 60) / 10),                                      // Tens
                Some(seconds % 10),                                             // Units 
            ];
            // Originally, the colon flashed every half-second, but I found it a bit distracting :/
            (digits, true)
        } else {
            ([None; 4], false)
        };

        let draw_shapes = digits.iter()
            .zip([2.0, 6.0, 12.0, 16.0])
            .map(|(&digit, along)| DrawShape::image(rect.x + along, rect.y + 2.0, spritesheet::timer_digit(digit), None))
            // Draw the colon and the background
            .chain(std::iter::once(DrawShape::image(rect.x + 10.0, rect.y + 2.0, spritesheet::timer_colon(colon_lit), None)))
            .chain(std::iter::once(DrawShape::nineslice(rect, spritesheet::TIMER_BACKGROUND)));
        renderer.draw_iter(draw_shapes);
    }
}