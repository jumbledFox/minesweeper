use macroquad::{math::{vec2, Rect, Vec2}, rand::gen_range, time::get_frame_time};

use crate::{minesweeper::{GameState, Minesweeper}, ui::{elements::{aligned_rect, Align}, renderer::{style::{bomb_counter_digit, bomb_counter_size, timer_colon, timer_digit, CounterDigit, Eyes, Face, BOMB_COUNTER_DIGIT_GAP, BOMB_COUNTER_DIGIT_OFFSET, BOMB_COUNTER_HEIGHT, FACE_BUTTON_SIZE, FACE_OFFSET, STATUS_V_PAD, TIMER_COLON_POSITION, TIMER_DIGIT_POSITIONS, TIMER_DIGIT_Y, TIMER_SIZE}, DrawShape, Renderer}, state::{ButtonState, State}}};

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
    pub fn height(&self) -> f32 {
        let (button_height, bomb_counter_height, timer_height) = (
            FACE_BUTTON_SIZE.y,
            BOMB_COUNTER_HEIGHT,
            TIMER_SIZE.y,
        );
        let max =  button_height
            .max(bomb_counter_height)
            .max(timer_height);
        max + 2.0 * STATUS_V_PAD
    }

    pub fn min_size(&self) -> Vec2 {
        // TODO: Maybe make this change depending on the number of digits in the bomb counter
        vec2(75.0, self.height())
    }

    pub fn reset_blink_timer(&mut self) {
        self.blink_timer = 0.0;
        self.blink_next = gen_range(1.0, 10.0);
    }

    // Returns whether a new game was requested
    pub fn update(&mut self, area: Rect, minefield_active: bool, game: &Minesweeper, timer: Option<f32>, state: &mut State, renderer: &mut Renderer) -> bool {
        // renderer.draw(DrawShape::rect(area, macroquad::color::Color::from_rgba(0, 0, 255, 128)));

        let height = self.height();
        let y = Align::Mid(area.y + height / 2.0);

        let new_game = self.button(   Align::Mid(area.x + area.w / 2.0),         y, minefield_active, game, state, renderer);
        StatusBar::bomb_counter(game, Align::Mid(area.x + area.w * (1.0 / 6.0)), y, renderer);
        StatusBar::timer(timer,       Align::Mid(area.x + area.w * (5.0 / 6.0)), y, renderer);
        new_game
    }

    fn button(&mut self, x: Align, y: Align, eek: bool, game: &Minesweeper, state: &mut State, renderer: &mut Renderer) -> bool {
        let size = FACE_BUTTON_SIZE;
        let rect = aligned_rect(x, y, size.x, size.y);
        let button_state = state.button_state(0xB00B135, state.mouse_in_rect(rect), false, true);

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
        
        // TODO: Make it so when you leave the game idle for too long the fox goes to sleep
        // Blinking
        self.blink_timer += get_frame_time();
        let blinking = self.blink_timer > self.blink_next - BLINK_DURATION;
        if blinking {
            if self.blink_timer > self.blink_next {
                self.reset_blink_timer();
            }
        }
        
        let (offset, source, _) = renderer.style().button(&button_state);
        let rect = rect.offset(offset);
        
        let face = match (game.state(), eek) {
            (GameState::Lose, _) => Face::Lose,
            (GameState::Win,  _) => Face::Win,
            (_, true)            => Face::Scared,
            (_, false)           => Face::Idle,
        };
        let button_held = matches!(button_state, ButtonState::Clicked | ButtonState::Held | ButtonState::Released);
        let eyes = match (blinking || button_held, self.angry) {
            (true, _) => Eyes::Blink,
            (_, true) => Eyes::Angry,
            _         => Eyes::Open,
        };
        let (face, eyes) = renderer.style().face(face, eyes);

        // Drawing
        if let Some(eyes) = eyes {
            renderer.draw(DrawShape::image(rect.x + FACE_OFFSET.x, rect.y + FACE_OFFSET.y, eyes, None));
        }
        renderer.draw(DrawShape::image(rect.x + FACE_OFFSET.x, rect.y + FACE_OFFSET.y, face, None));
        renderer.draw(DrawShape::nineslice(rect, source));
        button_state.released()
    }

    fn bomb_counter(game: &Minesweeper, x: Align, y: Align, renderer: &mut Renderer) {
        let value = game.flags_left();
        // Calculate the minimum number of digits needed to display the bomb count (always being 2 or larger for style purposes: 3)
        // TODO: Maybe this can be done better
        let digits = ((f32::log10(game.bomb_count() as f32) + 1.0).floor() as u32).max(2);

        let size = bomb_counter_size(digits);
        let rect = aligned_rect(x, y, size.x, size.y);

        let (gap, offset, background) = (
            BOMB_COUNTER_DIGIT_GAP,
            BOMB_COUNTER_DIGIT_OFFSET,
            renderer.style().bomb_counter_background(),
        );

        let draw_shapes = (0..digits)
            // Work out the place value of the current digit
            .map(|i| (i, 10u32.saturating_pow(i as u32)))
            .map(|(i, power_of_ten)| (i,
                match value {
                    // If value is none, draw dashes for every character
                    None => CounterDigit::Dash,
                    // Otherwise draw a number! This doesn't draw leading zeros, however always renders the first digit, even if it's a zero!
                    Some(v) if power_of_ten <= v as u32 || i == 0 => CounterDigit::Digit((v as u32 / power_of_ten) % 10),
                    _    => CounterDigit::Empty,
                }
            ))
            .map(|(i, digit)| (i, bomb_counter_digit(digit)))
            // Render the digits in reverse order so they appear the right way around
            .map(|(i, digit_rect)| DrawShape::image(
                rect.x + offset.x + (digit_rect.w + gap) * (digits - i - 1) as f32,
                rect.y + offset.y,
                digit_rect,
                None
            ))
            .chain(std::iter::once(DrawShape::nineslice(rect, background)));

        renderer.draw_iter(draw_shapes);
    }
    
    fn timer(timer: Option<f32>, x: Align, y: Align, renderer: &mut Renderer) {
        let size = TIMER_SIZE;
        let rect = aligned_rect(x, y, size.x, size.y);

        let (digits, colon): ([Option<u32>; 4], bool) = if let Some(time) = timer {
            let seconds = (time as usize).min(60*100-1).try_into().unwrap_or(u32::MAX);
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

        let (digits_x, colon_x) = (TIMER_DIGIT_POSITIONS, TIMER_COLON_POSITION);

        let draw_shapes = digits.iter()
            .zip(digits_x)
            .map(|(&digit, along)| DrawShape::image(rect.x + along,    rect.y + TIMER_DIGIT_Y, timer_digit(digit), None))
            .chain(std::iter::once(DrawShape::image(rect.x + colon_x,  rect.y + TIMER_DIGIT_Y, timer_colon(colon), None)))
            .chain(std::iter::once(DrawShape::nineslice(rect, renderer.style().timer_background())));

        renderer.draw_iter(draw_shapes);
    }
}