use std::path::PathBuf;

use ansi_to_tui::IntoText;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::{
    monster::monster::{MonsterElements, MonsterName},
    ui::centered_rect,
};

pub struct NameIcon {
    pub name: MonsterName,
    pub monster_el: Vec<MonsterElements>,
    pub asset_path: PathBuf,
}

impl NameIcon {
    pub fn new(name: MonsterName, monster_el: Vec<MonsterElements>, asset_path: PathBuf) -> Self {
        Self {
            name,
            monster_el,
            asset_path,
        }
    }
}

impl Widget for NameIcon {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [monster_name_rect, monster_icon_rect] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(area);

        let mut monster_info_title = vec![
            Span::styled(self.name.name, Style::default().bold().italic()),
            Span::styled(" | ", Style::default().fg(Color::White)),
            Span::styled(self.name.aka, Style::default().bold().italic()),
        ];

        if !self.monster_el.is_empty() {
            match self.monster_el[0] {
                MonsterElements::None => {}
                _ => {
                    monster_info_title.extend([
                        Span::from(" "),
                        Span::styled(
                            format!(" {} {} ", self.monster_el[0].icon(), self.monster_el[0]),
                            Style::default()
                                .bg(self.monster_el[0].color())
                                .fg(Color::White),
                        ),
                    ]);
                }
            }
        }

        if let Ok(buffer) = std::fs::read(self.asset_path) {
            let ansi_text = buffer.into_text();

            Paragraph::new(Line::from(monster_info_title)).render(monster_name_rect, buf);

            if let Ok(ansi_text) = ansi_text {
                Paragraph::new(ansi_text.clone()).render(
                    centered_rect(
                        monster_icon_rect,
                        Constraint::Length(ansi_text.width() as u16),
                        Constraint::Length(ansi_text.height() as u16),
                    ),
                    buf,
                );
            }
        }
    }
}
