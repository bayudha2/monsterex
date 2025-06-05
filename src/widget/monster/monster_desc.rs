use crate::{monster::MonsterDescText, state::monster::ScrollableParagraphState};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, StatefulWidget, Widget,
    },
};

use super::get_lines;

pub struct DescMonster(pub MonsterDescText);

impl StatefulWidget for DescMonster {
    type State = ScrollableParagraphState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let mut monster_abi_desc = vec![];
        let mut count = 0;

        let ori_text = get_lines(&self.0.original, area.width as usize - 4);
        ori_text.into_iter().for_each(|d| {
            monster_abi_desc.push(Line::from(Span::styled(
                d,
                Style::default().italic().white(),
            )))
        });

        monster_abi_desc.push(Line::from(""));

        for (i, val) in self.0.desc.into_iter().enumerate() {
            if i > 0 {
                monster_abi_desc.push(Line::from(""));
            }

            let desc = get_lines(&val, area.width as usize - 4);
            count += desc.len();
            desc.into_iter()
                .for_each(|d| monster_abi_desc.push(Line::from(d)));
        }

        state.set_height(if count > (area.height as usize).saturating_sub(4) {
            count
        } else {
            0
        });

        Paragraph::new(monster_abi_desc)
            .block(
                Block::bordered()
                    .title(" Ability ")
                    .title_style(Style::default().bold())
                    .border_type(BorderType::Rounded),
            )
            .scroll((state.position as u16, 0))
            .render(area, buf);

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().bg(Color::DarkGray))
            .render(area, buf, &mut state.scrollbar_state);
    }
}
