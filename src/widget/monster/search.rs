use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::state::{AppState, InputMode};

pub struct Search;

impl Search {
    fn paragraph(self, scroll: usize, value: &str) -> Paragraph {
        Paragraph::new(value)
            .style(Style::default().fg(Color::Green))
            .scroll((0, scroll as u16))
            .block(Block::bordered())
    }
}

impl StatefulWidget for Search {
    type State = AppState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Block::default().borders(Borders::LEFT).render(area, buf);

        let [wrapper] = Layout::default()
            .horizontal_margin(1)
            .constraints([Constraint::Min(0)])
            .areas(area);

        match state.tui_state.input_mode {
            InputMode::Normal => {
                state.tui_state.cursor = None;
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center)
                    .title(" Press '/' search ");

                Paragraph::new(state.key_handle.input.value())
                    .block(block)
                    .render(wrapper, buf);
            }
            InputMode::Editing => {
                let width = wrapper.width.max(3) - 3;
                let scroll = state.key_handle.input.visual_scroll(width as usize);
                self.paragraph(scroll, state.key_handle.input.value())
                    .render(wrapper, buf);
                state.tui_state.cursor = Some((
                    wrapper.x
                        + ((state.key_handle.input.visual_cursor()).max(scroll) - scroll) as u16
                        + 1,
                    wrapper.y + 1,
                ));
            }
        }
    }
}
