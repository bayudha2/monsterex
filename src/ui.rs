use crate::{
    screen::{
        main_s::render_main_screen,
        monster::{render_monster_screen, render_monster_selection},
    },
    state::AppState,
    widget::bottom_nav::BottomNavigation,
};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &mut AppState) {
    let area = frame.area();
    let [main_chunk, bottom_chunk] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).areas(area);

    // let nc = format!("width: {}", area.width);

    let constraint = [Constraint::Percentage(80), Constraint::Percentage(20)];
    let [left_chunk, right_chunk] = Layout::horizontal(constraint).areas(main_chunk);

    match app.current_screen {
        crate::state::CurrentScreen::Main => render_main_screen(frame, left_chunk, app),
        crate::state::CurrentScreen::Monster => {
            render_monster_screen(frame, left_chunk, app);
            render_monster_selection(frame, right_chunk, app);
        }
        _ => todo!(),
    }

    // render Bottom Navigation Info
    frame.render_stateful_widget(BottomNavigation, bottom_chunk, app);
}

pub fn centered_popup_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2), // e.g percent_y = 50 make sense
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn centered_rect(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
