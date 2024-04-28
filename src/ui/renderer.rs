use macroquad::{color::{Color, WHITE}, math::Rect, shapes::draw_rectangle, texture::{draw_texture_ex, DrawTextureParams, FilterMode, Texture2D}};

use super::text_renderer::TextRenderer;

pub enum DrawShape {
    Text { x: f32, y: f32, text: String, color: Color },
    Rect { x: f32, y: f32, w: f32, h: f32, color: Color },
    ImageRect { dest: Rect, source: Rect, color: Color },
}

impl DrawShape {
    pub fn rect(rect: Rect, color: Color) -> Self {
        Self::Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h, color }
    }
    pub fn image_rect(dest: Rect, source: Rect, color: Option<Color>) -> Self {
        Self::ImageRect { dest, source, color: color.unwrap_or(WHITE) }
    }
}

pub struct Renderer {
    pub texture: Texture2D,
    pub text_renderer: TextRenderer,
    pub draw_queue: Vec<DrawShape>
}

impl Renderer {
    pub fn new() -> Renderer {
        let texture = Texture2D::from_file_with_format(include_bytes!("../../resources/spritesheet.png"), None);
        texture.set_filter(FilterMode::Nearest);
        Renderer {
            texture,
            text_renderer: TextRenderer::new(),
            draw_queue: Vec::new(),
        }
    }

    pub fn draw(&mut self, draw_shape: DrawShape) {
        self.draw_queue.push(draw_shape);
    }

    pub fn begin(&mut self) {
        self.draw_queue.clear();
    }
    
    pub fn finish(&mut self) {
        // Draw all of the draw shapes in the queue
        for draw_shape in self.draw_queue.iter().rev() {
            match &draw_shape {
                &DrawShape::Text { x, y, text, color } => self.text_renderer.draw_text(text, *x, *y, *color, None),
                &DrawShape::Rect { x, y, w, h, color } => draw_rectangle(*x, *y, *w, *h, *color),
                &DrawShape::ImageRect { dest, source, color } => {
                    let params = DrawTextureParams {
                        dest_size: Some(dest.size()),
                        source: Some(*source),
                        ..Default::default()
                    };
                    draw_texture_ex(&self.texture, dest.x, dest.y, *color, params)
                }
            }
        }
    }
}