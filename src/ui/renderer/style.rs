use std::collections::HashMap;

use macroquad::{color::Color, color_u8, math::{vec2, Rect, Vec2}, texture::Texture2D};

use crate::ui::state::ButtonState;

pub const fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { x, y, w, h }
}

#[derive(Clone, Copy)]
pub struct Nineslice {
    pub rect: Rect,
    pub padding: f32
}

impl Nineslice {
    pub fn new(x: f32, y: f32, w: f32, h: f32, padding: f32) -> Nineslice {
        Nineslice { rect: Rect { x, y, w, h }, padding }
    }
}

pub const SPRITESHEET:     &[u8] = include_bytes!("../../../resources/spritesheet.png");
pub const WIN_SOUND:       &[u8] = include_bytes!("../../../resources/congrats.ogg");
pub const EXPLOSION_SOUND: &[u8] = include_bytes!("../../../resources/explosion.ogg");

pub const SHADOW: Color = color_u8!(0, 0, 0, 128);

pub const STATUS_V_PAD: f32 = 3.0;

pub const FACE_BUTTON_SIZE: Vec2 = vec2(19.0, 19.0);
pub const FACE_SIZE:        Vec2 = vec2(17.0, 17.0);
pub const FACE_OFFSET:      Vec2 = vec2( 1.0,  1.0);

pub const BOMB_COUNTER_DIGIT_SIZE:   Vec2 = vec2(8.0, 14.0);
pub const BOMB_COUNTER_DIGIT_OFFSET: Vec2 = vec2(3.0,  2.0);
pub const BOMB_COUNTER_DIGIT_GAP:    f32 = 2.0;
pub const BOMB_COUNTER_HEIGHT:       f32 = BOMB_COUNTER_DIGIT_SIZE.y + 4.0;

pub fn bomb_counter_size(digits: u32) -> Vec2 {
    Vec2::new(
        (BOMB_COUNTER_DIGIT_SIZE.x + BOMB_COUNTER_DIGIT_GAP) * digits as f32 + 4.0,
        BOMB_COUNTER_HEIGHT,
    )
}

pub enum CounterDigit {
    Digit(u32),
    Empty,
    Dash,
}
pub fn bomb_counter_digit(digit: CounterDigit) -> Rect {
    let along = match digit {
        CounterDigit::Digit(d) => d.min(9),
        CounterDigit::Empty => 10,
        CounterDigit::Dash  => 11,
    };
    Rect::new(
        BOMB_COUNTER_DIGIT_SIZE.x * along as f32,
        36.0,
        BOMB_COUNTER_DIGIT_SIZE.x,
        BOMB_COUNTER_DIGIT_SIZE.y,
    )
}

pub const TIMER_SIZE:            Vec2     = vec2(21.0, 9.0);
pub const TIMER_DIGIT_Y:         f32      = 2.0;
pub const TIMER_DIGIT_POSITIONS: [f32; 4] = [2.0, 6.0, 12.0, 16.0];
pub const TIMER_COLON_POSITION:  f32      = 10.0;

pub fn timer_digit(digit: Option<u32>) -> Rect {
    let d = digit.unwrap_or(10).min(10) as f32;
    rect(2.0 + d * 3.0, 50.0, 3.0, 5.0)
}
pub const fn timer_colon(lit: bool) -> Rect {
    let x = if lit { 1.0 } else { 0.0 };
    rect(x, 50.0, 1.0, 5.0)
}

pub const MINEFIELD_TILE_SIZE: u32  = 9;

#[derive(PartialEq, Eq, Hash)]
pub enum Theme { Light, Dark }
impl Theme {
    pub fn y(&self) -> f32 {match self {
        Theme::Light =>  0.0,
        Theme::Dark  => 18.0,
    }}
}
#[derive(PartialEq, Eq)]
pub enum FaceType { Fox, Nerd }
pub enum Face { Idle, Scared, Win, Lose }

pub struct Style {
    texture:   Texture2D,
    colors:    HashMap<Theme, Colors>,
    theme:     Theme,
    face_type: FaceType,
}

pub struct Colors {
    pub text:             Color,
    pub text_disabled:    Color,
    pub text_url:         Color,
    pub text_popup_title: Color,
    pub menubar_idle:     (Color, Color, Color),
    pub menubar_active:   (Color, Color, Color),
}

impl Style {
    pub fn new(texture: Texture2D) -> Style {
        let mut colors = HashMap::new();

        let data = texture.get_texture_data();
        let pixel = |x: u32, y: u32| {
            data.get_pixel(x, y)
        };
        for theme in [Theme::Light, Theme::Dark] {
            let y = theme.y() as u32;
            colors.insert(theme, Colors {
                text:             pixel(86, 11+y),
                text_disabled:    pixel(87, 11+y),
                text_url:         pixel(88, 11+y),
                text_popup_title: pixel(89, 11+y),
                menubar_idle:     (pixel(86, 12+y), pixel(87, 12+y), pixel(88, 12+y)),
                menubar_active:   (pixel(86, 13+y), pixel(87, 13+y), pixel(88, 13+y)),
            });
        }

        Style {
            texture,
            colors,
            theme:     Theme::Light,
            face_type: FaceType::Fox,
        }
    }
    
    pub fn texture(&self) -> Texture2D { self.texture.clone() }

    pub fn theme(&self)         -> &Theme { &self.theme }
    pub fn set_theme(&mut self, t: Theme) { self.theme = t }
    pub fn face_type(&self)        -> &FaceType  { &self.face_type }
    pub fn set_face_type(&mut self, f: FaceType) { self.face_type = f }

    pub fn background(&self) -> Nineslice { Nineslice::new(92.0, self.theme.y(), 3.0, 3.0, 1.0) }

    pub fn text(&self)          -> Color { self.colors.get(&self.theme).map(|c| c.text)         .unwrap_or_default() }
    pub fn text_disabled(&self) -> Color { self.colors.get(&self.theme).map(|c| c.text_disabled).unwrap_or_default() }
    pub fn text_url(&self)      -> Color { self.colors.get(&self.theme).map(|c| c.text_url)     .unwrap_or_default() }

    pub fn menubar(&self, active: bool) -> (Color, Color, Color) {
        self.colors.get(&self.theme)
            .map(|c| if active {c.menubar_active} else {c.menubar_idle})
            .unwrap_or_default()
    }

    pub fn dropdown_background(&self) -> Nineslice { Nineslice::new(92.0, self.theme.y(), 3.0, 3.0, 1.0) }
    pub fn dropdown_separator(&self)  -> Rect      { rect(94.0, 12.0 + self.theme.y(), 1.0, 2.0) }

    pub fn button(&self, button_state: &ButtonState) -> (Vec2, Nineslice, Color) {
        let (offset, y, text_col) = match button_state {
            ButtonState::Disabled => (0.0, 6.0, self.text_disabled()),
            ButtonState::Idle     |
            ButtonState::Hovered  => (0.0, 0.0, self.text()),
            _                     => (1.0, 3.0, self.text()),
        };
        (Vec2::splat(offset), Nineslice::new(92.0, y + self.theme.y(), 3.0, 3.0, 1.0), text_col)
    }

    pub fn text_input(&self) -> Nineslice { Nineslice::new(92.0, 15.0 + self.theme.y(), 3.0, 3.0, 1.0) }

    pub fn popup_title_text(&self) -> Color { self.colors.get(&self.theme).map(|c| c.text_popup_title).unwrap_or_default() }
    pub fn popup_title(&self) -> Nineslice { Nineslice::new(86.0, 15.0 + self.theme.y(), 3.0, 3.0, 1.0) }
    pub fn popup_body(&self)  -> Nineslice { Nineslice::new(89.0, 15.0 + self.theme.y(), 3.0, 3.0, 1.0) }
    pub fn popup_close(&self, hovered: bool) -> Rect {
        let y = if hovered { 7.0 } else { 0.0 };
        rect(95.0, y + self.theme.y(), 7.0, 7.0)
    }

    pub fn bomb_counter_background(&self) -> Nineslice { Nineslice::new(92.0, 9.0 + self.theme.y(), 3.0, 3.0, 1.0) }
    pub fn timer_background(&self)        -> Nineslice { Nineslice::new(92.0, 9.0 + self.theme.y(), 3.0, 3.0, 1.0) }

    pub fn minefield_tile(&self, id: u32) -> Rect {
        Rect {
            x: ((id % 9) * MINEFIELD_TILE_SIZE) as f32,
            y: ((id / 9) * MINEFIELD_TILE_SIZE) as f32 + self.theme.y(),
            w: MINEFIELD_TILE_SIZE as f32,
            h: MINEFIELD_TILE_SIZE as f32,
        }
    }
    pub fn minefield_border(&self)   -> Nineslice { Nineslice::new(81.0, 11.0 + self.theme.y(), 5.0, 5.0, 2.0) }
    pub fn minefield_selector(&self) -> Rect { rect(81.0, self.theme.y(), 11.0, 11.0) }

    fn face_y(&self) -> f32 {
        55.0 + match self.face_type {
            FaceType::Fox  =>  0.0,
            FaceType::Nerd => 17.0,
        }
    }

    pub fn face(&self, face: Face, blinking: bool, angry: bool) -> (Rect, Option<Rect>) {
        let size = FACE_SIZE;
        let face_rect = |index: usize| {
            Rect::new(index as f32 * size.x, self.face_y(), size.x, size.y)
        };
        
        let eye_index = match (&face, blinking, angry) {
            (Face::Win | Face::Lose, ..) => None,
            (_, false, false) => None,    // Neither
            (_, true,  false) => Some(4), // Blinking
            (_, false, true)  => Some(5), // Angry
            (_, true,  true)  => Some(6), // Both
        };
        let face_index = face as usize;

        (face_rect(face_index), eye_index.map(|e| face_rect(e)))
    }
}