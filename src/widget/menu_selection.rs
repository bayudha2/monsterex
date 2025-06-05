use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{StatefulWidget, Widget},
};

use crate::state::{AppState, MainMenuOption};

pub struct MenuSelection;

impl StatefulWidget for MenuSelection {
    type State = AppState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [monster_rect, quest_rect, weapon_rect, armor_rect] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
        ])
        .areas(area);

        Text::from(vec![
            Line::from(Span::styled(
                "█▀▄▀█ █▀█ █▄░█ █▀ ▀█▀ █▀▀ █▀█",
                Style::default().fg(get_color(MainMenuOption::Monster, &state.main_menu)),
            )),
            Line::from(Span::styled(
                "█░▀░█ █▄█ █░▀█ ▄█ ░█░ ██▄ █▀▄",
                Style::default().fg(get_color(MainMenuOption::Monster, &state.main_menu)),
            )),
        ])
        .render(monster_rect, buf);

        // Text::from(vec![
        //     Line::from(Span::styled(
        //         "█▀█ █░█ █▀▀ █▀ ▀█▀",
        //         Style::default().fg(get_color(MainMenuOption::Quest, &state.main_menu)),
        //     )),
        //     Line::from(Span::styled(
        //         "▀▀█ █▄█ ██▄ ▄█ ░█░",
        //         Style::default().fg(get_color(MainMenuOption::Quest, &state.main_menu)),
        //     )),
        // ])
        // .render(quest_rect, buf);

        // Text::from(vec![
        //     Line::from(Span::styled(
        //         "█░█░█ █▀▀ ▄▀█ █▀█ █▀█ █▄░█",
        //         Style::default().fg(get_color(MainMenuOption::Weapon, &state.main_menu)),
        //     )),
        //     Line::from(Span::styled(
        //         "▀▄▀▄▀ ██▄ █▀█ █▀▀ █▄█ █░▀█",
        //         Style::default().fg(get_color(MainMenuOption::Weapon, &state.main_menu)),
        //     )),
        // ])
        // .render(weapon_rect, buf);

        // Text::from(vec![
        //     Line::from(Span::styled(
        //         "▄▀█ █▀█ █▀▄▀█ █▀█ █▀█",
        //         Style::default().fg(get_color(MainMenuOption::Armor, &state.main_menu)),
        //     )),
        //     Line::from(Span::styled(
        //         "█▀█ █▀▄ █░▀░█ █▄█ █▀▄",
        //         Style::default().fg(get_color(MainMenuOption::Armor, &state.main_menu)),
        //     )),
        // ])
        // .render(armor_rect, buf);
    }
}

fn get_color(src: MainMenuOption, target: &MainMenuOption) -> Color {
    if src == *target {
        Color::White
    } else {
        Color::Rgb(128, 128, 128)
    }
}
