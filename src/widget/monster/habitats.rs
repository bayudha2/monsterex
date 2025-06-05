use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    symbols,
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::{
    monster::monster::MonsterHabitatData, state::monster::ChangeableHabitatPageState,
    ui::centered_rect,
};

pub struct HabitatsDetailInfo<'a> {
    title: &'a str,
    value: &'a str,
}

impl<'a> HabitatsDetailInfo<'a> {
    pub fn new(title: &'a str, value: &'a str) -> Self {
        Self { title, value }
    }
}

impl<'a> Widget for HabitatsDetailInfo<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [title_chunk, data_chunk] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(1)])
                .margin(1)
                .areas(area);

        Paragraph::new(Span::styled(self.title, Style::default().bold())).render(title_chunk, buf);
        Paragraph::new(Span::styled(
            self.value,
            Style::default()
                .bold()
                .fg(if self.title.contains("Region") {
                    Color::Rgb(54, 127, 222)
                } else {
                    Color::Rgb(199, 159, 0)
                }),
        ))
        .render(
            centered_rect(
                data_chunk,
                Constraint::Length(self.value.len() as u16),
                Constraint::Length(1),
            ),
            buf,
        );
    }
}

// pub struct HabitatsMonster {
//     habitats: Vec<MonsterHabitatData>,
// }

// impl HabitatsMonster {
//     pub fn new(mh: Vec<MonsterHabitatData>) -> Self {
//         Self { habitats: mh }
//     }
// }

pub struct HabitatsMonster(pub Vec<MonsterHabitatData>);

impl StatefulWidget for HabitatsMonster {
    type State = ChangeableHabitatPageState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(48), Constraint::Percentage(52)])
                .areas(area);

        let [top_left, bottom_left] =
            Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)]).areas(left);

        let [top_right, bottom_right] =
            Layout::vertical([Constraint::Percentage(45), Constraint::Percentage(55)]).areas(right);

        let top_left_block = Block::new()
            .borders(Borders::TOP | Borders::LEFT)
            .border_type(BorderType::Rounded)
            .title_style(Style::default().bold())
            .title(" Habitats ");

        let top_right_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.horizontal_down,
            top_right: symbols::line::ROUNDED_TOP_RIGHT,
            ..symbols::border::PLAIN
        };

        let top_right_block = Block::new()
            .border_set(top_right_border_set)
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT);

        let bottom_left_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            bottom_left: symbols::line::ROUNDED_BOTTOM_LEFT,
            ..symbols::border::PLAIN
        };

        let bottom_left_block = Block::new()
            .border_set(bottom_left_border_set)
            .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

        let bottom_right_block_border_set = symbols::border::Set {
            bottom_left: symbols::line::NORMAL.horizontal_up,
            top_right: symbols::line::NORMAL.vertical_left,
            top_left: symbols::line::CROSS,
            bottom_right: symbols::line::ROUNDED_BOTTOM_RIGHT,
            ..symbols::border::PLAIN
        };

        let bottom_right_block = Block::new()
            .border_set(bottom_right_block_border_set)
            .borders(Borders::ALL);

        top_left_block.render(top_left, buf);
        bottom_left_block.render(bottom_left, buf);
        top_right_block.render(top_right, buf);
        bottom_right_block.render(bottom_right, buf);

        if !self.0.is_empty() {
            let region = &self.0[state.habitat_current_page as usize].region;

            let starting_area = self.0[state.habitat_current_page as usize]
                .starting_area
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            let visited_areas = &self.0[state.habitat_current_page as usize]
                .visited_area
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            let resting_area = &self.0[state.habitat_current_page as usize]
                .resting_area
                .to_string();

            HabitatsDetailInfo::new(
                &format!(
                    "Region ({}/{})",
                    state.habitat_current_page + 1,
                    state.habitat_total_page,
                ),
                region,
            )
            .render(top_left, buf);
            HabitatsDetailInfo::new("Starting Area", &starting_area).render(top_right, buf);
            HabitatsDetailInfo::new("Visited Areas", visited_areas).render(bottom_left, buf);
            HabitatsDetailInfo::new("Resting Areas", resting_area).render(bottom_right, buf);
        }
    }
}
