use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::state::AppState;

pub struct BottomNavigation;

impl StatefulWidget for BottomNavigation {
    type State = AppState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let nav_block = Block::default().borders(Borders::TOP);

        let mut basic_nav = vec![
            Span::styled("(q)", Style::default().fg(Color::White).bold()),
            Span::from(" "),
            Span::styled("quit", Style::default().fg(Color::Gray)),
            Span::from(" "),
            Span::styled("(Esc)", Style::default().fg(Color::White).bold()),
            Span::from(" "),
            Span::styled("Back to Main Menu", Style::default().fg(Color::Gray)),
            Span::from(" "),
        ];

        match state.current_screen {
            crate::state::CurrentScreen::Main => {
                basic_nav.extend([
                    Span::styled("(j/k/down/up)", Style::default().fg(Color::White).bold()),
                    Span::from(" "),
                    Span::styled("Choose Menu", Style::default().fg(Color::Gray)),
                ]);
            }
            crate::state::CurrentScreen::Monster => {
                basic_nav.extend([
                    Span::styled("(j/k/down/up)", Style::default().fg(Color::White).bold()),
                    Span::from(" "),
                    Span::styled("Select Monster", Style::default().fg(Color::Gray)),
                    Span::from(" "),
                    Span::styled("(4)", Style::default().fg(Color::White).bold()),
                    Span::from(" "),
                    Span::styled("Toggle Weapon-Element", Style::default().fg(Color::Gray)),
                    Span::from(" "),
                    Span::styled("($)", Style::default().fg(Color::White).bold()),
                    Span::from(" "),
                    Span::styled("Toggle Ailment-Item", Style::default().fg(Color::Gray)),
                    Span::from(" "),
                    Span::styled("(5)", Style::default().fg(Color::White).bold()),
                    Span::from(" "),
                    Span::styled("Toggle Material Drop", Style::default().fg(Color::Gray)),
                    Span::from(" "),
                    Span::styled("(%)", Style::default().fg(Color::White).bold()),
                    Span::from(" "),
                    Span::styled("Toggle Drop Rank", Style::default().fg(Color::Gray)),
                ]);
            }
            _ => {}
        }

        // TODO: update key nav
        let mut nav_text = vec![
            // display current screen
            match state.current_screen {
                crate::state::CurrentScreen::Main => Span::styled(
                    " Main ",
                    Style::default().fg(Color::White).bg(Color::DarkGray),
                ),
                crate::state::CurrentScreen::Monster => Span::styled(
                    " Monster ",
                    Style::default().fg(Color::White).bg(Color::DarkGray),
                ),
                _ => Span::styled(" Other ", Style::default().fg(Color::Green)),
            },
            Span::styled("  ", Style::default()),
        ];

        nav_text.extend(basic_nav);

        Paragraph::new(Line::from(nav_text))
            .block(nav_block)
            .render(area, buf);
    }
}
