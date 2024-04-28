use macroquad::{miniquad::window::order_quit, prelude::*};
use minesweeper::Difficulty;
use ui::{menubar::Menubar, minesweeper::MinesweeperUI, spritesheet, DrawShape, PopupReturn, UIState};

pub mod ui;
pub mod minesweeper;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Minesweeper"),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    // Seed the random generation
    macroquad::rand::srand(macroquad::miniquad::date::now() as _);

    let mut ui = UIState::new().await;
    let mut menubar = Menubar::default();
    let mut difficulty = minesweeper::Difficulty::Easy;
    let mut minesweeper_ui = MinesweeperUI::new(difficulty).await;

    // I think windows users aren't gonna be using a window manager, so for them it should auto-resize
    let mut auto_resize = cfg!(target_family = "windows");
    let mut scale = 3;
    let mut resize_scale = Some(3);
    
    loop {
        ui.begin(scale as f32);
        // Menubar takes ownership of 'ui'

        menubar.begin(ui);
        if menubar.item("Game", 91.0) {
            if menubar.dropdown("New Game") {
                menubar.ui().new_game_popup(difficulty, &mut difficulty, &mut minesweeper_ui);
            }
            menubar.dropdown_separator();
            // TODO: Secondary colour (maybe with some control character)
            // TODO: Make it so these display a popup
            for (d, s) in [
                (Difficulty::Easy,   "Easy       ¬¬9¬¬*¬¬9¬¬,  ¬¬¬9"),
                (Difficulty::Normal, "Normal    16*16, ¬¬40"),
                (Difficulty::Hard,   "Hard      30*16, 100")
            ] {
                if menubar.dropdown_radio(s, difficulty == d) {
                    menubar.ui().new_game_popup(d, &mut difficulty, &mut minesweeper_ui);
                }
            }
            if menubar.dropdown_radio("Custom...", matches!(difficulty, Difficulty::Custom(_))) {
                menubar.ui().add_popup(ui::PopupKind::Custom);
            }
            menubar.dropdown_separator();

            menubar.dropdown_checkbox("Resize Window", &mut auto_resize);
            menubar.dropdown_separator();

            if menubar.dropdown("Exit") { order_quit(); }
            menubar.finish_item();
        }
        if menubar.item("Help", 34.0) {
            if menubar.dropdown("About") { 
                menubar.ui().add_popup(ui::PopupKind::About);
            };
            menubar.finish_item();
        }
        if menubar.item("Scale", 22.0) {
            // TODO: Make scaling automatically happen, and depend on how much the game ui fits on the screen or not
            // And then make minefield scaling separate and have panning maybe ?!?!?!
            // TODO: Maybe implement 'fit' button, maybe turn buttons into a slider
            for i in 1..=8 {
                if menubar.dropdown_radio(format!("{:?}{}*", i, if i == 1 {"¬"} else {""}), scale == i) {
                    resize_scale = Some(i);
                }
                if i == 4 { menubar.dropdown_new_column(); }
            }
            menubar.finish_item();
        }
        // Get ui back :3
        ui = menubar.finish();

        // Update and render the popup
        for popup_return in ui.popups(menubar.height()) {
            match popup_return {
                PopupReturn::Custom { width, height, bomb_count } => {
                    ui.new_game_popup(Difficulty::custom(width, height, bomb_count), &mut difficulty, &mut minesweeper_ui);
                }
                PopupReturn::NewGame { difficulty: d } => {
                    difficulty = d;
                    minesweeper_ui.new_game(difficulty);
                },
                _ => {}
            }
        }

        // The actual minefield
        let screen_size = ui.screen_size();
        
        let minefield_start_height = menubar.height() + minesweeper_ui.game_ui_height() - 6.0;
        minesweeper_ui.minefield(&mut ui, screen_size.x / 2.0, (screen_size.y - minefield_start_height) / 2.0 + minefield_start_height, minefield_start_height + 6.0);

        // The game ui, if the button's been pressed reset the game
        if minesweeper_ui.game_ui(&mut ui, menubar.height()) {
            ui.new_game_popup(difficulty, &mut difficulty, &mut minesweeper_ui);
        }
        
        if let Some(new_scale) = resize_scale.take() {
            println!("{:?}", new_scale);
            scale = new_scale;
        }
        
        // Draw the background
        // TODO: Make this not rounded
        ui.draw_queue().push(DrawShape::nineslice(Rect::new(0.0, menubar.height(), screen_size.x, screen_size.y-menubar.height()), spritesheet::BUTTON_IDLE));

        ui.finish();
        next_frame().await;
        // If I don't have this, cpu usage is 100%!
        // TODO: do more research into this
        if !cfg!(target_family = "wasm") {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}