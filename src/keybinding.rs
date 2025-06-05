use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::state::{AppState, CurrentScreen, InputMode, MainMenuOption};

#[derive(Default, Eq, PartialEq)]
pub enum KeyHandleResult {
    #[default]
    Continue,
    Exit,
}

impl KeyHandleResult {
    pub fn exit(&self) -> bool {
        *self == KeyHandleResult::Exit
    }
}

pub fn handle_key(event: KeyEvent, app: &mut AppState) -> KeyHandleResult {
    if event.kind == KeyEventKind::Release {
        return KeyHandleResult::Continue;
    }

    match app.tui_state.input_mode {
        crate::state::InputMode::Normal => on_normal(event, app),
        crate::state::InputMode::Editing => on_edit(event, app),
    }
}

fn on_edit(event: KeyEvent, app: &mut AppState) -> KeyHandleResult {
    match event.code {
        KeyCode::Esc => {
            app.reset();
            app.monster_list.reset();
        }
        KeyCode::Enter => app.tui_state.input_mode = InputMode::Normal,
        _ => {
            // handle normal key
            app.key_handle.input.handle_event(&Event::Key(event));
            app.monster_list.filter_query.clear();
            app.monster_list
                .set_list_filter(app.key_handle.input.value().to_string());
            app.monster_list.reset();
        }
    };

    KeyHandleResult::Continue
}
fn on_normal(event: KeyEvent, app: &mut AppState) -> KeyHandleResult {
    let KeyEvent {
        code,
        modifiers,
        kind: _,
        state: _,
    }: KeyEvent = event;

    match app.current_screen {
        CurrentScreen::Main => {
            match (code, modifiers) {
                // handle Ctrl + char Press
                (code, KeyModifiers::CONTROL) => match code {
                    KeyCode::Char(_) => todo!(),
                    _ => return KeyHandleResult::Continue,
                },

                // handle normal key
                (code, _) => match code {
                    KeyCode::Char('q') => return KeyHandleResult::Exit,

                    KeyCode::Down | KeyCode::Char('j') => app.main_menu = app.main_menu.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.main_menu = app.main_menu.prev(),
                    KeyCode::Enter => app.current_screen = select_screen(app),
                    _ => (),
                },
            }
        }
        CurrentScreen::Monster => match code {
            KeyCode::Char('q') => return KeyHandleResult::Exit,
            KeyCode::Down | KeyCode::Char('j') => {
                // app.monster_list.desc_scrollbar_state.scroll_down();
                // app.monster_list.monster_on_quest_list_state.next();
                // app.monster_list.monster_weakness_tab_state.next();
                // app.monster_list.monster_drops_tab_state.next();
                app.monster_list.reset();
                app.monster_list.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                // app.monster_list.desc_scrollbar_state.scroll_up();
                // app.monster_list.monster_on_quest_list_state.prev();
                // app.monster_list.monster_weakness_tab_state.prev();
                // app.monster_list.monster_drops_tab_state.prev();
                app.monster_list.reset();
                app.monster_list.prev();
            }

            KeyCode::Char('4') => {
                app.monster_list
                    .monster_weakness_tab_state
                    .toggle_selected_weapon_elm_tab();
            }
            KeyCode::Char('$') => {
                app.monster_list
                    .monster_weakness_tab_state
                    .toggle_selected_ailment_item_tab();
            }
            KeyCode::Char('5') => {
                app.monster_list
                    .monster_drops_tab_state
                    .toggle_selected_source_tab();
            }
            KeyCode::Char('%') => {
                app.monster_list
                    .monster_drops_tab_state
                    .toggle_selected_rank_tab();
            }
            KeyCode::Char('/') => app.tui_state.input_mode = InputMode::Editing,
            // KeyCode::Left | KeyCode::Char('h') => app.monster_list.habitat_page_state.prev(),
            // KeyCode::Right | KeyCode::Char('l') => app.monster_list.habitat_page_state.next(),
            KeyCode::Esc => {
                app.reset();
                app.current_screen = CurrentScreen::Main;
            }

            _ => {}
        },
        _ => {}
    }

    KeyHandleResult::Continue
}

fn select_screen(app: &mut AppState) -> CurrentScreen {
    match app.main_menu {
        MainMenuOption::Monster => {
            app.monster_list.reset();
            CurrentScreen::Monster
        }
        MainMenuOption::Quest => CurrentScreen::Quest,
        MainMenuOption::Weapon => CurrentScreen::Weapon,
        MainMenuOption::Armor => CurrentScreen::Armor,
    }
}
