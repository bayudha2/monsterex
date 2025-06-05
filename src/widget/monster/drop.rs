use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation,
        StatefulWidget, Table, Widget,
    },
};

use crate::{
    monster::monster::{MaterialDrop, MaterialDropWithPart, MonsterMaterialsDrop},
    state::monster::{MaterialSourceTab, MonsterDropRankTab, MonsterDropTabState},
};

use super::get_lines;

const SELECTED_ROW_COLOR: Color = Color::Rgb(54, 127, 222);

pub struct MonsterDropTable(pub MonsterMaterialsDrop, pub MonsterMaterialsDrop);

impl StatefulWidget for MonsterDropTable {
    type State = MonsterDropTabState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let block_title = Block::new()
            .title(" Drops ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        block_title.render(area, buf);

        let [drop_wrapper_chunk] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .margin(1)
            .areas(area);

        let [drop_rank_chunk] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .margin(1)
            .areas(drop_wrapper_chunk);

        let [drop_chunk] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .margin(1)
            .areas(drop_rank_chunk);

        state.set_scrollbar_height(drop_chunk);

        let title_rank = Line::from(vec![
            Span::from(" [ "),
            Span::styled(
                "Low",
                Style::default().fg(match state.selected_rank_tab {
                    MonsterDropRankTab::LowRank => SELECTED_ROW_COLOR,
                    _ => Color::White,
                }),
            ),
            Span::from(" | "),
            Span::styled(
                "High",
                Style::default().fg(match state.selected_rank_tab {
                    MonsterDropRankTab::HighRank => SELECTED_ROW_COLOR,
                    _ => Color::White,
                }),
            ),
            Span::from(" ] "),
        ]);

        let title_source = Line::from(vec![
            Span::from(" [ "),
            Span::styled(
                "Target",
                Style::default().fg(match state.selected_source_tab {
                    MaterialSourceTab::Target => SELECTED_ROW_COLOR,
                    _ => Color::White,
                }),
            ),
            Span::from(" | "),
            Span::styled(
                "Broken P.",
                Style::default().fg(match state.selected_source_tab {
                    MaterialSourceTab::BrokenPart => SELECTED_ROW_COLOR,
                    _ => Color::White,
                }),
            ),
            Span::from(" | "),
            Span::styled(
                "Wound D.",
                Style::default().fg(match state.selected_source_tab {
                    MaterialSourceTab::WoundDestroy => SELECTED_ROW_COLOR,
                    _ => Color::White,
                }),
            ),
            Span::from(" | "),
            Span::styled(
                "Carves",
                Style::default().fg(match state.selected_source_tab {
                    MaterialSourceTab::Carve => SELECTED_ROW_COLOR,
                    _ => Color::White,
                }),
            ),
            Span::from(" ] "),
        ]);

        let drop_rank_blok = Block::new()
            .title(title_rank)
            .title_style(Style::default().bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        drop_rank_blok.render(drop_wrapper_chunk, buf);

        let drop_source_blok = Block::new()
            .title(title_source)
            .title_style(Style::default().bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        drop_source_blok.render(drop_rank_chunk, buf);

        let header = ["Material", "Percentage"]
            .into_iter()
            .map(|m| Cell::from(Text::from(Span::styled(m, Style::default().white())).centered()))
            .collect::<Row>()
            .style(Style::default().bold())
            .height(1);

        let rows: Vec<Row<'_>> = match state.selected_rank_tab {
            MonsterDropRankTab::LowRank => match state.selected_source_tab {
                MaterialSourceTab::Target => {
                    generate_drop_rows(&self.0.target, drop_chunk.width as usize)
                }
                MaterialSourceTab::BrokenPart => {
                    generate_drop_with_part_rows(&self.0.broken_part, drop_chunk.width as usize)
                }
                MaterialSourceTab::WoundDestroy => {
                    generate_drop_rows(&self.0.wound_destroy, drop_chunk.width as usize)
                }
                MaterialSourceTab::Carve => {
                    generate_drop_with_part_rows(&self.0.carve, drop_chunk.width as usize)
                }
            },
            MonsterDropRankTab::HighRank => match state.selected_source_tab {
                MaterialSourceTab::Target => {
                    generate_drop_rows(&self.1.target, drop_chunk.width as usize)
                }
                MaterialSourceTab::BrokenPart => {
                    generate_drop_with_part_rows(&self.1.broken_part, drop_chunk.width as usize)
                }
                MaterialSourceTab::WoundDestroy => {
                    generate_drop_rows(&self.1.wound_destroy, drop_chunk.width as usize)
                }
                MaterialSourceTab::Carve => {
                    generate_drop_with_part_rows(&self.1.carve, drop_chunk.width as usize)
                }
            },
        };

        let columns = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let selected_row_style = Style::default().bg(Color::Rgb(16, 33, 56));
        let bar = " █ ";

        StatefulWidget::render(
            Table::new(rows, columns)
                .header(header)
                .row_highlight_style(selected_row_style)
                .highlight_symbol(Text::from(vec![
                    bar.yellow().into(),
                    bar.yellow().into(),
                    "".into(),
                ]))
                .highlight_spacing(HighlightSpacing::Always),
            drop_chunk,
            buf,
            &mut state.table_state,
        );

        Scrollbar::new(ScrollbarOrientation::VerticalRight).render(
            drop_chunk,
            buf,
            &mut state.scrollbar_state,
        );
    }
}

fn generate_drop_rows(data: &[MaterialDrop], width: usize) -> Vec<Row<'_>> {
    data.iter()
        .map(|data| {
            let fmtd_material_cell = get_lines(&data.material, width - 30)
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n");

            let row_list = [
                Cell::from(Text::from(fmtd_material_cell)),
                Cell::from(Text::from(Span::from(format!("{}%", &data.percentage))).centered()),
            ];

            Row::new(row_list).height(3)
        })
        .collect()
}

fn generate_drop_with_part_rows(data: &[MaterialDropWithPart], width: usize) -> Vec<Row<'_>> {
    data.iter()
        .map(|data| {
            let fmtd_material_cell = get_lines(&data.material, width - 30)
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n");

            let concatenated_carve = &data.carve.iter().fold(String::new(), |mut acc, c| {
                acc.push_str(&format!("{}%({}) ", &c.percentage, &c.part));
                acc
            });

            let fmtd_carve = get_lines(concatenated_carve, width - 30)
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n");

            let row_list = [
                Cell::from(Text::from(fmtd_material_cell)),
                // Cell::from(Text::from(Span::from(fmtd_carve.to_string()))),
                Cell::from(Text::from(fmtd_carve)),
            ];

            Row::new(row_list).height(3)
        })
        .collect()
}
