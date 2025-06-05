use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::{
    state::AppState,
    widget::monster::{search::Search, selection::MonsterSelection, MonsterProfileWidget},
};

pub fn render_monster_screen(frame: &mut Frame, chunk: Rect, state: &mut AppState) {
    frame.render_stateful_widget(MonsterProfileWidget, chunk, &mut state.monster_list);
}

pub fn render_monster_selection(frame: &mut Frame, chunk: Rect, state: &mut AppState) {
    let [search, monster_list] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(chunk);

    frame.render_stateful_widget(Search, search, state);
    if let Some((x, y)) = state.tui_state.cursor {
        frame.set_cursor_position((x, y))
    };

    frame.render_stateful_widget(MonsterSelection, monster_list, &mut state.monster_list);
}
