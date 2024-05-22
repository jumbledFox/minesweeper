use macroquad::{input::MouseButton, math::{vec2, Rect, Vec2}, miniquad::window::order_quit};

use crate::minesweeper::{Difficulty, MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_WIDTH};

use super::{elements::{self, Align}, hash_string, menubar::Menubar, minesweeper_element::MinesweeperElement, renderer::{style::SHADOW, DrawShape, Renderer}, state::{ButtonState, Id, State}};

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
            .min(state.screen_size() - titlebar_height)
            .max(vec2(-self.size.x + titlebar_height, menubar.height()))
            .round();

        let title_rect = Rect::new(self.pos.x, self.pos.y,                self.size.x, titlebar_height);
        let body_rect  = Rect::new(self.pos.x, self.pos.y + title_rect.h, self.size.x, self.size.y - title_rect.h);

        let id = self.id;
        let mut id_add = 1;
        
        let mut return_value = None;
        let mut close = Popup::close_button(id.wrapping_add(id_add), title_rect, state, renderer);

        let active_before = state.active_item;

        let text = |text: String, x: Align, y: Align, renderer: &mut Renderer| {
            elements::text(text, None, renderer.style().text(), x, y, renderer)
        };
        let url = |text: String, url: String, x: Align, y: Align, state: &mut State, renderer: &mut Renderer, id_add: &mut Id| {
            *id_add += 1;
            elements::url(self.id.wrapping_add(*id_add), text, url, None, x, y, state, renderer)
        };
        let button = |text: String, x: Align, y: Align, disabled: bool, state: &mut State, renderer: &mut Renderer, id_add: &mut Id| {
            *id_add += 1;
            elements::button_text(self.id.wrapping_add(*id_add), text, x, y, disabled, state, renderer)
        };
        let number_field = |number: &mut String, hint: String, x: Align, y: Align, w: f32, state: &mut State, renderer: &mut Renderer, id_add: &mut Id| {
            *id_add += 1;
            elements::number_field(self.id.wrapping_add(*id_add), number, 7, hint, x, y, w, state, renderer)
        }; 

        match &mut self.kind {
            PopupKind::Exit => {
                text("Are you sure you\nwant to exit?".to_owned(), Align::Beg(body_rect.x+3.0), Align::Beg(body_rect.y+3.0), renderer);
                if button("Exit".to_owned(), Align::End(body_rect.right() - 3.0), Align::End(body_rect.bottom() - 3.0), false, state, renderer, &mut id_add).released() {
                    return_value = Some(PopupReturn::Exit);
                    close = true;
                }
                close = close | button("Cancel".to_owned(), Align::End(body_rect.right() - 25.0), Align::End(body_rect.bottom() - 3.0), false, state, renderer, &mut id_add).released();
            }
            PopupKind::NewGame { difficulty } => {
                text("Are you sure you want\nto start a new game?".to_owned(), Align::Beg(body_rect.x+3.0), Align::Beg(body_rect.y+3.0), renderer);
                if button("Yes".to_owned(), Align::End(body_rect.right() - 3.0), Align::End(body_rect.bottom() - 3.0), false, state, renderer, &mut id_add).released() {
                    return_value = Some(PopupReturn::NewGame { difficulty: *difficulty });
                    close = true;
                }
                close = close | button("Cancel".to_owned(), Align::End(body_rect.right() - 25.0), Align::End(body_rect.bottom() - 3.0), false, state, renderer, &mut id_add).released();
            }
            PopupKind::Hint => {
                text("You're on your\nown.".to_owned(),  Align::Beg(body_rect.x+3.0),       Align::Beg(body_rect.y+3.0), renderer);
                close = close | button("Ah.".to_owned(), Align::End(body_rect.right()-3.0), Align::End(body_rect.bottom()-3.0), false, state, renderer, &mut id_add).released();
            }
            PopupKind::Win => {
                text("Congratulations!".to_owned(),  Align::Beg(body_rect.x+3.0),       Align::Beg(body_rect.y+3.0), renderer);
                close = close | button("Yippee!".to_owned(), Align::End(body_rect.right()-3.0), Align::End(body_rect.bottom()-3.0), false, state, renderer, &mut id_add).released();
            }
            PopupKind::About => {
                url(
                    "jumbledFox".to_owned(), "https://jumbledFox.github.io".to_owned(),          Align::Beg(body_rect.x+3.0),  Align::Beg(body_rect.y+10.0), 
                    state, renderer, &mut id_add
                );
                url(
                    "Macroquad".to_owned(), "https://github.com/not-fl3/macroquad".to_owned(),   Align::Beg(body_rect.x+24.0), Align::Beg(body_rect.y+31.0),
                    state, renderer, &mut id_add
                );
                url(
                    "Github".to_owned(), "https://github.com/jumbledfox/minesweeper".to_owned(), Align::Beg(body_rect.x+63.0), Align::Beg(body_rect.y+45.0),
                    state, renderer, &mut id_add
                );
                text(
                    "Minesweeper\njumbledFox - 2024\n\nMade with love in Rust, \nusing Macroquad.\n\nOpen source on Github!\nThanks for playing <3".to_owned(),
                    Align::Beg(body_rect.x + 3.0), Align::Beg(body_rect.y + 3.0), renderer
                )
            }
            PopupKind::Custom { width, height, bomb_count } => {
                text("Width" .to_owned(), Align::Mid(body_rect.x + 17.0), Align::Beg(body_rect.y +  4.0), renderer);
                text("Height".to_owned(), Align::Mid(body_rect.x + 17.0), Align::Beg(body_rect.y + 14.0), renderer);
                text("Bombs" .to_owned(), Align::Mid(body_rect.x + 17.0), Align::Beg(body_rect.y + 24.0), renderer);
                number_field(width,  format!("{:?} - {:?}", MIN_WIDTH,  MAX_WIDTH),  Align::End(body_rect.right()-3.0), Align::Beg(body_rect.y +  2.0), 41.0, state, renderer, &mut id_add);
                number_field(height, format!("{:?} - {:?}", MIN_HEIGHT, MAX_HEIGHT), Align::End(body_rect.right()-3.0), Align::Beg(body_rect.y + 12.0), 41.0, state, renderer, &mut id_add);
                let (size, max_bombs) = match (width.parse::<usize>(), height.parse::<usize>()) {
                    (Ok(w), Ok(h)) if Difficulty::dimensions_in_range(w, h) => (Some((w, h)), Difficulty::max_bombs(w, h)),
                    _ => (None, None),
                };
                let bomb_hint = match max_bombs {
                    Some(m) => format!("0 - {:?}", m),
                    _ => "".to_owned(),
                };
                number_field(bomb_count, bomb_hint, Align::End(body_rect.right()-3.0), Align::Beg(body_rect.y+22.0), 41.0, state, renderer, &mut id_add);

                let diff = match (size, bomb_count.parse::<usize>()) {
                    (Some((w, h)), Ok(b)) => Difficulty::custom(w, h, b),
                    _ => None
                };

                if button("Submit".to_owned(), Align::End(body_rect.right()-3.0), Align::End(body_rect.bottom()-3.0), diff.is_none(), state, renderer, &mut id_add).released() {
                    if let Some(difficulty) = diff {
                        return_value = Some(PopupReturn::NewGame { difficulty });
                        close = true;
                    }
                }
                close = close | button("Cancel".to_owned(), Align::End(body_rect.right()-36.0), Align::End(body_rect.bottom()-3.0), false, state, renderer, &mut id_add).released();
            }
        }

        // Dragging the popup around
        let hovered = state.mouse_in_rect(title_rect) || state.mouse_in_rect(body_rect);

        if state.hot_item.assign_if_none_and(id, hovered) {
            if state.active_item.assign_if_none_and(id, state.mouse_down(MouseButton::Left)) {
                *drag_offset = state.mouse_pos() - self.pos;
            }
        }

        if state.active_item == id {
            state.hot_item = super::state::SelectedItem::Unavailable;
            self.pos = state.mouse_pos() - *drag_offset;
        }
        
        // let title_text_size = renderer.text_renderer.text_size(&self.title, None).y;
        renderer.draw(DrawShape::text(
            title_rect.x + 2.0,
            title_rect.y + 2.0,
            self.title.clone(), None, None, None, renderer.style().popup_title_text()
        ));
        renderer.draw(DrawShape::nineslice(title_rect, renderer.style().popup_title()));
        renderer.draw(DrawShape::nineslice(body_rect,  renderer.style().popup_body()));
        renderer.draw(DrawShape::rect(body_rect.combine_with(title_rect).offset(vec2(3.0, 3.0)), SHADOW));
        
        let action = match (close, active_before.is_none() && !state.active_item.is_none()) {
            (true, _) => PopupAction::Close,
            (_, true) => PopupAction::Front,
            _         => PopupAction::None,
        };
        (action, return_value)
    }

    fn close_button(id: Id, title_rect: Rect, state: &mut State, renderer: &mut Renderer) -> bool {
        let rect = Rect::new(title_rect.right() - 8.0, title_rect.y + 1.0, 7.0, 7.0);
        let button_state = state.button_state(id, state.mouse_in_rect(rect), false, false);

        let source = renderer.style().popup_close(button_state != ButtonState::Idle);

        renderer.draw(DrawShape::image(rect.x, rect.y, source, None));

        button_state == ButtonState::Released
    }
}