use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Cell, HighlightSpacing, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, StatefulWidget, Table, Widget,
    },
};

use crate::{
    monster::monster::{DamageData, MonsterElements, StatusAilmentItem},
    state::monster::{
        MonsterStatusAndItemWeaknessTab, MonsterWeaknessTab, MonsterWeaknessTabState,
    },
};

use super::get_lines;

pub struct WeaknessAilmentItems<'a> {
    weakness: &'a StatusAilmentItem,
}

impl<'a> WeaknessAilmentItems<'a> {
    pub fn new(weakness: &'a StatusAilmentItem) -> Self {
        Self { weakness }
    }
}

impl<'a> StatefulWidget for WeaknessAilmentItems<'a> {
    type State = MonsterStatusAndItemWeaknessTab;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [_, ailment_item_wrapper_chunk] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(1)])
                .horizontal_margin(1)
                .areas(area);

        let [ailment_item_chunk] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .margin(1)
            .areas(ailment_item_wrapper_chunk);

        let title_ailment_item = Line::from(vec![
            Span::from(" [ "),
            Span::styled(
                "Ailment",
                Style::default().fg(match state {
                    MonsterStatusAndItemWeaknessTab::Status => Color::Rgb(54, 127, 222),
                    _ => Color::White,
                }),
            ),
            Span::from(" | "),
            Span::styled(
                "Item",
                Style::default().fg(match state {
                    MonsterStatusAndItemWeaknessTab::Item => Color::Rgb(54, 127, 222),
                    _ => Color::White,
                }),
            ),
            Span::from(" ] "),
        ]);

        let weakness_block = Block::new()
            .title(title_ailment_item)
            .title_style(Style::default().bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        weakness_block.render(ailment_item_wrapper_chunk, buf);

        let [w1_chunk, w2_chunk, w3_chunk, w4_chunk, w5_chunk, w6_chunk] = Layout::horizontal([
            Constraint::Percentage(100 / 6),
            Constraint::Percentage(100 / 6),
            Constraint::Percentage(100 / 6),
            Constraint::Percentage(100 / 6),
            Constraint::Percentage(100 / 6),
            Constraint::Percentage(100 / 6),
        ])
        .areas(ailment_item_chunk);

        let mapped_paragraph: Vec<Paragraph<'_>> = match state {
            MonsterStatusAndItemWeaknessTab::Status => self
                .weakness
                .status
                .iter()
                .map(|h| {
                    let text_header = vec![
                        Line::from(vec![
                            Span::styled(h.ailment.icon(), Style::default().fg(h.ailment.color())),
                            Span::styled(format!(" {}", &h.ailment), Style::default().bold()),
                        ]),
                        Line::from(vec![
                            Span::styled("", Style::default().fg(Color::Yellow)),
                            Span::from(format!(" {}", &h.eff)),
                        ]),
                    ];

                    Paragraph::new(text_header).centered()
                })
                .collect(),
            MonsterStatusAndItemWeaknessTab::Item => self
                .weakness
                .items
                .iter()
                .map(|h| {
                    let text_header = vec![
                        Line::from(vec![Span::styled(
                            format!(" {}", &h.item),
                            Style::default().bold(),
                        )]),
                        if h.is_effective {
                            "".green().bold().into()
                        } else {
                            "".red().bold().into()
                        },
                    ];

                    Paragraph::new(text_header).centered()
                })
                .collect(),
        };

        let [w1, w2, w3, w4, w5, w6]: [Paragraph; 6] = mapped_paragraph.try_into().unwrap_or([
            Paragraph::new(""),
            Paragraph::new(""),
            Paragraph::new(""),
            Paragraph::new(""),
            Paragraph::new(""),
            Paragraph::new(""),
        ]);

        w1.render(w1_chunk, buf);
        w2.render(w2_chunk, buf);
        w3.render(w3_chunk, buf);
        w4.render(w4_chunk, buf);
        w5.render(w5_chunk, buf);
        w6.render(w6_chunk, buf);
    }
}

pub struct WeaknessMonsterTable(pub Vec<DamageData>, pub StatusAilmentItem);

impl StatefulWidget for WeaknessMonsterTable {
    type State = MonsterWeaknessTabState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [ailment_item_chunk, table_chunk] =
            Layout::vertical([Constraint::Length(5), Constraint::Min(1)]).areas(area);

        let [tab_table_wrapper_chunk, _] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)])
                .horizontal_margin(1)
                .areas(table_chunk);

        let [tab_table_chunk] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .margin(1)
            .areas(tab_table_wrapper_chunk);

        state.set_scrollbar_height(tab_table_chunk);

        let title_weapon_element = Line::from(vec![
            Span::from(" [ "),
            Span::styled(
                "Weapon",
                Style::default().fg(match state.selected_weapon_elm_tab {
                    MonsterWeaknessTab::WeaponDamage => Color::Rgb(54, 127, 222),
                    _ => Color::White,
                }),
            ),
            Span::from(" | "),
            Span::styled(
                "Element",
                Style::default().fg(match state.selected_weapon_elm_tab {
                    MonsterWeaknessTab::ElementDamage => Color::Rgb(54, 127, 222),
                    _ => Color::White,
                }),
            ),
            Span::from(" ] "),
        ]);

        let block_title = Block::new()
            .title(" Weakness ")
            .title_style(Style::default().bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        block_title.render(area, buf);

        let weakness_block = Block::new()
            .title(title_weapon_element)
            .title_style(Style::default().bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        weakness_block.render(tab_table_wrapper_chunk, buf);

        let fire_icon = MonsterElements::Fire.icon();
        let water_icon = MonsterElements::Water.icon();
        let thunder_icon = MonsterElements::Thunder.icon();
        let ice_icon = MonsterElements::Ice.icon();
        let dragon_icon = MonsterElements::Dragon.icon();

        let selected_header = match state.selected_weapon_elm_tab {
            MonsterWeaknessTab::WeaponDamage => vec!["Part", "Cut(󰓥)", "Blunt(󰣪)", "Ammo(󰳳)"],
            MonsterWeaknessTab::ElementDamage => vec![
                "Part",
                &fire_icon,
                &water_icon,
                &thunder_icon,
                &ice_icon,
                &dragon_icon,
            ],
        };

        let header = selected_header
            .into_iter()
            .enumerate()
            .map(|(i, h)| {
                Cell::from(Span::styled(
                    h,
                    Style::default().fg(match (i, &state.selected_weapon_elm_tab) {
                        (1, MonsterWeaknessTab::ElementDamage) => MonsterElements::Fire.color(),
                        (2, MonsterWeaknessTab::ElementDamage) => MonsterElements::Water.color(),
                        (3, MonsterWeaknessTab::ElementDamage) => MonsterElements::Thunder.color(),
                        (4, MonsterWeaknessTab::ElementDamage) => MonsterElements::Ice.color(),
                        (5, MonsterWeaknessTab::ElementDamage) => MonsterElements::Dragon.color(),
                        _ => Color::White,
                    }),
                ))
            })
            .collect::<Row>()
            .style(Style::default().bold())
            .height(1);

        let rows: Vec<Row<'_>> = self
            .0
            .iter()
            .map(|data| {
                let mp_row = get_lines(&data.monster_part, 20 - 4)
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join("\n");

                let selected_rows = match state.selected_weapon_elm_tab {
                    MonsterWeaknessTab::WeaponDamage => vec![
                        Cell::from(text_rows_generator(&data.weapon.cut_damage)),
                        Cell::from(text_rows_generator(&data.weapon.blunt_damage)),
                        Cell::from(text_rows_generator(&data.weapon.ammo_damage)),
                    ],
                    MonsterWeaknessTab::ElementDamage => vec![
                        Cell::from(text_rows_generator(&data.element.fire_damage)),
                        Cell::from(text_rows_generator(&data.element.water_damage)),
                        Cell::from(text_rows_generator(&data.element.thunder_damage)),
                        Cell::from(text_rows_generator(&data.element.ice_damage)),
                        Cell::from(text_rows_generator(&data.element.dragon_damage)),
                    ],
                };

                let mut row_list = vec![Cell::from(Text::from(mp_row))];
                row_list.extend_from_slice(&selected_rows);
                Row::new(row_list).height(3) // INFO: row item height
            })
            .collect();

        let selected_row_style = Style::default().bg(Color::Rgb(16, 33, 56));
        let bar = " █ ";

        let columns = match state.selected_weapon_elm_tab {
            MonsterWeaknessTab::WeaponDamage => vec![
                Constraint::Length(20),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
            ],
            MonsterWeaknessTab::ElementDamage => vec![
                Constraint::Length(20),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
            ],
        };

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
            tab_table_chunk,
            buf,
            &mut state.table_state,
        );

        Scrollbar::new(ScrollbarOrientation::VerticalRight).render(
            tab_table_chunk,
            buf,
            &mut state.scrollbar_state,
        );

        WeaknessAilmentItems::new(&self.1).render(
            ailment_item_chunk,
            buf,
            &mut state.selected_status_item_tab,
        );
    }
}

fn text_rows_generator(number: &u8) -> Text {
    Text::from(vec![
        Line::from(if *number == 0 {
            vec!["".red().bold()]
        } else {
            vec![
                Span::styled("", Style::default().fg(Color::Yellow)),
                Span::from(format!(" {}", &number)),
            ]
        }),
        "".into(),
    ])
}
