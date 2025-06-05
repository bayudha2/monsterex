pub mod basic_info;
pub mod drop;
pub mod habitats;
pub mod monster_desc;
pub mod monster_name_icon;
pub mod monster_quest;
pub mod search;
pub mod selection;
pub mod weakness;

use basic_info::BasicInfo;
use drop::MonsterDropTable;
use habitats::HabitatsMonster;
use monster_desc::DescMonster;
use monster_name_icon::NameIcon;
use monster_quest::QuestMonsterList;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{StatefulWidget, Widget},
};
use textwrap::WordSeparator;
use weakness::WeaknessMonsterTable;

use crate::state::monster::MonsterListState;

type ProfileLayout = [Rect; 7];

pub struct MonsterProfileWidget;

impl MonsterProfileWidget {
    fn get_render_areas(&self, area: Rect) -> ProfileLayout {
        let [left_chunk, right_chunk] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(area);

        let [monster_name_icon_chunk, monster_desc_chunk] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(left_chunk);

        let [basic_info_chunk, right_chunk] =
            Layout::vertical([Constraint::Length(4), Constraint::Min(1)]).areas(right_chunk);

        let [habitat_quest_chunk, right_chunk] =
            Layout::vertical([Constraint::Length(15), Constraint::Min(1)]).areas(right_chunk);

        let [habitat_chunk, quest_chunk] =
            Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)])
                .areas(habitat_quest_chunk);

        let [monster_material_chunk, weakness_chunk] =
            Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)])
                .areas(right_chunk);

        [
            monster_name_icon_chunk,
            monster_desc_chunk,
            basic_info_chunk,
            habitat_chunk,
            quest_chunk,
            monster_material_chunk,
            weakness_chunk,
        ]
    }
}

impl StatefulWidget for MonsterProfileWidget {
    type State = MonsterListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let [monster_icon_rect, monster_desc_rect, basic_info_chunk, habitat_chunk, quest_chunk, monster_material_chunk, weakness_chunk] =
            self.get_render_areas(area);

        NameIcon::new(
            state.get_name_data(),
            state.get_elements_data(),
            state.get_asset_path(),
        )
        .render(monster_icon_rect, buf);

        BasicInfo::new(state.get_basic_info_data()).render(basic_info_chunk, buf);

        HabitatsMonster(state.get_habitat_data()).render(
            habitat_chunk,
            buf,
            &mut state.habitat_page_state,
        );

        QuestMonsterList(state.get_quest_list()).render(
            quest_chunk,
            buf,
            &mut state.monster_on_quest_list_state,
        );

        DescMonster(state.get_desc_data()).render(
            monster_desc_rect,
            buf,
            &mut state.desc_scrollbar_state,
        );

        WeaknessMonsterTable(state.get_damage_data(), state.get_status_ailment_data()).render(
            weakness_chunk,
            buf,
            &mut state.monster_weakness_tab_state,
        );

        MonsterDropTable(
            state.get_low_rank_drop_data(),
            state.get_high_rank_drop_data(),
        )
        .render(
            monster_material_chunk,
            buf,
            &mut state.monster_drops_tab_state,
        );
    }
}

pub fn get_lines(text: &str, width: usize) -> Vec<String> {
    let options = textwrap::Options::new(width).word_separator(WordSeparator::AsciiSpace);

    let lines = textwrap::wrap(text, options);
    lines.into_iter().map(|x| x.to_string()).collect()
}
