// The logic of the game

use ggez::{event::{EventHandler, MouseButton}, glam::Vec2, graphics, mint::Point2, Context, GameResult};
use rand::Rng;

use crate::elements::{self, menubar::MenuBar, minesweeper_element::MinesweeperElement, text_renderer::TextRenderer, MouseAction};

pub struct MainState {
    text_renderer: TextRenderer,
    menu_bar: MenuBar,
    minesweeper_element: MinesweeperElement,
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context) -> MainState {
        let window_size = window_size(ctx);

        let minesweeper_element = MinesweeperElement::new(ctx, crate::minesweeper::Difficulty::Easy, window_size / 2.0);

        let text_renderer = TextRenderer::new_from_default(ctx);

        let menu_bar = MenuBar::new(&text_renderer);

        MainState {
            text_renderer,
            menu_bar,
            minesweeper_element,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mouse_pos = Vec2::new(ctx.mouse.position().x, ctx.mouse.position().y);
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

        if self.menu_bar.pressed(0) {
            let window_middle = window_size(ctx) / 2.0;
            self.minesweeper_element.new_game(
                ctx,
                crate::minesweeper::Difficulty::Hard,
                window_middle,
            );
        }
        if self.menu_bar.pressed(1) {
            ctx.request_quit();
        }

        self.minesweeper_element.update(&mut mouse_free, mouse_pos, mouse_action);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let window_size = window_size(ctx);

        let mut canvas = graphics::Canvas::from_frame(ctx, elements::TEXT_DISABLED);
        // canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());

        let _ = self.minesweeper_element.render_minefield(ctx);
        self.minesweeper_element.draw(&mut canvas);
        self.menu_bar.draw(&mut canvas, &mut self.text_renderer);

        canvas.finish(ctx)
    }
}

pub fn window_size(ctx: &mut Context) -> Vec2 {
    Vec2::new(
        ctx.gfx.window().inner_size().width  as f32,
        ctx.gfx.window().inner_size().height as f32,
    )
}