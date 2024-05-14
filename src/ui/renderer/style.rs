use macroquad::{color::Color, math::{Rect, Vec2}, texture::Texture2D};

use crate::ui::state::ButtonState;

pub fn rect_from_pos_size(pos: Vec2, size: Vec2) -> Rect {
    Rect::new(pos.x, pos.y, size.x, size.y)
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

#[derive(Clone, Copy)]
pub struct MenuItemStyle {
    pub background: Color,
    pub text_col:   Color,
}
#[derive(Clone, Copy)]
pub struct DropdownStyle {
    pub background:     Color,
    pub text_col:       Color,
    pub other_text_col: Color,
}

pub struct Style {
    texture: Texture2D,
    // UI
    background:                 Nineslice,
    url_text:                   Color,
    shadow:                     Color,
    // TODO: shadow_offset? 
    // Buttons
    button_idle:                Nineslice,
    button_down:                Nineslice,
    button_disabled:            Nineslice,
    button_down_offset:         Vec2,
    button_text:                Color,
    button_disabled_text:       Color,
    // Menubar
    menu_item_idle:             MenuItemStyle,
    menu_item_active:           MenuItemStyle,
    dropdown_idle:              DropdownStyle,
    dropdown_active:            DropdownStyle,
    dropdown_background:        Nineslice,
    dropdown_separator:         Nineslice,
    // Popups
    popup_title:                Nineslice,
    // TODO: popup_title_height:         f32,
    popup_body:                 Nineslice,
    popup_title_text_col:       Color,
    popup_body_text_col:        Color,
    popup_close_idle:           Rect,
    popup_close_down:           Rect,
    // Minesweeper
    minefield_border:           Nineslice,
    minefield_tile_origin:      Vec2,
    minefield_tile_width:       u32,
    minefield_tile_height:      u32,
    minefield_tile_size:        Vec2,
    minefield_selector:         Rect,
    minefield_selector_offset:  Vec2,
    // Status bar
    status_v_pad:               f32,
    // Face button
    face_button_idle:           Nineslice,
    face_button_down:           Nineslice,
    face_button_down_offset:    Vec2,
    face_button_size:           Vec2,
    face_origin:                Vec2,
    face_size:                  Vec2,
    face_offset:                Vec2,
    mouth_origin:               Vec2,
    mouth_size:                 Vec2,
    mouth_offset:               Vec2,
    // Bomb counter
    bomb_counter_background:    Nineslice,
    bomb_counter_digit_origin:  Vec2,
    bomb_counter_digit_offset:  Vec2,
    bomb_counter_digit_size:    Vec2,
    bomb_counter_digit_gap:     f32,
    // Timer
    timer_background:           Nineslice,
    timer_digit_origin:         Vec2,
    timer_digit_offset:         Vec2,
    timer_digit_size:           Vec2,
    timer_digit_gap:            f32,
    timer_colon_width:          f32,
}

pub struct ButtonStyle {
    pub offset:   Vec2,
    pub source:   Nineslice,
    pub text_col: Color,
}
pub struct PopupStyle {
    pub title:          Nineslice,
    pub title_text_col: Color,
    pub body:           Nineslice,
    pub body_text_col:  Color,
}
pub struct FaceButtonStyle {
    pub offset:       Vec2,
    pub source:       Nineslice,
    pub face_offset:  Vec2,
    pub face:         Rect,
    pub mouth_offset: Vec2,
    pub mouth:        Rect,
}

pub enum CounterDigit {
    Empty,
    Dash,
    Digit(u32),
}

pub enum ButtonFace {
    Idle, Blink, Angry, Happy, Dead,
}
pub enum ButtonMouth {
    Idle, Open, Dead,
}

impl Style {
    pub fn texture(&self) -> Texture2D {
        self.texture.clone()
    }

    // UI
    pub fn background(&self) -> Nineslice { self.background }
    pub fn url_text(&self)   -> Color     { self.url_text }
    pub fn shadow(&self)     -> Color     { self.shadow }

    pub fn button_style(&self, button_state: &ButtonState) -> ButtonStyle {
        let (offset, source, text_col) = match *button_state {
            ButtonState::Disabled => (Vec2::ZERO,               self.button_disabled, self.button_disabled_text),
            ButtonState::Clicked  |
            ButtonState::Held     |
            ButtonState::Released => (self.button_down_offset,  self.button_down,     self.button_text),
            _                     => (Vec2::ZERO,               self.button_idle,     self.button_text),
        };
        ButtonStyle { offset, source, text_col }
    }

    // Menubar
    pub fn menu_item_style(&self, active: bool) -> MenuItemStyle {
        match active {
            false => self.menu_item_idle,
            true  => self.menu_item_active,
        }
    }
    pub fn dropdown_style(&self, active: bool) -> DropdownStyle {
        match active {
            false => self.dropdown_idle,
            true  => self.dropdown_active,
        }
    }
    pub fn dropdown_background(&self) -> Nineslice { self.dropdown_background }
    pub fn dropdown_separator(&self)  -> Nineslice { self.dropdown_separator }

    // Popups
    pub fn popup_style(&self) -> PopupStyle {
        PopupStyle {
            title:          self.popup_title,
            title_text_col: self.popup_title_text_col,
            body:           self.popup_body,
            body_text_col:  self.popup_body_text_col,
        }
    }

    pub fn popup_close(&self, hovered: bool) -> Rect {
        match hovered {
            false => self.popup_close_idle,
            true  => self.popup_close_down,
        }
    }

    // Minefield
    pub fn minefield_border(&self)      -> Nineslice { self.minefield_border }
    pub fn minefield_tile_width(&self)  -> u32  { self.minefield_tile_width }
    pub fn minefield_tile_height(&self) -> u32  { self.minefield_tile_height }
    pub fn minefield_tile_size(&self)   -> Vec2 { self.minefield_tile_size }

    pub fn minefield_tile(&self, id: u32) -> Rect {
        let pos = Vec2::new(
            (id % 9) as f32,
            (id / 9) as f32,
        ) * self.minefield_tile_size;

        rect_from_pos_size(pos, self.minefield_tile_size)
        .offset(self.minefield_tile_origin)
    }

    pub fn minefield_selector(&self)        -> Rect { self.minefield_selector }
    pub fn minefield_selector_offset(&self) -> Vec2 { self.minefield_selector_offset }

    // Status bar
    pub fn status_v_pad(&self) -> f32 { self.status_v_pad }
    // Face button
    pub fn face_button_size(&self) -> Vec2 { self.face_button_size }

    pub fn face_button(&self, button_state: &ButtonState, face: ButtonFace, mouth: ButtonMouth) -> FaceButtonStyle {
        let (offset, source) = match *button_state {
            ButtonState::Clicked  |
            ButtonState::Held     |
            ButtonState::Released => (self.face_button_down_offset, self.face_button_down),
            _                     => (Vec2::ZERO,                   self.face_button_idle),
        };
        FaceButtonStyle {
            offset,
            source,
            face_offset:  self.face_offset,
            face:         self.face(face),
            mouth_offset: self.mouth_offset,
            mouth:        self.mouth(mouth),
        }
    }
    
    pub fn face(&self, face: ButtonFace) -> Rect {
        let pos = Vec2::new(face as u32 as f32, 0.0) * self.face_size;
        rect_from_pos_size(pos, self.face_size)
        .offset(self.face_origin)
    }

    pub fn mouth(&self, mouth: ButtonMouth) -> Rect {
        let pos = Vec2::new(mouth as u32 as f32, 0.0) * self.mouth_size;
        rect_from_pos_size(pos, self.mouth_size)
        .offset(self.mouth_origin)
    }
    
    // Bomb counter
    pub fn bomb_counter_background(&self)   -> Nineslice { self.bomb_counter_background }
    pub fn bomb_counter_digit_size(&self)   -> Vec2      { self.bomb_counter_digit_size }
    pub fn bomb_counter_digit_offset(&self) -> Vec2      { self.bomb_counter_digit_offset }
    pub fn bomb_counter_digit_gap(&self)    -> f32       { self.bomb_counter_digit_gap }

    pub fn bomb_counter_height(&self) -> f32 {
        self.bomb_counter_digit_size.y + self.bomb_counter_digit_offset.y * 2.0
    }

    pub fn bomb_counter_size(&self, digits: u32) -> Vec2 {
        let d = digits as f32;
        Vec2::new(
            self.bomb_counter_digit_size.x * d + self.bomb_counter_digit_gap * (d-1.0).max(0.0)  + self.bomb_counter_digit_offset.x * 2.0,
            self.bomb_counter_height()
        )
    }

    pub fn bomb_counter_digit(&self, digit: CounterDigit) -> Rect {
        let pos = Vec2::new(match digit {
            CounterDigit::Empty    => 10,
            CounterDigit::Dash     => 11,
            CounterDigit::Digit(d) => d,
        } as f32, 0.0) * self.bomb_counter_digit_size;

        rect_from_pos_size(pos, self.bomb_counter_digit_size)
        .offset(self.bomb_counter_digit_origin)
    } 

    // Timer
    pub fn timer_background(&self) -> Nineslice { self.timer_background }
    pub fn timer_digit_size(&self) -> Vec2      { self.timer_digit_size }

    pub fn timer_size(&self) -> Vec2 {
        Vec2::new(
            self.timer_digit_gap * 4.0 + self.timer_digit_size.x * 4.0 + self.timer_colon_width,
            self.timer_digit_size.y,
        ) + self.timer_digit_offset * 2.0
    }

    // TODO: Mayhaps optimise this using an iter and indexes ?
    pub fn timer_digit_positions(&self) -> ([f32; 4], f32) {
        (
            [
                self.timer_digit_offset.x + (self.timer_digit_size.x + self.timer_digit_gap) * 0.0 + (self.timer_colon_width + self.timer_digit_gap) * 0.0,
                self.timer_digit_offset.x + (self.timer_digit_size.x + self.timer_digit_gap) * 1.0 + (self.timer_colon_width + self.timer_digit_gap) * 0.0,
                self.timer_digit_offset.x + (self.timer_digit_size.x + self.timer_digit_gap) * 2.0 + (self.timer_colon_width + self.timer_digit_gap) * 1.0,
                self.timer_digit_offset.x + (self.timer_digit_size.x + self.timer_digit_gap) * 3.0 + (self.timer_colon_width + self.timer_digit_gap) * 1.0,
            ],
            self.timer_digit_offset.x + (self.timer_digit_size.x + self.timer_digit_gap) * 2.0,
        )
    }

    pub fn timer_digit(&self, digit: Option<u32>) -> Rect {
        Rect::new(
            digit.unwrap_or(10).min(10) as f32 * self.timer_digit_size.x + self.timer_colon_width * 2.0,
            0.0,
            self.timer_digit_size.x,
            self.timer_digit_size.y,
        )
        .offset(self.timer_digit_origin)
    }
    
    pub fn timer_colon(&self, lit: bool) -> Rect {
        let x = match lit {
            true  => self.timer_colon_width,
            false => 0.0,
        };
        Rect::new(
            x,
            0.0,
            self.timer_colon_width,
            self.timer_digit_size.y,
        )
        .offset(self.timer_digit_origin)
    }
}





pub fn temp_style() -> Style {
    let texture = Texture2D::from_file_with_format(include_bytes!("../../../resources/spritesheet.png"), None);
    texture.set_filter(macroquad::texture::FilterMode::Nearest);

    Style {
        texture,

        background: Nineslice::new(81.0, 48.0, 3.0, 3.0, 1.0),
        url_text:   Color::from_rgba( 51, 118, 230, 255),
        shadow:     Color::from_rgba(  0,   0,   0, 128),

        button_idle:          Nineslice::new(81.0, 51.0, 3.0, 3.0, 1.0),
        button_down:          Nineslice::new(84.0, 51.0, 3.0, 3.0, 1.0),
        button_disabled:      Nineslice::new(87.0, 51.0, 3.0, 3.0, 1.0),
        button_down_offset:   Vec2::ONE,
        button_text:          Color::from_rgba( 24,  20,  37, 255),
        button_disabled_text: Color::from_rgba(110, 128, 156, 255),

        menu_item_idle:      MenuItemStyle { background: Color::from_hex(0xc0cbdc), text_col: Color::from_hex(0x181425) },
        menu_item_active:    MenuItemStyle { background: Color::from_hex(0x262b44), text_col: Color::from_hex(0xffffff) },
        dropdown_idle:       DropdownStyle { background: Color::from_hex(0xc0cbdc), text_col: Color::from_hex(0x181425), other_text_col: Color::from_hex(0x495673) },
        dropdown_active:     DropdownStyle { background: Color::from_hex(0x262b44), text_col: Color::from_hex(0xffffff), other_text_col: Color::from_hex(0xffffff) },
        dropdown_background: Nineslice::new(92.0, 43.0, 3.0, 3.0, 1.0),
        // dropdown_separator:  Rect::new(95.0, 43.0, 1.0, 2.0),
        dropdown_separator:  Nineslice::new(92.0, 46.0, 3.0, 3.0, 1.0),

        popup_title:          Nineslice::new(92.0, 37.0, 3.0, 3.0, 1.0),
        popup_body:           Nineslice::new(92.0, 40.0, 3.0, 3.0, 1.0),
        popup_title_text_col: Color::from_rgba(255, 255, 255, 255),
        popup_body_text_col:  Color::from_rgba( 24,  20,  37, 255),
        popup_close_idle:     Rect::new(85.0, 37.0, 7.0, 7.0),
        popup_close_down:     Rect::new(85.0, 44.0, 7.0, 7.0),
        
        minefield_border:          Nineslice::new(81.0, 11.0, 5.0, 5.0, 2.0),
        minefield_tile_origin:     Vec2::ZERO,
        minefield_tile_width:      9,
        minefield_tile_height:     9,
        minefield_tile_size:       Vec2::new(9.0, 9.0),
        minefield_selector:        Rect::new(81.0, 0.0, 11.0, 11.0),
        minefield_selector_offset: Vec2::ONE,

        status_v_pad: 3.0,
        face_button_idle:        Nineslice::new(86.0, 11.0, 3.0, 3.0, 1.0),
        face_button_down:        Nineslice::new(86.0, 14.0, 3.0, 3.0, 1.0),
        face_button_down_offset: Vec2::ONE,
        face_button_size:        Vec2::new(19.0, 19.0),
        face_origin:  Vec2::new( 0.0, 37.0),
        face_size:    Vec2::new(17.0, 11.0),
        face_offset:  Vec2::new( 1.0,  1.0),
        mouth_origin: Vec2::new( 0.0, 48.0),
        mouth_size:   Vec2::new(17.0,  6.0),
        mouth_offset: Vec2::new( 1.0, 12.0),

        bomb_counter_background:   Nineslice::new(89.0, 11.0, 3.0, 3.0, 1.0),
        bomb_counter_digit_origin: Vec2::new(0.0, 18.0),
        bomb_counter_digit_offset: Vec2::new(3.0, 2.0),
        bomb_counter_digit_size:   Vec2::new(8.0, 14.0),
        bomb_counter_digit_gap:    2.0,

        timer_background:   Nineslice::new(89.0, 14.0, 3.0, 3.0, 1.0),
        timer_digit_origin: Vec2::new(0.0, 32.0),
        timer_digit_offset: Vec2::new(2.0, 2.0),
        timer_digit_size:   Vec2::new(3.0, 5.0),
        timer_digit_gap:    1.0,
        timer_colon_width:  1.0,
    }
}

pub fn win_style() -> Style {
    let texture = Texture2D::from_file_with_format(include_bytes!("../../../resources/windows-spritesheet.png"), None);
    texture.set_filter(macroquad::texture::FilterMode::Nearest);

    Style {
        texture,

        background: Nineslice::new(85.0, 114.0, 7.0, 7.0, 3.0),
        url_text:   Color::from_rgba( 51, 118, 230, 255),
        shadow:     Color::from_rgba(  0,   0,   0, 128),

        button_idle:          Nineslice::new(81.0, 51.0, 3.0, 3.0, 1.0),
        button_down:          Nineslice::new(84.0, 51.0, 3.0, 3.0, 1.0),
        button_disabled:      Nineslice::new(87.0, 51.0, 3.0, 3.0, 1.0),
        button_down_offset:   Vec2::ONE,
        button_text:          Color::from_rgba( 24,  20,  37, 255),
        button_disabled_text: Color::from_rgba(110, 128, 156, 255),

        menu_item_idle:      MenuItemStyle { background: Color::from_hex(0xc0c0c0), text_col: Color::from_hex(0x000000) },
        menu_item_active:    MenuItemStyle { background: Color::from_hex(0x000080), text_col: Color::from_hex(0xffffff) },
        dropdown_idle:       DropdownStyle { background: Color::from_hex(0xc0c0c0), text_col: Color::from_hex(0x000000), other_text_col: Color::from_hex(0x606060) },
        dropdown_active:     DropdownStyle { background: Color::from_hex(0x000080), text_col: Color::from_hex(0xffffff), other_text_col: Color::from_hex(0xffffff) },
        dropdown_background: Nineslice::new(85.0, 121.0, 5.0, 5.0, 2.0),
        dropdown_separator:  Nineslice::new(90.0, 121.0, 3.0, 5.0, 1.0),

        popup_title:          Nineslice::new(92.0, 37.0, 3.0, 3.0, 1.0),
        popup_body:           Nineslice::new(92.0, 40.0, 3.0, 3.0, 1.0),
        popup_title_text_col: Color::from_rgba(255, 255, 255, 255),
        popup_body_text_col:  Color::from_rgba( 24,  20,  37, 255),
        popup_close_idle:     Rect::new(85.0, 37.0, 7.0, 7.0),
        popup_close_down:     Rect::new(85.0, 44.0, 7.0, 7.0),
        
        minefield_border:          Nineslice::new(95.0, 114.0, 7.0, 7.0, 3.0),
        minefield_tile_origin:     Vec2::new(0.0, 54.0),
        minefield_tile_width:      16,
        minefield_tile_height:     16,
        minefield_tile_size:       Vec2::new(16.0, 16.0),
        minefield_selector:        Rect::default(),
        minefield_selector_offset: Vec2::ZERO,

        status_v_pad: 3.0,
        face_button_idle:        Nineslice::new(85.0, 109.0, 5.0, 5.0, 2.0),
        face_button_down:        Nineslice::new(90.0, 109.0, 3.0, 3.0, 1.0),
        face_button_down_offset: Vec2::ONE,
        face_button_size:        Vec2::new(24.0, 24.0),
        face_origin:  Vec2::new( 0.0, 109.0),
        face_size:    Vec2::new(17.0, 17.0),
        face_offset:  Vec2::new( 4.0,  4.0),
        mouth_origin: Vec2::new( 0.0, 126.0),
        mouth_size:   Vec2::new(17.0, 17.0),
        mouth_offset: Vec2::new( 4.0,  4.0),

        bomb_counter_background:   Nineslice::new(93.0, 109.0, 3.0, 3.0, 1.0),
        bomb_counter_digit_origin: Vec2::new(10.0, 86.0),
        bomb_counter_digit_offset: Vec2::new(3.0, 2.0),
        bomb_counter_digit_size:   Vec2::new(13.0, 23.0),
        bomb_counter_digit_gap:    0.0,

        timer_background:   Nineslice::new(93.0, 109.0, 3.0, 3.0, 1.0),
        timer_digit_origin: Vec2::new(0.0, 86.0),
        timer_digit_offset: Vec2::new(2.0, 2.0),
        timer_digit_size:   Vec2::new(13.0, 23.0),
        timer_digit_gap:    0.0,
        timer_colon_width:  5.0,
    }
}

pub fn mini_style() -> Style {
    let texture = Texture2D::from_file_with_format(include_bytes!("../../../resources/mini-spritesheet.png"), None);
    texture.set_filter(macroquad::texture::FilterMode::Nearest);

    Style {
        texture,

        background: Nineslice::new(81.0, 48.0, 3.0, 3.0, 1.0),
        url_text:   Color::from_rgba( 51, 118, 230, 255),
        shadow:     Color::from_rgba(  0,   0,   0, 128),

        button_idle:          Nineslice::new(81.0, 51.0, 3.0, 3.0, 1.0),
        button_down:          Nineslice::new(84.0, 51.0, 3.0, 3.0, 1.0),
        button_disabled:      Nineslice::new(87.0, 51.0, 3.0, 3.0, 1.0),
        button_down_offset:   Vec2::ONE,
        button_text:          Color::from_rgba( 24,  20,  37, 255),
        button_disabled_text: Color::from_rgba(110, 128, 156, 255),

        menu_item_idle:      MenuItemStyle { background: Color::from_hex(0xc0cbdc), text_col: Color::from_hex(0x181425) },
        menu_item_active:    MenuItemStyle { background: Color::from_hex(0x262b44), text_col: Color::from_hex(0xffffff) },
        dropdown_idle:       DropdownStyle { background: Color::from_hex(0xc0cbdc), text_col: Color::from_hex(0x181425), other_text_col: Color::from_hex(0x495673) },
        dropdown_active:     DropdownStyle { background: Color::from_hex(0x262b44), text_col: Color::from_hex(0xffffff), other_text_col: Color::from_hex(0xffffff) },
        dropdown_background: Nineslice::new(92.0, 43.0, 3.0, 3.0, 1.0),
        dropdown_separator:  Nineslice::new(0.0, 0.0, 0.0, 0.0, 0.0),

        popup_title:          Nineslice::new(92.0, 37.0, 3.0, 3.0, 1.0),
        popup_body:           Nineslice::new(92.0, 40.0, 3.0, 3.0, 1.0),
        popup_title_text_col: Color::from_rgba(255, 255, 255, 255),
        popup_body_text_col:  Color::from_rgba( 24,  20,  37, 255),
        popup_close_idle:     Rect::new(85.0, 37.0, 7.0, 7.0),
        popup_close_down:     Rect::new(85.0, 44.0, 7.0, 7.0),
        
        minefield_border:          Nineslice::new(81.0, 11.0, 5.0, 5.0, 2.0),
        minefield_tile_origin:     Vec2::new(0.0, 57.0),
        minefield_tile_width:      1,
        minefield_tile_height:     1,
        minefield_tile_size:       Vec2::new(1.0, 1.0),
        minefield_selector:        Rect::default(),
        minefield_selector_offset: Vec2::ZERO,

        status_v_pad: 3.0,
        face_button_idle:        Nineslice::new(86.0, 11.0, 3.0, 3.0, 1.0),
        face_button_down:        Nineslice::new(86.0, 14.0, 3.0, 3.0, 1.0),
        face_button_down_offset: Vec2::ONE,
        face_button_size:        Vec2::new(19.0, 19.0),
        face_origin:  Vec2::new( 0.0, 37.0),
        face_size:    Vec2::new(17.0, 11.0),
        face_offset:  Vec2::new( 1.0,  1.0),
        mouth_origin: Vec2::new( 0.0, 48.0),
        mouth_size:   Vec2::new(17.0,  6.0),
        mouth_offset: Vec2::new( 1.0, 12.0),

        bomb_counter_background:   Nineslice::new(89.0, 11.0, 3.0, 3.0, 1.0),
        bomb_counter_digit_origin: Vec2::new(0.0, 18.0),
        bomb_counter_digit_offset: Vec2::new(3.0, 2.0),
        bomb_counter_digit_size:   Vec2::new(8.0, 14.0),
        bomb_counter_digit_gap:    2.0,

        timer_background:   Nineslice::new(89.0, 14.0, 3.0, 3.0, 1.0),
        timer_digit_origin: Vec2::new(0.0, 32.0),
        timer_digit_offset: Vec2::new(2.0, 2.0),
        timer_digit_size:   Vec2::new(3.0, 5.0),
        timer_digit_gap:    1.0,
        timer_colon_width:  1.0,
    }
}