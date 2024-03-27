// The logic of the game

use ggez::{event::{EventHandler, MouseButton}, graphics, mint::Point2, GameResult};

use crate::elements::{self, menubar::MenuBar, text_renderer::TextRenderer, MouseAction};

pub struct MainState {
    text_renderer: TextRenderer,
    menu_bar: MenuBar,
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context) -> MainState {
        let text_renderer = TextRenderer::new_from_default(ctx);

        let menu_bar = MenuBar::new(&text_renderer);

        MainState {
            text_renderer,
            menu_bar,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mouse_pos = ctx.mouse.position();
        let mouse_action = (
            if      ctx.mouse.button_just_pressed(MouseButton::Left)   { MouseAction::Press }
            else if ctx.mouse.button_just_released(MouseButton::Left)  { MouseAction::Release }
            else { MouseAction::None },
            if      ctx.mouse.button_just_pressed(MouseButton::Right)  { MouseAction::Press }
            else if ctx.mouse.button_just_released(MouseButton::Right) { MouseAction::Release }
            else { MouseAction::None },
        );
        let mut mouse_free = true;
        // Update all of the elements
        self.menu_bar.update(&mut mouse_free, mouse_pos, &mouse_action);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, elements::TEXT_DISABLED);
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());

        self.menu_bar.draw(&mut canvas, &mut self.text_renderer);

        canvas.finish(ctx)
    }
}