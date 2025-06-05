use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, Scrollbar,
        ScrollbarOrientation, StatefulWidget,
    },
};

use crate::{monster::monster::MonsterQuestData, state::monster::MonsterOnQuestListState};

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::Rgb(29, 46, 69))
    .add_modifier(Modifier::BOLD);

pub struct QuestMonsterList(pub Vec<MonsterQuestData>);

impl StatefulWidget for QuestMonsterList {
    type State = MonsterOnQuestListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        state.set_scrollbar_height(area);

        let [layout] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .areas(area);

        let items: Vec<ListItem> = self
            .0
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let quest_type_text = format!("[ {} ]", item.quest_type);
                let quest_level_text = format!("[ {} ]", item.level);
                let quest_name_text = item.name.to_string();

                let line = Line::from(vec![
                    quest_level_text.into(),
                    quest_type_text.into(),
                    " ".into(),
                    quest_name_text.fg(if let Some(i_selected) = state.list_state.selected() {
                        if i == i_selected {
                            Color::Rgb(54, 127, 222)
                        } else {
                            Color::White
                        }
                    } else {
                        Color::White
                    }),
                ]);
                let text = Text::from(line);

                ListItem::new(text)
            })
            .collect();

        List::new(items)
            .block(
                Block::default()
                    .title(" Quest Appearances ")
                    .title_style(Style::default().bold())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .render(layout, buf, &mut state.list_state);

        Scrollbar::new(ScrollbarOrientation::VerticalRight).render(
            layout,
            buf,
            &mut state.scrollbar_state,
        );
    }
}
