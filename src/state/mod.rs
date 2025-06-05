pub mod monster;
pub mod tui;

use monster::MonsterListState;
use tui_input::Input;

use self::tui::TuiState;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum MainMenuOption {
    #[default]
    Monster,
    Quest,
    Weapon,
    Armor,
}

impl MainMenuOption {
    pub fn next(self) -> Self {
        match self {
            MainMenuOption::Monster => MainMenuOption::Quest,
            MainMenuOption::Quest => MainMenuOption::Weapon,
            MainMenuOption::Weapon => MainMenuOption::Armor,
            MainMenuOption::Armor => MainMenuOption::Monster,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            MainMenuOption::Monster => MainMenuOption::Armor,
            MainMenuOption::Quest => MainMenuOption::Monster,
            MainMenuOption::Weapon => MainMenuOption::Quest,
            MainMenuOption::Armor => MainMenuOption::Weapon,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum CurrentScreen {
    #[default]
    Main,
    Monster,
    Quest,
    Weapon,
    Armor,
}

#[derive(Default)]
pub struct KeyHandleState {
    pub input: Input,
}

#[derive(Default)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default)]
pub struct AppState {
    pub tui_state: TuiState,
    pub key_handle: KeyHandleState,
    pub current_screen: CurrentScreen,
    pub main_menu: MainMenuOption,
    pub monster_list: MonsterListState,
}

impl AppState {
    pub fn reset(&mut self) {
        self.tui_state.input_mode = InputMode::Normal;
        self.key_handle.input.reset();
        self.monster_list.set_list_filter(String::from(""));
    }
}
