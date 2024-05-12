use macroquad::{math::{vec2, Rect, Vec2}, rand::gen_range, time::get_frame_time};

use crate::{minesweeper::{GameState, Minesweeper}, ui::{elements::{aligned_rect, Align}, renderer::{DrawShape, Renderer}, spritesheet::{self, FoxFace}, state::{ButtonState, State}}};

const BLINK_DURATION:        f32   = 0.1;
const SPAM_MAX_TIME:         f32   = 0.5;
const SPAM_ANGER_CLICKS:     usize = 10;
const SPAM_ANGER_RESET_TIME: f32   = 1.0;

#[derive(Default)]
pub struct StatusBar {
    blink_timer: f32,
    blink_next: f32,

    spam_timer:   f32,
    spam_counter: usize,
    angry:        bool,
}

impl StatusBar {
    pub fn min_size(&self) -> Vec2 {
        // TODO: Maybe make this change depending on the number of digits in the bomb counter
        vec2(75.0, 23.0)
    }

    pub fn reset_blink_timer(&mut self) {
        self.blink_timer = 0.0;
        self.blink_next = gen_range(1.0, 10.0);
    }

    // Returns whether a new game was requested
    pub fn update(&mut self, area: Rect, minefield_active: bool, game: &Minesweeper, timer: Option<f32>, state: &mut State, renderer: &mut Renderer) -> bool {
        let new_game = self.button(   Align::Mid(area.x + area.w / 2.0),         Align::Beg(area.y + 3.0), minefield_active, game, state, renderer);
        StatusBar::bomb_counter(game, Align::Mid(area.x + area.w * (1.0 / 6.0)), Align::Beg(area.y + 4.0), renderer);
        StatusBar::timer(timer,       Align::Mid(area.x + area.w * (5.0 / 6.0)), Align::Beg(area.y + 9.0), renderer);
        new_game
    }

    fn button(&mut self, x: Align, y: Align, eek: bool, game: &Minesweeper, state: &mut State, renderer: &mut Renderer) -> bool {
        let rect = aligned_rect(x, y, 19.0, 19.0);
        let button_state = state.button_state(0xB00B135, state.mouse_in_rect(rect), false, true);

        let (offset, source, eyes_closed) = match button_state {
            | ButtonState::Held 
            | ButtonState::Clicked
            | ButtonState::Released => (1.0, spritesheet::BUTTON_DOWN, true ),
            _                       => (0.0, spritesheet::BUTTON_IDLE, false),
        };

        let (face, can_change_eyes) = match (game.state(), eek) {
            (GameState::Lose, _) => (FoxFace::Dead,   false),
            (GameState::Win,  _) => (FoxFace::Happy,  false),
            (_, true )           => (FoxFace::Eek,    true ),
            (_, false)           => (FoxFace::Normal, true ),
        };

        let rect = rect.offset(Vec2::splat(offset));

        // Angring when you spam the button
        // If we're not angry, reset the counter and timer after a given amount of time not being clicked
        // If we're angry, go back to the inital state of not being angry if we've clicked the button a given amount of time after the initial angering
        if (!self.angry && self.spam_timer > SPAM_MAX_TIME) || (self.angry && button_state.clicked() && self.spam_timer > SPAM_ANGER_RESET_TIME) {
            self.spam_counter = 0;
            self.angry = false;
        }
        self.spam_timer += get_frame_time();
        // If it's been clicked, reset the timer and increment the counter
        if button_state.clicked() {
            self.spam_timer = 0.0;
            self.spam_counter += 1;
            self.angry = self.spam_counter >= SPAM_ANGER_CLICKS;
        }
        // TODO: do something with self.angry

        // TODO: Make it so when you leave the game idle for too long the fox goes to sleep
        // Blinking
        self.blink_timer += get_frame_time();
        if (self.blink_timer > self.blink_next - BLINK_DURATION || eyes_closed || self.angry) && can_change_eyes {
            renderer.draw(DrawShape::image(rect.x+3.0, rect.y+8.0, spritesheet::fox_face_blink(), None));
            if self.blink_timer > self.blink_next {
                self.reset_blink_timer();
            }
        }
        
        // Drawing the face and button
        renderer.draw(DrawShape::image(rect.x+1.0, rect.y+1.0, spritesheet::fox_face(face), None));
        renderer.draw(DrawShape::nineslice(rect, source));
        button_state.released()
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