// The logic of the game

use ggez::{event::EventHandler, GameResult};

use crate::elements::{menubar::MenuBar, text_renderer::TextRenderer};

pub struct MainState {
    text_renderer: TextRenderer,
    menu_bar: MenuBar,
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context) {
        let text_renderer = TextRenderer::new_from_default(ctx);
        
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mouse_pos = ctx.mouse.position();
        let mut mouse_free = false;
        // Update all of the elements
        self.menu_bar.update(&mut mouse_free, mouse_pos);
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        self.menu_bar.draw();
        Ok(())
    }
}