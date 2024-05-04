use macroquad::{input::MouseButton, math::{vec2, Rect, Vec2}, miniquad::window::order_quit};

use crate::minesweeper::{Difficulty, DifficultyValues, MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_WIDTH};

use super::{elements::{align_beg, align_end, align_mid, button_text, text, text_field, TextFieldKind}, hash_string, menubar::Menubar, renderer::{DrawShape, Renderer}, spritesheet, state::{ButtonState, Id, State}};

#[derive(Default)]
pub struct Popups {
    popups: Vec<Popup>,
    popup_drag_offset: Vec2,
}

impl Popups {
    pub fn add(&mut self, kind: PopupKind, state: &State) {
        self.popups.retain(|p| std::mem::discriminant(&p.kind) != std::mem::discriminant(&kind));
        self.popups.push(Popup::new(kind, state));
    }

    pub fn update(&mut self, state: &mut State, menubar: &Menubar, renderer: &mut Renderer) {
        let mut popup_returns: Vec<PopupReturn> = Vec::new();
        let mut close = None; 
        let mut front = None; 

        for (i, popup) in self.popups.iter_mut().enumerate().rev() {
            let (action, return_value) = popup.update(&mut self.popup_drag_offset, state, menubar, renderer);
            match action {
                PopupAction::Close => close = Some(i),
                PopupAction::Front => front = Some(i),
                _ => ()
            }
            if let Some(r) = return_value {
                popup_returns.push(r);
            }
        }

        if let Some(close) = close {
            self.popups.remove(close);
        }
        if let Some(front) = front {
            let popup = self.popups.remove(front);
            self.popups.push(popup);
        }

        // for p in popup_returns {
        //     match p {
        //         PopupReturn::NewGame { difficulty } => ()
        //     }
        // }
    }
}

pub enum PopupKind {
    NewGame { difficulty: Difficulty },
    Custom { width: String, height: String, bomb_count: String },
    About,
    Win,
    Exit,
}

impl PopupKind {
    pub fn new_game(difficulty: Difficulty) -> Self {
        Self::NewGame { difficulty }
    }
    pub fn custom(difficulty_values: Option<DifficultyValues>) -> Self {
        let (width, height, bomb_count) = match difficulty_values {
            Some(d) => (d.width.to_string(), d.height.to_string(), d.bomb_count.to_string()),
            None => (String::new(), String::new(), String::new()),
        };
        Self::Custom { width, height, bomb_count }
    }
}

pub enum PopupReturn {
    NewGame { difficulty: Difficulty },
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
            PopupKind::NewGame{..} => ("New game", vec2( 90.0, 40.0)),
            PopupKind::Custom{..}  => ("Custom",   vec2( 78.0, 58.0)),
            PopupKind::About       => ("About",    vec2(100.0, 80.0)),
            PopupKind::Win         => ("You win!", vec2( 70.0, 40.0)),
            PopupKind::Exit        => ("Exit",     vec2( 70.0, 40.0)),
        };
        let pos = (state.screen_size() - size) / 2.0;

        Popup { pos, size, title: title.to_owned(), kind, id: hash_string(&format!("popup!!{}", macroquad::miniquad::date::now())) }
    }

    pub fn update(&mut self, popup_drag_offset: &mut Vec2, state: &mut State, menubar: &Menubar, renderer: &mut Renderer) -> (PopupAction, Option<PopupReturn>) {
        let titlebar_height = renderer.text_renderer.text_size(&self.title, None).y + 3.0;
        self.pos = self.pos
            .min(state.screen_size() - self.size)
            .max(vec2(0.0, menubar.height()));

        let titlebar = Rect::new(self.pos.x, self.pos.y,              self.size.x, titlebar_height);
        let body     = Rect::new(self.pos.x, self.pos.y + titlebar.h, self.size.x, self.size.y - titlebar.h);

        let id = self.id;
        let mut close = Popup::close_button(id.wrapping_add(1), titlebar, state, renderer);
        let mut return_value = None;

        // This is done AFTER making the close button, so when you make it active it doesn't bring the popup to the top
        let active_before = state.active_item;

        // Elements inside of the popups
        match &mut self.kind {
            PopupKind::NewGame { difficulty } => {
                // button clicked
                if false {
                    close = true;
                    return_value = Some(PopupReturn::NewGame { difficulty: *difficulty })
                }
            }
            PopupKind::Custom { width, height, bomb_count } => {
                text("Width" .to_owned(), None, spritesheet::POPUP_BODY_TEXT, align_mid(body.x + 17.0), align_beg(body.y +  4.0), renderer);
                text("Height".to_owned(), None, spritesheet::POPUP_BODY_TEXT, align_mid(body.x + 17.0), align_beg(body.y + 14.0), renderer);
                text("Bombs" .to_owned(), None, spritesheet::POPUP_BODY_TEXT, align_mid(body.x + 17.0), align_beg(body.y + 24.0), renderer);
                // TODO: THIS
                let width_hint  = format!("{:?} - {:?}", MIN_WIDTH,  MAX_WIDTH);
                let height_hint = format!("{:?} - {:?}", MIN_HEIGHT, MAX_HEIGHT);
                text_field(
                    id.wrapping_add(4), align_end(body.right() - 3.0), align_beg(body.y +  2.0),
                    41.0, TextFieldKind::Digits { min: MIN_WIDTH,  max: MAX_WIDTH  }, width, width_hint, state, renderer
                );
                text_field(
                    id.wrapping_add(5), align_end(body.right() - 3.0), align_beg(body.y + 12.0),
                    41.0, TextFieldKind::Digits { min: MIN_HEIGHT, max: MAX_HEIGHT }, height, height_hint, state, renderer
                );

                let w = match width.parse::<usize>() {
                    Ok(w) if (MIN_WIDTH..=MAX_WIDTH).contains(&w) => Some(w),
                    _ => None,
                };
                let h = match height.parse::<usize>() {
                    Ok(h) if (MIN_HEIGHT..=MAX_HEIGHT).contains(&h) => Some(h),
                    _ => None,
                };

                let bombs_hint = match (w, h) {
                    (Some(w), Some(h)) => format!("{:?} - {:?}", 1, (w-1)*(h-1)),
                    _ => String::new(),
                };

                text_field(
                    id.wrapping_add(6), align_end(body.right() - 3.0), align_beg(body.y + 22.0),
                    41.0, TextFieldKind::Digits { min: MIN_HEIGHT, max: MAX_HEIGHT }, bomb_count, bombs_hint, state, renderer
                );

                let submit_valid = false;

                let submit = button_text(id.wrapping_add(2), "Submit".to_owned(), align_end(body.right()-3.0), align_end(body.bottom()-3.0), !submit_valid, state, renderer) == ButtonState::Released;
                if submit {
                    return_value = Some(PopupReturn::NewGame { difficulty: Difficulty::custom(200, 100, 2000) });
                    close = true;
                }
                close = close || button_text(id.wrapping_add(3), "Cancel".to_owned(), align_end(body.right()-36.0), align_end(body.bottom()-3.0), false, state, renderer) == ButtonState::Released;
            }
            PopupKind::About => {
                renderer.draw(DrawShape::text(body.x + 3.0, body.y + 3.0,
                    // TODO: Add text styling?
                    "Minesweeper\n\njumbledFox - 2024\n\nMade in Rust and the\nMacroquad framework.\n\nOpen source on Github!\njumbledFox.github.io".to_owned(),
                spritesheet::POPUP_BODY_TEXT));
            }
            PopupKind::Win => {
                renderer.draw(DrawShape::text(body.x + 3.0, body.y + 3.0, "You win,\ncongratulations!".to_owned(), spritesheet::POPUP_BODY_TEXT));
                if false {
                    close = true;
                }
            }
            PopupKind::Exit => {
                renderer.draw(DrawShape::text(body.x + 3.0, body.y + 3.0, "Are you sure you\nwant to exit?".to_owned(), spritesheet::POPUP_BODY_TEXT));
                if button_text(
                    id.wrapping_add(2),
                    "Exit".to_owned(),
                    align_end(body.right() -3.0),
                    align_end(body.bottom()-3.0),
                    false,
                    state,
                    renderer
                ) == ButtonState::Released {
                    order_quit();
                }
                close = close || button_text(id.wrapping_add(3), "Cancel".to_owned(), align_end(body.right()-25.0), align_end(body.bottom()-3.0), false, state, renderer) == ButtonState::Released;
            }
        }

        // Dragging the popup around
        let hovered = state.mouse_in_rect(titlebar) || state.mouse_in_rect(body);

        if state.hot_item.assign_if_none_and(id, hovered) {
            if state.active_item.assign_if_none_and(id, state.mouse_down(MouseButton::Left)) {
                *popup_drag_offset = state.mouse_pos() - self.pos;
            }
        }

        if state.active_item == id {
            state.hot_item = super::state::SelectedItem::Unavailable;
            self.pos = state.mouse_pos() - *popup_drag_offset;
        }
        
        renderer.draw(DrawShape::text(titlebar.x + 2.0, titlebar.y + 2.0, self.title.clone(), spritesheet::POPUP_TITLE_TEXT));
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