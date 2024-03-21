pub mod button;
pub mod dropdown;
pub mod menu_bar;
pub mod popup;
pub mod text_renderer;
pub use button::Button;
pub use dropdown::Dropdown;
use ggez::{glam::Vec2, graphics::Rect};
pub use menu_bar::MenuBar;
pub use popup::Popup;
pub use text_renderer::TextRenderer;

#[derive(Debug)]
enum DropdownGroupType {
    None,
    PickOne,
}

const MENU_OPTIONS: &[(&str, &[(DropdownGroupType, &[&str])])] = &[
    (
        "Game",
        &[
            (DropdownGroupType::None, &["New Game"]),
            (
                DropdownGroupType::PickOne,
                &[
                    "Easy    10*10, 9",
                    "Normal 15*13,40",
                    "Hard   30*16,99",
                    "Custom...",
                ],
            ),
            // (DropdownGroupType::ToggleEach, &["Question marks"]), // Maybe another day, who needs question marks?
            (DropdownGroupType::None, &["Exit"]),
        ],
    ),
    (
        "Scale",
        &[(
            DropdownGroupType::PickOne,
            &[
                " 1x ", " 2x ", " 3x ", " 4x ", " 5x ", " 6x ", " 7x ", " 8x ",
            ],
        )],
    ),
    (
        "Help",
        &[(DropdownGroupType::None, &["How to play", "About"])],
    ),
];

enum DropdownGroupInfo {
    None,
    PickOne { selected: usize },
}

#[derive(PartialEq, Clone, Copy)]
pub enum MousePressMode {
    None,
    Down,
    Up,
}
pub struct Gui {
    // menubar: [Button; 3],

    // The menu bar button, a vector of each button in the dropdown
    menubar: [(Button, Vec<DropdownGroupInfo>, Vec<(Button, usize)>); MENU_OPTIONS.len()],

    pub menu_bar: MenuBar,
    pub popup: Option<Popup>,
    hovered_on_gui: bool,
}

// TODO: !!! Message boxes, number fields for customisation

impl Gui {
    pub fn new(menu_bar: MenuBar) -> Gui {
        println!("{:?}", MENU_OPTIONS);

        let menubar: [(Button, Vec<DropdownGroupInfo>, Vec<Button>); MENU_OPTIONS.len()];

        todo!();
        // Gui { menu_bar, hovered_on_gui: false, popup: None }
    }
    pub fn update(&mut self, mouse_pos: Vec2, mouse_mode: MousePressMode) {
        self.menu_bar.update(mouse_pos, mouse_mode);
        if let Some(popup) = self.popup.as_mut() {
            popup.update(mouse_pos);
        }

        if self.hovering() {
            self.hovered_on_gui = true;
        }
    }
    pub fn hovering(&self) -> bool {
        // TODO: make better?
        self.menu_bar.hovering_over || self.popup.as_ref().is_some_and(|p| p.hovering_over)
    }
    pub fn check_and_reset_hover_flag(&mut self) -> bool {
        if self.hovered_on_gui {
            self.hovered_on_gui = false;
            return true;
        } else {
            false
        }
    }

    pub fn popup(&mut self, kind: popup::PopupKind, window_middle: Vec2) {
        self.popup = Some(Popup::new(kind, window_middle))
    }
}
