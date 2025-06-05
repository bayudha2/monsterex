use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::monster::monster::MonsterBasicInfo;

pub struct BasicInfo {
    pub basic_info: MonsterBasicInfo,
}

impl BasicInfo {
    pub fn new(bi: MonsterBasicInfo) -> Self {
        Self { basic_info: bi }
    }
}

impl Widget for BasicInfo {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let basic_info_block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Basic Information ")
            .title_style(Style::default().bold());

        let text_type_info = vec!["Type".bold().into(), self.basic_info.m_type.into()];
        let info_type_paragraph = Paragraph::new(text_type_info).centered();

        // TODO: create implementaion for roar, wind p, tremor.
        // like roar.bg_color or roar.color
        // match {}  none, minor, weak, strong, then give color for each level
        let text_roar = vec![
            "Roar".bold().into(),
            self.basic_info.roar.to_string().dark_gray().into(),
        ];
        let info_roar_paragraph = Paragraph::new(text_roar).centered();

        let text_wind_pressure = vec![
            "Wind Pressure".bold().into(),
            self.basic_info.wind_pressure.to_string().dark_gray().into(),
        ];
        let info_wind_p_paragraph = Paragraph::new(text_wind_pressure).centered();

        let text_tremor = vec![
            "Tremor".bold().into(),
            self.basic_info.tremor.to_string().dark_gray().into(),
        ];
        let info_tremor_paragraph = Paragraph::new(text_tremor).centered();

        let mut status_effect_list = vec![];
        for i in 0..self.basic_info.status_effect.len() {
            if self.basic_info.status_effect[i].to_string() == "None" {
                continue;
            }

            status_effect_list.push(Span::styled(
                self.basic_info.status_effect[i].to_string(),
                Style::new().fg(self.basic_info.status_effect[i].color()),
            ));
            let next = self.basic_info.status_effect.get(i + 1);

            // if not empty add space
            if next.is_some() {
                status_effect_list.push(Span::from(" "))
            }
        }

        let text_status_effect = vec![
            "Status Effect".bold().into(),
            Line::from(status_effect_list),
        ];
        let info_status_eff_paragraph = Paragraph::new(text_status_effect).centered();

        // constraint layout percentage
        let (status_effect_perc, roar_wind_tremor_perc) =
            if self.basic_info.status_effect.len() >= 4 {
                (50, 30)
            } else {
                (30, 50)
            };

        let [type_info_rect, roar_info_rect, wind_pressure_info_rect, tremor_info_rect, status_effect_info_rect] =
            Layout::horizontal([
                Constraint::Percentage(20),
                Constraint::Percentage(roar_wind_tremor_perc / 3),
                Constraint::Percentage(roar_wind_tremor_perc / 3),
                Constraint::Percentage(roar_wind_tremor_perc / 3),
                Constraint::Percentage(status_effect_perc),
            ])
            .margin(1)
            .areas(area);

        basic_info_block.render(area, buf);
        info_type_paragraph.render(type_info_rect, buf);
        info_roar_paragraph.render(roar_info_rect, buf);
        info_wind_p_paragraph.render(wind_pressure_info_rect, buf);
        info_tremor_paragraph.render(tremor_info_rect, buf);
        info_status_eff_paragraph.render(status_effect_info_rect, buf);
    }
}
