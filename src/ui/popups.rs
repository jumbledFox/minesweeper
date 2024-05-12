use macroquad::{input::MouseButton, math::{vec2, Rect, Vec2}, miniquad::window::order_quit};

use crate::minesweeper::{Difficulty, DifficultyValues, MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_WIDTH};

use super::{elements::{align_beg, align_end, align_mid, button_text, text, text_field, url, Align, TextFieldKind}, hash_string, menubar::Menubar, minesweeper_element::MinesweeperElement, renderer::{DrawShape, Renderer}, spritesheet, state::{ButtonState, Id, State}};

#[derive(Default)]
pub struct Popups {
    popups: Vec<Popup>,
    return_values: Vec<PopupReturn>,
    drag_offset: Vec2,
}

impl Popups {
    pub fn add(&mut self, kind: PopupKind, state: &State) {
        self.popups.retain(|p| std::mem::discriminant(&p.kind) != std::mem::discriminant(&kind));
        self.popups.push(Popup::new(kind, state));
    }

    pub fn update(&mut self, state: &mut State, menubar: &Menubar, renderer: &mut Renderer) {
        self.return_values.clear();
        let mut close = None; 
        let mut front = None; 

        for (i, popup) in self.popups.iter_mut().enumerate().rev() {
            let (action, return_value) = popup.update(&mut self.drag_offset, state, menubar, renderer);
            match action {
                PopupAction::Close => close = Some(i),
                PopupAction::Front => front = Some(i),
                _ => ()
            }
            if let Some(r) = return_value {
                self.return_values.push(r);
            }
        }

        if let Some(close) = close {
            self.popups.remove(close);
        }
        if let Some(front) = front {
            let popup = self.popups.remove(front);
            self.popups.push(popup);
        }
    }

    pub fn handle_returns(&mut self, minesweeper_element: &mut MinesweeperElement) {
        for return_value in &mut self.return_values {
            match return_value {
                PopupReturn::NewGame { difficulty } => minesweeper_element.new_game(*difficulty),
                PopupReturn::Exit                   => order_quit(),
            }
        }
    }
}

pub enum PopupKind {
    NewGame { difficulty: Difficulty },
    Custom { width: String, height: String, bomb_count: String },
    About,
    Hint,
    Win,
    Exit,
}

impl PopupKind {
    pub fn new_game(difficulty: Difficulty) -> Self {
        Self::NewGame { difficulty }
    }
    pub fn custom(difficulty: Option<Difficulty>) -> Self {
        let (width, height, bomb_count) = match difficulty {
            Some(d) => {
                let v = d.values();
                (format!("{:?}", v.width()), format!("{:?}", v.height()), format!("{:?}", v.bomb_count()))
            },
            None => (String::new(), String::new(), String::new()),
        };
        Self::Custom { width, height, bomb_count }
    }
}

pub enum PopupReturn {
    NewGame { difficulty: Difficulty },
    Exit,
}

pub enum PopupAction {
    None,
    Front,
    Close,
}

pub struct Popup {
    pos: Vec2,
    size: Vec2,
    title: String,
    kind: PopupKind,
    id: Id,
}

impl Popup {
    pub fn new(kind: PopupKind, state: &State) -> Popup {
        let (title, size) = match kind {
            PopupKind::NewGame{..} => ("New game", vec2( 90.0, 46.0)),
            PopupKind::Custom{..}  => ("Custom",   vec2( 78.0, 58.0)),
            PopupKind::About       => ("About",    vec2(100.0, 70.0)),
            PopupKind::Hint        => ("Hint",     vec2( 60.0, 40.0)),
            PopupKind::Win         => ("You win!", vec2( 70.0, 34.0)),
            PopupKind::Exit        => ("Exit",     vec2( 70.0, 40.0)),
        };
        let pos = (state.screen_size() - size) / 2.0;

        Popup { pos, size, title: title.to_owned(), kind, id: hash_string(&format!("popup!!{}", macroquad::miniquad::date::now())) }
    }

    pub fn update(&mut self, drag_offset: &mut Vec2, state: &mut State, menubar: &Menubar, renderer: &mut Renderer) -> (PopupAction, Option<PopupReturn>) {
        let titlebar_height = renderer.text_renderer.text_size(&self.title, None).y + 3.0;
        self.pos = self.pos
            // .min(state.screen_size() - self.size)
            .min(state.screen_size() - titlebar_height)
            .max(vec2(-self.size.x + titlebar_height, menubar.height()))
            .round();

        let titlebar = Rect::new(self.pos.x, self.pos.y,              self.size.x, titlebar_height);
        let body     = Rect::new(self.pos.x, self.pos.y + titlebar.h, self.size.x, self.size.y - titlebar.h);

        let id = self.id;
        let mut close = Popup::close_button(id.wrapping_add(1), titlebar, state, renderer);
        let mut return_value = None;

        // This is done AFTER doing the close button, so that when you click X it doesn't bring the popup to the top
        let active_before = state.active_item;

        // Elements inside of the popups
        // This is a bit messy, but it's easy to add to.
        match &mut self.kind {
            PopupKind::NewGame { difficulty } => {
                text("Are you sure you want\nto start a new game?".to_owned(), None, spritesheet::POPUP_BODY_TEXT, Align::Beg(body.x+3.0), Align::Beg(body.y+3.0), renderer);

                if button_text(id.wrapping_add(2), "Yes".to_owned(), align_end(body.right() -3.0), align_end(body.bottom()-3.0), false, state, renderer).released() {
                    close = true;
                    return_value = Some(PopupReturn::NewGame { difficulty: *difficulty });
                }
                close = close || button_text(id.wrapping_add(3), "Cancel".to_owned(), align_end(body.right()-25.0), align_end(body.bottom()-3.0), false, state, renderer).released();
            }
            PopupKind::Custom { width, height, bomb_count } => {
                text("Width" .to_owned(), None, spritesheet::POPUP_BODY_TEXT, align_mid(body.x + 17.0), align_beg(body.y +  4.0), renderer);
                text("Height".to_owned(), None, spritesheet::POPUP_BODY_TEXT, align_mid(body.x + 17.0), align_beg(body.y + 14.0), renderer);
                text("Bombs" .to_owned(), None, spritesheet::POPUP_BODY_TEXT, align_mid(body.x + 17.0), align_beg(body.y + 24.0), renderer);
                // TODO: This is also a bit messy, but once again it works and is easy to know what's going on
                let width_hint  = format!("{:?} - {:?}", MIN_WIDTH,  MAX_WIDTH);
                let height_hint = format!("{:?} - {:?}", MIN_HEIGHT, MAX_HEIGHT);

                let (width_id, height_id, bombs_id) = (id.wrapping_add(4), id.wrapping_add(5), id.wrapping_add(6));
                text_field(
                    width_id,  align_end(body.right() - 3.0), align_beg(body.y +  2.0),
                    41.0, width,  width_hint,  TextFieldKind::Digits { min: MIN_WIDTH,  max: MAX_WIDTH  }, None, Some(height_id), 7, state, renderer
                );
                text_field(
                    height_id, align_end(body.right() - 3.0), align_beg(body.y + 12.0),
                    41.0, height, height_hint, TextFieldKind::Digits { min: MIN_HEIGHT, max: MAX_HEIGHT }, Some(width_id), Some(bombs_id), 7, state, renderer
                );

                let (w, h) = match (width.parse::<usize>(), height.parse::<usize>()) {
                    (Ok(w), Ok(h)) if Difficulty::dimensions_in_range(w, h) => (Some(w), Some(h)),
                    _ => (None, None),
                };

                let bombs_hint = match (w, h) {
                    (Some(w), Some(h)) => match Difficulty::max_bombs(w, h) {
                        Some(b) => format!("{} - {}", 0, b),
                        None => String::new()
                    }
                    _ => String::new(),
                };

                text_field(
                    bombs_id, align_end(body.right() - 3.0), align_beg(body.y + 22.0),
                    41.0, bomb_count, bombs_hint, TextFieldKind::Digits { min: MIN_HEIGHT, max: MAX_HEIGHT }, Some(height_id), None, 7, state, renderer
                );
                // TODO: Maybe add nice sliders to all of these??

                let difficulty = match (w, h, bomb_count.parse::<usize>()) {
                    (Some(w), Some(h), Ok(b)) => Difficulty::custom(w, h, b),
                    _ => None,
                };

                // Buttons
                if button_text(id.wrapping_add(2), "Submit".to_owned(), align_end(body.right()-3.0), align_end(body.bottom()-3.0), difficulty.is_none(), state, renderer).released() {
                    if let Some(difficulty) = difficulty {
                        return_value = Some(PopupReturn::NewGame { difficulty });
                        close = true;
                    }
                }
                close = close || button_text(id.wrapping_add(3), "Cancel".to_owned(), align_end(body.right()-36.0), align_end(body.bottom()-3.0), false, state, renderer).released();
            }
            PopupKind::About => {
                url(
                    id.wrapping_add(2), "jumbledFox".to_owned(), "https://jumbledFox.github.io".to_owned(), None,
                    Align::Beg(body.x+3.0), Align::Beg(body.y+10.0), state, renderer
                );
                url(
                    id.wrapping_add(2), "Macroquad".to_owned(), "https://github.com/not-fl3/macroquad".to_owned(), None,
                    Align::Beg(body.x+24.0), Align::Beg(body.y+31.0), state, renderer
                );
                url(
                    id.wrapping_add(2), "Github".to_owned(), "https://github.com/jumbledfox/minesweeper".to_owned(), None,
                    Align::Beg(body.x+63.0), Align::Beg(body.y+45.0), state, renderer
                );
                text(
                    "Minesweeper\njumbledFox - 2024\n\nMade with love in Rust, \nusing Macroquad.\n\nOpen source on Github!\nThanks for playing <3".to_owned(),
                    None, spritesheet::POPUP_BODY_TEXT, Align::Beg(body.x+3.0), Align::Beg(body.y+3.0), renderer
                );
            }
            PopupKind::Hint => {
                text("You're on your\nown.".to_owned(), None, spritesheet::POPUP_BODY_TEXT, Align::Beg(body.x+3.0), Align::Beg(body.y+3.0), renderer);
                close = close || button_text(id.wrapping_add(3), "Ah.".to_owned(), align_end(body.right()-3.0), align_end(body.bottom()-3.0), false, state, renderer).released();
            }
            PopupKind::Win => {
                text("Congratulations!".to_owned(), None, spritesheet::POPUP_BODY_TEXT, Align::Beg(body.x+3.0), Align::Beg(body.y+3.0), renderer);
                close = close || button_text(id.wrapping_add(3), "Yippee!".to_owned(), align_end(body.right()-3.0), align_end(body.bottom()-3.0), false, state, renderer).released();
            }
            PopupKind::Exit => {
                text("Are you sure you\nwant to exit?".to_owned(), None, spritesheet::POPUP_BODY_TEXT, Align::Beg(body.x+3.0), Align::Beg(body.y+3.0), renderer);
                if button_text(id.wrapping_add(2), "Exit".to_owned(), align_end(body.right() -3.0), align_end(body.bottom()-3.0), false, state, renderer).released() {
                    return_value = Some(PopupReturn::Exit);
                }
                close = close || button_text(id.wrapping_add(3), "Cancel".to_owned(), align_end(body.right()-25.0), align_end(body.bottom()-3.0), false, state, renderer).released()
            }
        }

        // Dragging the popup around
        let hovered = state.mouse_in_rect(titlebar) || state.mouse_in_rect(body);

        if state.hot_item.assign_if_none_and(id, hovered) {
            if state.active_item.assign_if_none_and(id, state.mouse_down(MouseButton::Left)) {
                *drag_offset = state.mouse_pos() - self.pos;
            }
        }

        if state.active_item == id {
            state.hot_item = super::state::SelectedItem::Unavailable;
            self.pos = state.mouse_pos() - *drag_offset;
        }
        
        renderer.draw(DrawShape::text(titlebar.x + 2.0, titlebar.y + 2.0, self.title.clone(), None, None, None, spritesheet::POPUP_TITLE_TEXT));
        renderer.draw(DrawShape::nineslice(titlebar, spritesheet::POPUP_TITLE));
        renderer.draw(DrawShape::nineslice(body,     spritesheet::POPUP_BODY));
        renderer.draw(DrawShape::rect(body.combine_with(titlebar).offset(vec2(3.0, 3.0)), spritesheet::SHADOW));
        
        let action = match (close, active_before.is_none() && !state.active_item.is_none()) {
            (true, _) => PopupAction::Close,
            (_, true) => PopupAction::Front,
            _ => PopupAction::None,
        };
        (action, return_value)
    }

    fn close_button(id: Id, titlebar: Rect, state: &mut State, renderer: &mut Renderer) -> bool {
        let pos = titlebar.point() + vec2(titlebar.w - 8.0, 1.0);
        let rect = Rect::new(pos.x, pos.y, 7.0, 7.0);
        let button_state = state.button_state(id, state.mouse_in_rect(rect), false, false);

        let colors = (
            spritesheet::popup_close_color(button_state != ButtonState::Idle),
            spritesheet::popup_close_color(button_state == ButtonState::Idle),
        );
        renderer.draw(DrawShape::image(pos.x + 2.0, pos.y + 2.0, spritesheet::POPUP_CLOSE, Some(colors.0)));
        renderer.draw(DrawShape::rect(rect, colors.1));

        button_state == ButtonState::Released
    }
}