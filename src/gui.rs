pub mod text_renderer;
pub mod button;
pub mod menu_bar;
pub mod dropdown;
use ggez::{glam::Vec2, graphics::{Canvas, DrawParam, Image, InstanceArray, Rect}};
pub use text_renderer::TextRenderer;
pub use button::Button;
pub use menu_bar::MenuBar;
pub use dropdown::Dropdown;

// Draws a nine-slice texture, from a batch image, to a given canvas
pub fn draw_nineslice(canvas: &mut Canvas, batch_img: &mut InstanceArray, src: Rect, slice_size: f32, dest: Rect) {
    let s = slice_size; // Brevity
    // Generate an array of parts to draw.
    // (source rect, destination rect), both of these are localised.
    let middle_size = Vec2::new((dest.w-s*2.0)/(src.w-s*2.0), (dest.h-s*2.0)/(src.h-s*2.0));
    let parts: [(Rect, Rect); 9] = [
        // ===== Middle ===== //
        (Rect::new(s, s, src.w-s*2.0, src.h-s*2.0), Rect::new(s, s, middle_size.x, middle_size.y)),
        // ===== Edges ===== //
        /*Left  */ (Rect::new(0.0,     s, s, src.h-2.0*s), Rect::new(0.0,      s, 1.0, middle_size.y)),
        /*Right */ (Rect::new(src.w-s, s, s, src.h-2.0*s), Rect::new(dest.w-s, s, 1.0, middle_size.y)),
        /*Top   */ (Rect::new(s, 0.0,     src.w-2.0*s, s), Rect::new(s, 0.0,      middle_size.x, 1.0)),
        /*Bottom*/ (Rect::new(s, src.h-s, src.w-2.0*s, s), Rect::new(s, dest.h-s, middle_size.x, 1.0)),
        // ===== Corners ===== //
        /*TL*/ (Rect::new(0.0,     0.0,     s, s), Rect::new(0.0,      0.0,      1.0, 1.0)),
        /*TR*/ (Rect::new(src.w-s, 0.0,     s, s), Rect::new(dest.w-s, 0.0,      1.0, 1.0)),
        /*BL*/ (Rect::new(0.0,     src.h-s, s, s), Rect::new(0.0,      dest.h-s, 1.0, 1.0)),
        /*BR*/ (Rect::new(src.w-s, src.h-s, s, s), Rect::new(dest.w-s, dest.h-s, 1.0, 1.0)),
    ];
    // Draw each of the parts
    let image = &batch_img.image().clone();
    batch_img.set(
        parts.iter().map(|(s, d)| DrawParam::new()
            .src(normalize_rect(Rect::new(s.x + src.x,  s.y + src.y,  s.w, s.h), image))
            .dest_rect(Rect::new(d.x + dest.x, d.y + dest.y, d.w, d.h)))
    );
    canvas.draw(batch_img, DrawParam::new());
}

// Fits a rect to a given image
pub fn normalize_rect(rect: Rect, image: &Image) -> Rect {
    Rect::new(
        rect.x / image.width() as f32, rect.y / image.height() as f32,
        rect.w / image.width() as f32, rect.h / image.height() as f32,)
}
