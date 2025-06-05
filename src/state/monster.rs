use std::{path::PathBuf, rc::Rc};

use ratatui::{
    layout::Rect,
    widgets::{ListState, ScrollbarState, TableState},
};

use crate::monster::{
    monster::{
        DamageData, MonsterBasicInfo, MonsterElements, MonsterEntity, MonsterHabitatData,
        MonsterMaterialsDrop, MonsterName, MonsterQuestData, StatusAilmentItem,
    },
    MonsterDescText,
};

#[derive(Default)]
pub struct ScrollableParagraphState {
    pub scrollbar_state: ScrollbarState,
    pub position: usize,
    pub height: usize,
}

impl ScrollableParagraphState {
    pub fn set_height(&mut self, height: usize) {
        self.scrollbar_state = self.scrollbar_state.content_length(height);
        self.height = height;
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.update_scrollbar();
    }

    pub fn scroll_down(&mut self) {
        if self.position < self.height.saturating_sub(2) {
            self.position = self.position.saturating_add(1);
        }

        self.update_scrollbar();
    }

    pub fn scroll_up(&mut self) {
        self.position = self.position.saturating_sub(1);
        self.update_scrollbar();
    }

    fn update_scrollbar(&mut self) {
        self.scrollbar_state = self.scrollbar_state.position(self.position)
    }
}

#[derive(Default)]
pub struct MonsterOnQuestListState {
    pub list_state: ListState,
    pub scrollbar_state: ScrollbarState,
    pub list_len: u16,
}

impl MonsterOnQuestListState {
    pub fn set_scrollbar_height(&mut self, area: Rect) {
        if self.list_len > (area.height - 2) {
            self.scrollbar_state = self.scrollbar_state.content_length(self.list_len as usize);
        }
    }

    pub fn next(&mut self) {
        let idx = if let Some(i) = self.list_state.selected() {
            if i as u16 == self.list_len - 1 {
                0
            } else {
                i.saturating_add(1)
            }
        } else {
            0
        };

        self.select(idx);
    }

    pub fn prev(&mut self) {
        let idx: usize = if let Some(i) = self.list_state.selected() {
            if i == 0 {
                self.list_len as usize - 1
            } else {
                i.saturating_sub(1)
            }
        } else {
            self.list_len as usize - 1
        };

        self.select(idx);
    }

    pub fn select(&mut self, idx: usize) {
        self.list_state.select(Some(idx));
        self.scrollbar_state = self.scrollbar_state.position(idx);
    }

    pub fn reset(&mut self) {
        self.list_state.select(Some(0));
        self.scrollbar_state = self.scrollbar_state.position(0);
    }
}

#[derive(Default)]
pub enum MonsterWeaknessTab {
    #[default]
    WeaponDamage,
    ElementDamage,
}

#[derive(Default)]
pub enum MonsterStatusAndItemWeaknessTab {
    #[default]
    Status,
    Item,
}

#[derive(Default)]
pub struct MonsterWeaknessTabState {
    pub selected_weapon_elm_tab: MonsterWeaknessTab,
    pub selected_status_item_tab: MonsterStatusAndItemWeaknessTab,
    pub table_state: TableState,
    pub scrollbar_state: ScrollbarState,
    pub damage_data_len: u16,
}

impl MonsterWeaknessTabState {
    pub fn toggle_selected_ailment_item_tab(&mut self) {
        use MonsterStatusAndItemWeaknessTab::*;
        match self.selected_status_item_tab {
            Status => self.selected_status_item_tab = Item,
            Item => self.selected_status_item_tab = Status,
        }
    }

    pub fn toggle_selected_weapon_elm_tab(&mut self) {
        use MonsterWeaknessTab::*;
        match self.selected_weapon_elm_tab {
            WeaponDamage => self.selected_weapon_elm_tab = ElementDamage,
            ElementDamage => self.selected_weapon_elm_tab = WeaponDamage,
        };
        self.table_state.select(Some(0));
        self.scrollbar_state = self.scrollbar_state.position(0);
    }

    pub fn reset(&mut self) {
        self.table_state.select(Some(0));
        self.scrollbar_state = self.scrollbar_state.position(0);
        self.selected_status_item_tab = MonsterStatusAndItemWeaknessTab::Status;
        self.selected_weapon_elm_tab = MonsterWeaknessTab::WeaponDamage;
    }

    pub fn set_scrollbar_height(&mut self, area: Rect) {
        if (self.damage_data_len as usize) * 3 > area.height as usize - 2 {
            self.scrollbar_state = self
                .scrollbar_state
                .content_length(self.damage_data_len as usize)
        }
    }

    pub fn next(&mut self) {
        let idx = if let Some(id) = self.table_state.selected() {
            if id == (self.damage_data_len as usize) - 1 {
                0
            } else {
                id.saturating_add(1)
            }
        } else {
            0
        };

        self.select(idx);
    }

    pub fn prev(&mut self) {
        let idx = if let Some(id) = self.table_state.selected() {
            if id == 0 {
                (self.damage_data_len as usize) - 1
            } else {
                id.saturating_sub(1)
            }
        } else {
            (self.damage_data_len as usize) - 1
        };

        self.select(idx);
    }

    pub fn select(&mut self, idx: usize) {
        self.table_state.select(Some(idx));
        self.scrollbar_state = self.scrollbar_state.position(idx)
    }
}

#[derive(Default)]
pub enum MonsterDropRankTab {
    #[default]
    LowRank,
    HighRank,
}

#[derive(Default)]
pub enum MaterialSourceTab {
    #[default]
    Target,
    BrokenPart,
    WoundDestroy,
    Carve,
}

#[derive(Default)]
pub struct MonsterDropTabState {
    pub selected_rank_tab: MonsterDropRankTab,
    pub selected_source_tab: MaterialSourceTab,
    pub table_state: TableState,
    pub scrollbar_state: ScrollbarState,
    // pub low_rank_material: MonsterMaterialsDrop,
    // pub high_rank_material: MonsterMaterialsDrop,
    pub low_target_len: u16,
    pub low_broken_part_len: u16,
    pub low_wound_destroy_len: u16,
    pub low_carve_len: u16,
    pub high_target_len: u16,
    pub high_broken_part_len: u16,
    pub high_wound_destroy_len: u16,
    pub high_carve_len: u16,
}

impl MonsterDropTabState {
    // pub fn set_tab_table_state(
    //     low_rank_material: MonsterMaterialsDrop,
    //     high_rank_material: MonsterMaterialsDrop,
    // ) -> Self {
    //     Self {
    //         low_rank_material,
    //         high_rank_material,
    //         table_state: TableState::default().with_selected(0),
    //         ..Default::default()
    //     }
    // }

    pub fn toggle_selected_rank_tab(&mut self) {
        use MonsterDropRankTab::*;
        match self.selected_rank_tab {
            LowRank => self.selected_rank_tab = HighRank,
            HighRank => self.selected_rank_tab = LowRank,
        };
        self.table_state.select(Some(0));
        self.scrollbar_state = self.scrollbar_state.position(0);
    }

    pub fn toggle_selected_source_tab(&mut self) {
        use MaterialSourceTab::*;
        match self.selected_source_tab {
            Target => self.selected_source_tab = BrokenPart,
            BrokenPart => self.selected_source_tab = WoundDestroy,
            WoundDestroy => self.selected_source_tab = Carve,
            Carve => self.selected_source_tab = Target,
        };
        self.table_state.select(Some(0));
        self.scrollbar_state = self.scrollbar_state.position(0);
    }

    pub fn set_scrollbar_height(&mut self, area: Rect) {
        if self.get_current_data_use_len() * 3 > area.height - 2 {
            self.scrollbar_state = self
                .scrollbar_state
                .content_length(self.get_current_data_use_len() as usize);
        } else {
            self.scrollbar_state = self.scrollbar_state.content_length(0);
        }
    }

    pub fn next(&mut self) {
        let idx = if let Some(id) = self.table_state.selected() {
            if id == self.get_current_data_use_len() as usize - 1 {
                0
            } else {
                id.saturating_add(1)
            }
        } else {
            0
        };

        self.select(idx);
    }

    pub fn prev(&mut self) {
        let idx = if let Some(id) = self.table_state.selected() {
            if id == 0 {
                self.get_current_data_use_len() as usize - 1
            } else {
                id.saturating_sub(1)
            }
        } else {
            self.get_current_data_use_len() as usize - 1
        };

        self.select(idx);
    }

    fn get_current_data_use_len(&self) -> u16 {
        match self.selected_rank_tab {
            MonsterDropRankTab::LowRank => match self.selected_source_tab {
                MaterialSourceTab::Target => self.low_target_len,
                MaterialSourceTab::BrokenPart => self.low_broken_part_len,
                MaterialSourceTab::WoundDestroy => self.low_wound_destroy_len,
                MaterialSourceTab::Carve => self.low_carve_len,
            },
            MonsterDropRankTab::HighRank => match self.selected_source_tab {
                MaterialSourceTab::Target => self.high_target_len,
                MaterialSourceTab::BrokenPart => self.high_broken_part_len,
                MaterialSourceTab::WoundDestroy => self.high_wound_destroy_len,
                MaterialSourceTab::Carve => self.high_carve_len,
            },
        }
    }

    pub fn select(&mut self, idx: usize) {
        self.table_state.select(Some(idx));
        self.scrollbar_state = self.scrollbar_state.position(idx)
    }

    pub fn reset(&mut self) {
        self.table_state.select(Some(0));
        self.scrollbar_state = self.scrollbar_state.position(0);
        self.selected_rank_tab = MonsterDropRankTab::LowRank;
        self.selected_source_tab = MaterialSourceTab::Target;
    }
}

#[derive(Default, Debug)]
pub struct ChangeableHabitatPageState {
    pub habitat_total_page: u8,
    pub habitat_current_page: u8,
}

impl ChangeableHabitatPageState {
    pub fn reset(&mut self) {
        self.habitat_current_page = 0;
    }

    pub fn next(&mut self) {
        if self.habitat_total_page == 0 {
            return;
        }

        if self.habitat_current_page == self.habitat_total_page - 1 {
            self.habitat_current_page = 0;
        } else {
            self.habitat_current_page += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.habitat_total_page == 0 {
            return;
        }

        if self.habitat_current_page == 0 {
            self.habitat_current_page = self.habitat_total_page - 1;
        } else {
            self.habitat_current_page -= 1;
        }
    }
}

#[derive(Default)]
pub struct MonsterListState {
    pub list_state: ListState,
    pub list_scrollbar_state: ScrollbarState,
    pub desc_scrollbar_state: ScrollableParagraphState,
    pub habitat_page_state: ChangeableHabitatPageState,
    pub monster_on_quest_list_state: MonsterOnQuestListState,
    pub monster_weakness_tab_state: MonsterWeaknessTabState,
    pub monster_drops_tab_state: MonsterDropTabState,
    pub filter_query: String,
    pub filtered_list: Vec<Rc<MonsterEntity>>,
    pub bundle: Vec<Rc<MonsterEntity>>,
    pub current_data: Rc<MonsterEntity>,
    pub ascii_asset: PathBuf,
}

impl MonsterListState {
    pub fn new(bundle: Vec<Rc<MonsterEntity>>) -> Option<Self> {
        let current_data = Rc::clone(bundle.first()?);
        let quest_list_len = current_data.quest_list.len() as u16;
        let h_total_page = current_data.habitats.len() as u8;
        let damage_data_len = current_data.weaknesses.dmg_data.len() as u16;

        let low_target_len = current_data.drops.low_rank.target.len() as u16;
        let low_broken_part_len = current_data.drops.low_rank.broken_part.len() as u16;
        let low_wound_destroy_len = current_data.drops.low_rank.wound_destroy.len() as u16;
        let low_carve_len = current_data.drops.low_rank.carve.len() as u16;

        let high_target_len = current_data.drops.high_rank.target.len() as u16;
        let high_broken_part_len = current_data.drops.high_rank.broken_part.len() as u16;
        let high_wound_destroy_len = current_data.drops.high_rank.wound_destroy.len() as u16;
        let high_carve_len = current_data.drops.high_rank.carve.len() as u16;

        let filtered_list = Vec::with_capacity(bundle.len());

        Some(Self {
            bundle,
            current_data,
            filtered_list,
            list_state: ListState::default().with_selected(Some(0)),
            monster_on_quest_list_state: MonsterOnQuestListState {
                list_len: quest_list_len,
                ..Default::default()
            },
            habitat_page_state: ChangeableHabitatPageState {
                habitat_total_page: h_total_page,
                ..Default::default()
            },
            monster_weakness_tab_state: MonsterWeaknessTabState {
                damage_data_len,
                ..Default::default()
            },
            monster_drops_tab_state: MonsterDropTabState {
                low_target_len,
                low_broken_part_len,
                low_wound_destroy_len,
                low_carve_len,
                high_target_len,
                high_broken_part_len,
                high_wound_destroy_len,
                high_carve_len,
                ..Default::default()
            },
            ..Default::default()
        })
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.ascii_asset = path;
        self
    }

    pub fn update_selected_monster_data(&mut self) {
        let idx = self.list_state.selected().unwrap_or(0);

        let current_monster = self.list_items().get(idx);
        self.current_data = if let Some(cm) = current_monster {
            Rc::clone(cm)
        } else {
            Rc::new(MonsterEntity::default())
        };

        self.monster_on_quest_list_state.list_len = self.current_data.quest_list.len() as u16;
        self.habitat_page_state.habitat_total_page = self.current_data.habitats.len() as u8;
        self.monster_weakness_tab_state.damage_data_len =
            self.current_data.weaknesses.dmg_data.len() as u16;

        self.monster_drops_tab_state.low_target_len =
            self.current_data.drops.low_rank.target.len() as u16;
        self.monster_drops_tab_state.low_broken_part_len =
            self.current_data.drops.low_rank.broken_part.len() as u16;
        self.monster_drops_tab_state.low_wound_destroy_len =
            self.current_data.drops.low_rank.wound_destroy.len() as u16;
        self.monster_drops_tab_state.low_carve_len =
            self.current_data.drops.low_rank.carve.len() as u16;

        self.monster_drops_tab_state.high_target_len =
            self.current_data.drops.high_rank.target.len() as u16;
        self.monster_drops_tab_state.high_broken_part_len =
            self.current_data.drops.high_rank.broken_part.len() as u16;
        self.monster_drops_tab_state.high_wound_destroy_len =
            self.current_data.drops.high_rank.wound_destroy.len() as u16;
        self.monster_drops_tab_state.high_carve_len =
            self.current_data.drops.high_rank.carve.len() as u16;
    }

    pub fn next(&mut self) {
        if self.list_items().is_empty() {
            return;
        }

        let idx = match self.list_state.selected() {
            Some(i) => {
                if i >= self.list_items().len() - 1 {
                    0
                } else {
                    i.saturating_add(1)
                }
            }
            None => 0,
        };

        self.select(idx);
    }

    pub fn prev(&mut self) {
        if self.list_items().is_empty() {
            return;
        }

        let idx = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list_items().len() - 1
                } else {
                    i.saturating_sub(1)
                }
            }
            None => 0,
        };

        self.select(idx);
    }

    pub fn select(&mut self, idx: usize) {
        self.list_state.select(Some(idx));
        // self.scrollbar_state = self.scrollbar_state.position(idx)

        self.update_selected_monster_data();
    }

    pub fn get_quest_list(&self) -> Vec<MonsterQuestData> {
        self.current_data.quest_list.clone()
    }

    pub fn get_habitat_data(&self) -> Vec<MonsterHabitatData> {
        self.current_data.habitats.clone()
    }

    pub fn get_basic_info_data(&self) -> MonsterBasicInfo {
        self.current_data.basic_info.clone()
    }

    pub fn get_desc_data(&self) -> MonsterDescText {
        self.current_data.desc.clone()
    }

    pub fn get_name_data(&self) -> MonsterName {
        self.current_data.name.clone()
    }

    pub fn get_elements_data(&self) -> Vec<MonsterElements> {
        self.current_data.elements.clone()
    }

    pub fn get_asset_path(&self) -> PathBuf {
        self.ascii_asset.join(self.current_data.icon_code.clone())
    }

    pub fn get_damage_data(&self) -> Vec<DamageData> {
        self.current_data.weaknesses.dmg_data.clone()
    }

    pub fn get_status_ailment_data(&self) -> StatusAilmentItem {
        self.current_data.weaknesses.ailment_data.clone()
    }

    pub fn get_low_rank_drop_data(&self) -> MonsterMaterialsDrop {
        self.current_data.drops.low_rank.clone()
    }

    pub fn get_high_rank_drop_data(&self) -> MonsterMaterialsDrop {
        self.current_data.drops.high_rank.clone()
    }

    pub(crate) fn set_list_filter(&mut self, filter: String) {
        self.filter_query.clone_from(&filter);

        if !filter.is_empty() {
            self.filtered_list.clear();
            self.filtered_list.extend(
                self.bundle
                    .iter()
                    .filter(|item| {
                        item.name
                            .name
                            .to_lowercase()
                            .contains(&filter.to_lowercase())
                    })
                    .cloned(),
            );
        }

        self.select(0);
    }

    pub fn list_items(&self) -> &Vec<Rc<MonsterEntity>> {
        if self.filter_query.is_empty() {
            &self.bundle
        } else {
            &self.filtered_list
        }
    }

    pub fn reset(&mut self) {
        self.desc_scrollbar_state.reset();
        self.habitat_page_state.reset();
        self.monster_on_quest_list_state.reset();
        self.monster_weakness_tab_state.reset();
        self.monster_drops_tab_state.reset();

        // self.list_state.select(Some(0));
        // self.scrollbar_state = self.scrollbar_state.position(0);
    }
}
