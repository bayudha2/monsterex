use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, StatefulWidget, Widget},
};

use crate::state::monster::MonsterListState;

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::Rgb(29, 46, 69))
    .add_modifier(Modifier::BOLD);

pub struct MonsterSelection;

impl StatefulWidget for MonsterSelection {
    type State = MonsterListState;

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

        let items: Vec<ListItem> = state
            .list_items()
            .iter()
            .map(|item| {
                let text = Text::from(item.name.name.to_string());
                ListItem::new(text)
            })
            .collect();

        StatefulWidget::render(
            List::new(items)
                .highlight_style(SELECTED_STYLE)
                .highlight_symbol("> ")
                .highlight_spacing(HighlightSpacing::Always),
            wrapper,
            buf,
            &mut state.list_state,
        );
    }
}
