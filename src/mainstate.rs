// The logic of the game

use ggez::{event::{EventHandler, MouseButton}, glam::Vec2, graphics, mint::Point2, winit::dpi::LogicalSize, Context, GameResult};
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
        self.minesweeper_element.update(&mut mouse_free, mouse_pos, mouse_action);

        // Do things depending on what's happened
        if self.menu_bar.pressed(0) {
            let window_middle = window_size(ctx) / 2.0;
            // Makes a new minesweeper game
            self.minesweeper_element.new_game(ctx, crate::minesweeper::Difficulty::Hard);
            // How big the minesweeper game is, plus some padding
            const TOP_PADDING: f32 = 2.0;
            let minesweeper_dimensions = self.minesweeper_element.size() + Vec2::new(10.0, 5.0 + TOP_PADDING);
            // Temp
            let menu_bar_height = 10.0;
            let menu_bar_width = 10.0;
            // Let the new window size be however big the minesweeper game is plus padding, plus the menubar height and the top bar height
            // Making sure that if the menu bar is bigger there's space for it
            let new_window_size = (minesweeper_dimensions + Vec2::new(0.0, menu_bar_height + 21.0)).max(Vec2::new(menu_bar_width, 0.0));
            ctx.gfx.window().set_inner_size(LogicalSize::new(new_window_size.x, new_window_size.y));
            
            let minesweeper_pos = Vec2::new((new_window_size.x - self.minesweeper_element.size().x) / 2.0, (new_window_size.y - self.minesweeper_element.size().y) / 2.0 + 21.0 + TOP_PADDING);
            self.minesweeper_element.set_pos(minesweeper_pos);
        }
        if self.menu_bar.pressed(1) {
            ctx.request_quit();
        }
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