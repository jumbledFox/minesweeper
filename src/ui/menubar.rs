use macroquad::math::{vec2, Rect, Vec2};

use super::{elements::Align, hash_string, renderer::{style::SHADOW, DrawShape, Renderer}, state::{ButtonState, Id, SelectedItem, State}};

#[derive(Default)]
pub struct Menubar {
    height: f32,

    item_current: Option<Id>,
    item_current_prev: Option<Id>,

    item_current_x: f32,
    item_next_x: f32,

    dropdown_width: f32,
    
    dropdown_start_y: f32,
    dropdown_current_y: f32,
    dropdown_next_y: f32,
    
    dropdown_rect: Rect,
}

impl Menubar {
    pub fn height(&self) -> f32 {
        self.height
    } 

    pub fn begin(&mut self) {
        self.item_current_prev = self.item_current;
        self.item_next_x = 0.0;
    }

    pub fn finish(&mut self, state: &mut State, renderer: &mut Renderer) {
        renderer.draw(super::renderer::DrawShape::Rect {
            x: self.item_next_x,
            y: 0.0,
            w: state.screen_size().x - self.item_next_x + 1.0, // + 1.0 for rounding..
            h: self.height,
            color: renderer.style().menubar(false).0,
        });

        // If anywhere that's not the dropdown has been clicked, deselect the menubar
        if self.item_current.is_some() && self.item_current_prev.is_some()
        && state.mouse_pressed(macroquad::input::MouseButton::Left) && !state.mouse_in_rect(self.dropdown_rect) {
            state.active_item = SelectedItem::Unavailable;
            self.item_current = None;
        }
    }

    pub fn item(&mut self, text: String, dropdown_width: f32, state: &mut State, renderer: &mut Renderer) -> bool {
        self.item_current_x = self.item_next_x;
        
        let size = renderer.text_renderer.text_size(&text, None) + vec2(4.0, 2.0);
        self.item_next_x += size.x;
        self.height = self.height.max(size.y);
        
        self.dropdown_start_y = self.height + renderer.style().dropdown_background().padding;
        self.dropdown_next_y = self.dropdown_start_y;
        self.dropdown_width = dropdown_width;
        
        let id = hash_string(&text);
        let rect = Rect::new(self.item_current_x, 0.0, size.x, size.y);
        let hovered = state.mouse_in_rect(rect);
        let button_state = state.button_state(id, hovered, false, false);
        // If a dropdown is open and the mouse has hovered this menu item, or if this menu item's been clicked, set THIS to be the current one.
        if (button_state == ButtonState::Hovered && self.item_current.is_some()) || button_state == ButtonState::Clicked {
            self.item_current = Some(id);
        }
        
        let (background, text_col, _) = renderer.style().menubar(self.item_current == Some(id) || state.hot_item == id);

        renderer.draw(super::renderer::DrawShape::text(rect.x + 2.0, rect.y + 1.0, text, None, None, None,text_col));
        renderer.draw(super::renderer::DrawShape::rect(rect, background));

        self.item_current == Some(id)
    }

    pub fn finish_item(&mut self, state: &mut State, renderer: &mut Renderer) {
        // If the dropdown doesn't go down at all, it has not dropdown items and therefore doesn't have a rect,
        // so we don't really care about doing anything
        if self.dropdown_next_y == self.dropdown_start_y {
            self.dropdown_rect = Rect::new(0.0, 0.0, 0.0, 0.0);
            return;
        }

        let pad = renderer.style().dropdown_background().padding;
        self.dropdown_rect =  Rect::new(
            self.item_current_x,
            self.dropdown_start_y - pad,
            self.dropdown_width  + pad * 2.0,
            self.dropdown_next_y + pad * 2.0 - self.dropdown_start_y,
        );

        // Draw the dropdown box and it's shadow
        renderer.draw(DrawShape::nineslice(self.dropdown_rect, renderer.style().dropdown_background()));
        renderer.draw(DrawShape::rect(self.dropdown_rect.offset(Vec2::splat(3.0)), SHADOW));

        // Make it so the box captures the hot item
        state.hot_item.make_unavailable_if_none_and(state.mouse_in_rect(self.dropdown_rect));
    }

    fn dropdown_item(&mut self, text: String, other_text: Option<String>, icon: bool, state: &mut State, renderer: &mut Renderer) -> bool {
        self.dropdown_current_y = self.dropdown_next_y;
        let rect = Rect::new(
            self.item_current_x + renderer.style().dropdown_background().padding,
            self.dropdown_current_y,
            self.dropdown_width,
            renderer.text_renderer.text_size(&text, None).y + 3.0,
        );
        self.dropdown_next_y += rect.h;

        // I do different button logic here because they behave slightly differently than normal buttons
        let id = hash_string(&format!("{:?}{}", self.item_next_x, text));

        let mouse_down = state.mouse_down(macroquad::input::MouseButton::Left);

        if state.hot_item.assign_if_none_and(id, state.mouse_in_rect(rect)) {
            if mouse_down {
                state.active_item.assign(id);
            }
        }

        let released = state.hot_item == id && state.active_item == id && !mouse_down;
        if released {
            self.item_current = None;
        }

        let (background, text_col, other_text_col) = renderer.style().menubar(state.hot_item == id);

        if icon {
            renderer.draw(super::renderer::DrawShape::Rect {
                x: rect.x + 2.0,
                y: rect.y + 3.0,
                w: 3.0,
                h: 3.0,
                color: text_col,
            })
        }
        renderer.draw(super::renderer::DrawShape::text(rect.x + 7.0, rect.y + 2.0, text, None, None, None, text_col ));
        if let Some(other_text) = other_text {
            super::elements::text(other_text, None, other_text_col, Align::End(rect.right() - 3.0), Align::Beg(rect.y + 2.0), renderer);
        }
        renderer.draw(super::renderer::DrawShape::rect(rect, background));

        released
    }

    pub fn dropdown(&mut self, text: String, other_text: Option<String>, state: &mut State, renderer: &mut Renderer) -> bool {
        self.dropdown_item(text, other_text, false, state, renderer)
    }
    pub fn dropdown_radio(&mut self, text: String, other_text: Option<String>, qualifier: bool, state: &mut State, renderer: &mut Renderer) -> bool {
        self.dropdown_item(text, other_text, qualifier, state, renderer)
    }
    pub fn dropdown_toggle(&mut self, text: String, other_text: Option<String>, value: &mut bool, state: &mut State, renderer: &mut Renderer) -> bool {
        let pressed = self.dropdown_item(text, other_text, *value, state, renderer);
        if pressed { *value = !*value; }
        pressed
    }
    pub fn dropdown_separator(&mut self, renderer: &mut Renderer) {
        self.dropdown_current_y = self.dropdown_next_y;
        let source = renderer.style().dropdown_separator();
        let dest = Rect::new(
            self.item_current_x + 2.0,
            self.dropdown_current_y,
            self.dropdown_width - 2.0,
            source.h,
        );
        self.dropdown_next_y += dest.h;
        renderer.draw(super::renderer::DrawShape::image_rect(dest, source, None));
    }
}