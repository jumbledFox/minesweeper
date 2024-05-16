use macroquad::{math::{vec2, Rect, Vec2}, rand::gen_range, time::get_frame_time};

use crate::{minesweeper::{GameState, Minesweeper}, ui::{elements::{aligned_rect, Align}, renderer::{style::{ButtonFace, ButtonMouth, ButtonStateStyle, CounterDigit, FaceButtonStyle}, DrawShape, Renderer}, state::{ButtonState, State}}};

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
    pub fn height(&self, renderer: &Renderer) -> f32 {
        let (button_height, bomb_counter_height, timer_height) = (
            renderer.style().face_button_size().y,
            renderer.style().bomb_counter_height(),
            renderer.style().timer_size().y,
        );
        button_height.max(bomb_counter_height.max(timer_height)) + 2.0 * renderer.style().status_v_pad()
    }
    pub fn min_size(&self, renderer: &Renderer) -> Vec2 {
        // TODO: Maybe make this change depending on the number of digits in the bomb counter
        vec2(75.0, self.height(renderer))
    }

    pub fn reset_blink_timer(&mut self) {
        self.blink_timer = 0.0;
        self.blink_next = gen_range(1.0, 10.0);
    }

    // Returns whether a new game was requested
    pub fn update(&mut self, area: Rect, minefield_active: bool, game: &Minesweeper, timer: Option<f32>, state: &mut State, renderer: &mut Renderer) -> bool {
        // renderer.draw(DrawShape::rect(area, macroquad::color::Color::from_rgba(0, 0, 255, 128)));

        let height = self.height(&renderer);
        let y = Align::Mid(area.y + height / 2.0);

        let new_game = self.button(   Align::Mid(area.x + area.w / 2.0),         y, minefield_active, game, state, renderer);
        StatusBar::bomb_counter(game, Align::Mid(area.x + area.w * (1.0 / 6.0)), y, renderer);
        StatusBar::timer(timer,       Align::Mid(area.x + area.w * (5.0 / 6.0)), y, renderer);
        new_game
    }

    fn button(&mut self, x: Align, y: Align, eek: bool, game: &Minesweeper, state: &mut State, renderer: &mut Renderer) -> bool {
        let size = renderer.style().face_button_size();
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
        let button_held = matches!(button_state, ButtonState::Clicked | ButtonState::Held | ButtonState::Released);
        
        let face = match (game.state(), blinking || button_held, self.angry) {
            (GameState::Lose, ..) => ButtonFace::Dead,
            (GameState::Win,  ..) => ButtonFace::Happy,
            (_, true, _)          => ButtonFace::Blink,
            (_, _, true)          => ButtonFace::Angry,
            _                     => ButtonFace::Idle,
        };
        let mouth = match (&face, eek) {
            (ButtonFace::Dead, _) => ButtonMouth::Dead,
            (_, true)             => ButtonMouth::Open,
            _                     => ButtonMouth::Idle,
        };
        
        let FaceButtonStyle {
            button_style: ButtonStateStyle { source, source_offset, inner_offset, .. },
            face_offset, face, mouth_offset, mouth
        } = renderer.style().face_button(&button_state, face, mouth);

        // Drawing the face and button
        renderer.draw(DrawShape::image(rect.x+inner_offset.x+mouth_offset.x, rect.y+inner_offset.y+mouth_offset.y, mouth, None));
        renderer.draw(DrawShape::image(rect.x+inner_offset.x+face_offset.x,  rect.y+inner_offset.y+face_offset.y,  face, None));
        renderer.draw(DrawShape::nineslice(rect.offset(source_offset), source));
        button_state.released()
    }

    fn bomb_counter(game: &Minesweeper, x: Align, y: Align, renderer: &mut Renderer) {
        let value = game.flags_left();
        // Calculate the minimum number of digits needed to display the bomb count (always being 2 or larger for style purposes: 3)
        // TODO: Maybe this can be done better
        let digits = ((f32::log10(game.bomb_count() as f32) + 1.0).floor() as u32).max(2);

        let size = renderer.style().bomb_counter_size(digits);
        let rect = aligned_rect(x, y, size.x, size.y);

        let (gap, offset, background) = (
            renderer.style().bomb_counter_digit_gap(),
            renderer.style().bomb_counter_digit_offset(),
            renderer.style().bomb_counter_background(),
        );

        let draw_shapes: Vec<DrawShape> = (0..digits)
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
            .map(|(i, digit)| (i, renderer.style().bomb_counter_digit(digit)))
            // Render the digits in reverse order so they appear the right way around
            .map(|(i, digit_rect)| DrawShape::image(
                rect.x + offset.x + (digit_rect.w + gap) * (digits - i - 1) as f32,
                rect.y + offset.y,
                digit_rect,
                None
            ))
            .chain(std::iter::once(DrawShape::nineslice(rect, background)))
            .collect();
        renderer.draw_iter(draw_shapes.into_iter());
    }
    
    fn timer(timer: Option<f32>, x: Align, y: Align, renderer: &mut Renderer) {
        let size = renderer.style().timer_size();
        let rect = aligned_rect(x, y, size.x, size.y);

        let (digits, colon_lit): ([Option<u32>; 4], bool) = if let Some(time) = timer {
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

        let (digits_x, colon_x) = renderer.style().timer_digit_positions();

        let draw_shapes: Vec<DrawShape> = digits.iter()
            .zip(digits_x)
            .map(|(&digit, along)| DrawShape::image(rect.x + along,    rect.y + 2.0, renderer.style().timer_digit(digit),     None))
            .chain(std::iter::once(DrawShape::image(rect.x + colon_x,  rect.y + 2.0, renderer.style().timer_colon(colon_lit), None)))
            .chain(std::iter::once(DrawShape::nineslice(rect, renderer.style().timer_background())))
            .collect();
        renderer.draw_iter(draw_shapes.into_iter());
    }
}