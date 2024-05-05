use macroquad::{color::{Color, WHITE}, math::{vec2, Rect}, shapes::draw_rectangle, texture::{draw_texture_ex, DrawTextureParams, FilterMode, Texture2D}};

use self::text_renderer::TextRenderer;

use super::{menubar::Menubar, spritesheet::{self, Nineslice}, state::State, Round};

pub mod text_renderer;

pub enum DrawShape {
    Text { x: f32, y: f32, text: String, color: Color },
    TextCaret { x: f32, y: f32, color: Color },
    Rect { x: f32, y: f32, w: f32, h: f32, color: Color },
    Image { x: f32, y: f32, source: Rect, color: Color },
    ImageRect { dest: Rect, source: Rect, color: Color },
    Nineslice { dest: Rect, source: Rect, padding: f32 },
    // Minefield { x: f32, y: f32, },
}

impl DrawShape {
    pub fn text(x: f32, y: f32, text: String, color: Color) -> Self {
        Self::Text { x, y, text, color }
    }
    pub fn text_caret(x: f32, y: f32, color: Color) -> Self {
        Self::TextCaret { x, y, color }
    }
    pub fn rect(rect: Rect, color: Color) -> Self {
        Self::Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h, color }
    }
    pub fn image(x: f32, y: f32, source: Rect, color: Option<Color>) -> Self {
        Self::Image { x, y, source, color: color.unwrap_or(WHITE)}
    }
    pub fn image_rect(dest: Rect, source: Rect, color: Option<Color>) -> Self {
        Self::ImageRect { dest, source, color: color.unwrap_or(WHITE) }
    }
    pub fn nineslice(dest: Rect, source: Nineslice) -> Self {
        Self::Nineslice { dest, source: source.rect, padding: source.padding }
    }
    pub fn round(&mut self) {
        match self {
            Self::Text      { x, y, .. }              => { *x = x.round(); *y = y.round(); }
            Self::TextCaret { x, y, .. }              => { *x = x.round(); *y = y.round(); }
            Self::Rect      { x, y, w, h, ..  }       => { *x = x.round(); *y = y.round(); *w = w.round(); *h = h.round(); }
            Self::Image     { x, y, source, .. }      => { *x = x.round(); *y = y.round(); *source = source.round(); }
            Self::ImageRect { dest, source, .. }      => { *dest = dest.round(); *source = source.round(); }
            Self::Nineslice { dest, source, padding } => { *dest = dest.round(); *source = source.round(); *padding = padding.round(); }
            // Self::Minefield { x, y }                  => { *x = x.round(); *y = y.round(); }
        }
    }
}

pub struct Renderer {
    pub texture: Texture2D,
    pub text_renderer: TextRenderer,
    pub draw_queue: Vec<DrawShape>,
    pub caret_timer: f32,
}

impl Renderer {
    pub fn new() -> Renderer {
        let texture = Texture2D::from_file_with_format(include_bytes!("../../../resources/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);
        Renderer {
            texture,
            text_renderer: TextRenderer::new(),
            draw_queue: Vec::new(),
            caret_timer: 0.0,
        }
    }

    pub fn draw(&mut self, draw_shape: DrawShape) {
        self.draw_queue.push(draw_shape);
    }

    pub fn draw_iter(&mut self, draw_shapes: impl Iterator<Item = DrawShape>) {
        self.draw_queue.extend(draw_shapes);
    }

    pub fn begin(&mut self, state: &State) {
        self.draw_queue.clear();
        self.caret_timer = match state.text_field {
            None => 0.0,
            _ => (self.caret_timer + macroquad::time::get_frame_time()).rem_euclid(1.0),
        };
    }
    
    pub fn finish(&mut self, state: &State, menubar: &Menubar) {
        // Draw the unrounded background
        let background_rect = Rect::new(
            0.0,
            menubar.height(),
            state.screen_size().x,
            state.screen_size().y - menubar.height(),
        );
        self.draw_shape(&DrawShape::nineslice(background_rect, spritesheet::BACKGROUND));
        
        // Draw all of the rounded draw shapes in the queue
        self.draw_queue.iter_mut().for_each(|d| d.round());
        for draw_shape in self.draw_queue.iter().rev() {
            self.draw_shape(&draw_shape);
        }
    }

    fn draw_shape(&self, draw_shape: &DrawShape) {
        match &draw_shape {
            &DrawShape::Text { x, y, text, color } => self.text_renderer.draw_text(text, *x, *y, *color, None),
            &DrawShape::TextCaret { x, y, color }  => if self.caret_timer < 0.5 { self.text_renderer.draw_text(&"|".to_owned(), *x, *y, *color, None) },
            &DrawShape::Rect { x, y, w, h, color } => draw_rectangle(*x, *y, *w, *h, *color),
            &DrawShape::Image { x, y, source, color } => {
                let params = DrawTextureParams {
                    source: Some(*source),
                    ..Default::default()
                };
                draw_texture_ex(&self.texture, *x, *y, *color, params)
            },
            &DrawShape::ImageRect { dest, source, color } => {
                let params = DrawTextureParams {
                    dest_size: Some(dest.size()),
                    source: Some(*source),
                    ..Default::default()
                };
                draw_texture_ex(&self.texture, dest.x, dest.y, *color, params)
            },
            &DrawShape::Nineslice { dest, source, padding } => {
                let dest_parts   = calculate_nineslice_parts(*dest,   *padding);
                let source_parts = calculate_nineslice_parts(*source, *padding);

                for (d, s) in dest_parts.iter().zip(source_parts) {
                    let params = DrawTextureParams {
                        dest_size: Some(d.size()),
                        source: Some(s),
                        ..Default::default()
                    };
                    draw_texture_ex(&self.texture, d.x, d.y, WHITE, params);
                }
            },
        }
    }
}

fn calculate_nineslice_parts(rect: Rect, pad: f32) -> [Rect; 9] {
    let pad = pad.round();
    let middle_size = vec2(rect.w, rect.h) - pad*2.0;
    [
        // Middle
        Rect::new(rect.x + pad, rect.y + pad,  middle_size.x, middle_size.y),
        // Edges
        Rect::new(rect.x,                rect.y + pad,  pad, middle_size.y), // Left
        Rect::new(rect.x + rect.w - pad, rect.y + pad,  pad, middle_size.y), // Right
        Rect::new(rect.x + pad, rect.y,                 middle_size.x, pad), // Top
        Rect::new(rect.x + pad, rect.y + rect.h - pad,  middle_size.x, pad), // Bottom
        // Corners
        Rect::new(rect.x,                rect.y,                pad, pad), // Top left
        Rect::new(rect.x + rect.w - pad, rect.y,                pad, pad), // Top right
        Rect::new(rect.x,                rect.y + rect.h - pad, pad, pad), // Bottom left
        Rect::new(rect.x + rect.w - pad, rect.y + rect.h - pad, pad, pad), // Bottom right
    ]
}