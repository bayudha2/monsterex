use std::fmt::Display;

use ratatui::style::Color;
use serde::{de::Visitor, Deserialize};

use super::MonsterDescText;

pub type MonsterDescriptionAbility = [String; 2];

#[derive(Deserialize, Clone, Default)]
pub struct MonsterEntity {
    pub id: u16,
    pub name: MonsterName,
    pub icon_code: String,
    pub elements: Vec<MonsterElements>,
    pub desc: MonsterDescText,
    pub basic_info: MonsterBasicInfo,
    pub quest_list: Vec<MonsterQuestData>,
    pub habitats: Vec<MonsterHabitatData>,
    pub weaknesses: MonsterWeaknesses,
    pub drops: MonsterDrops,
}

#[derive(Default, Clone)]
pub enum Roarskind {
    Weak,
    Strong,
    #[default]
    None,
}

impl Display for Roarskind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Roarskind::*;
        match self {
            Strong => write!(f, "Strong"),
            Weak => write!(f, "Weak"),
            None => write!(f, "None"),
        }
    }
}

impl<'de> Deserialize<'de> for Roarskind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(RoarskindVisitor)
    }
}

struct RoarskindVisitor;

impl<'de> Visitor<'de> for RoarskindVisitor {
    type Value = Roarskind;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Roar kind")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use Roarskind::*;

        match v {
            "weak" => Ok(Weak),
            "strong" => Ok(Strong),
            _ => Ok(None),
        }
    }
}

#[derive(Default, Clone)]
pub enum WindPressureKind {
    Strong,
    Weak,
    Minor,
    #[default]
    None,
}

impl Display for WindPressureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WindPressureKind::*;
        match self {
            Strong => write!(f, "Strong"),
            Weak => write!(f, "Weak"),
            Minor => write!(f, "Minor"),
            None => write!(f, "None"),
        }
    }
}

impl<'de> Deserialize<'de> for WindPressureKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(WindPressureKindVisitor)
    }
}

struct WindPressureKindVisitor;

impl<'de> Visitor<'de> for WindPressureKindVisitor {
    type Value = WindPressureKind;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Wind Pressure Kind")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use WindPressureKind::*;
        match v {
            "strong" => Ok(Strong),
            "weak" => Ok(Weak),
            "minor" => Ok(Minor),
            _ => Ok(None),
        }
    }
}

#[derive(Default, Clone)]
pub enum TremorKind {
    Strong,
    Weak,
    Minor,
    #[default]
    None,
}

impl Display for TremorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TremorKind::*;
        match self {
            Strong => write!(f, "Strong"),
            Weak => write!(f, "Weak"),
            Minor => write!(f, "Minor"),
            None => write!(f, "None"),
        }
    }
}

impl<'de> Deserialize<'de> for TremorKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TremorKindVisitor)
    }
}

struct TremorKindVisitor;

impl<'de> Visitor<'de> for TremorKindVisitor {
    type Value = TremorKind;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Tremor Kind")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use TremorKind::*;

        match v {
            "strong" => Ok(Strong),
            "weak" => Ok(Weak),
            "minor" => Ok(Minor),
            _ => Ok(None),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum QuestType {
    #[default]
    Assignments,
    Optional,
    Event,
    Arena,
}

impl Display for QuestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use QuestType::*;
        match self {
            Assignments => write!(f, "Assignments"),
            Optional => write!(f, "Optional"),
            Event => write!(f, "Event"),
            Arena => write!(f, "Arena"),
        }
    }
}

impl<'de> Deserialize<'de> for QuestType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(QuestTypeVisitor)
    }
}

struct QuestTypeVisitor;

impl<'de> Visitor<'de> for QuestTypeVisitor {
    type Value = QuestType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Quest Type")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use QuestType::*;

        match v {
            "assignments" => Ok(Assignments),
            "optional" => Ok(Optional),
            "event" => Ok(Event),
            "arena" => Ok(Arena),
            _ => Ok(Optional),
        }
    }
}

#[derive(Default, Clone)]
pub enum StatusAilments {
    Poison,
    #[default]
    Stun,
    Paralysis,
    Sleep,
    Blast,
    Exhaust,
}

impl StatusAilments {
    pub fn icon(&self) -> String {
        use StatusAilments::*;
        match self {
            Poison => "󰻊".to_string(),
            Stun => "󰙴".to_string(),
            Paralysis => "".to_string(),
            Sleep => "󰒲".to_string(),
            Blast => "".to_string(),
            Exhaust => "".to_string(),
        }
    }

    pub fn color(&self) -> Color {
        use StatusAilments::*;
        match self {
            Poison => Color::Rgb(126, 39, 219),
            Stun => Color::Yellow,
            Paralysis => Color::Yellow,
            Sleep => Color::Cyan,
            Blast => Color::DarkGray,
            Exhaust => Color::Gray,
        }
    }
}

impl Display for StatusAilments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use StatusAilments::*;
        match self {
            Poison => write!(f, "Poison"),
            Stun => write!(f, "Stun"),
            Paralysis => write!(f, "Paralysis"),
            Sleep => write!(f, "Sleep"),
            Blast => write!(f, "Blast"),
            Exhaust => write!(f, "Exhaust"),
        }
    }
}

impl<'de> Deserialize<'de> for StatusAilments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StatusAilmentsVisitor)
    }
}

pub struct StatusAilmentsVisitor;

impl<'de> Visitor<'de> for StatusAilmentsVisitor {
    type Value = StatusAilments;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Status Ailments")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use StatusAilments::*;

        match v {
            "poison" => Ok(Poison),
            "stun" => Ok(Stun),
            "paralysis" => Ok(Paralysis),
            "sleep" => Ok(Sleep),
            "blast" => Ok(Blast),
            "exhaust" => Ok(Exhaust),
            _ => Ok(Stun),
        }
    }
}

#[derive(Default, Clone)]
pub enum ItemWeakness {
    Pitfall,
    Shock,
    Meats,
    #[default]
    Flashpod,
    Sonicpod,
    Dungpod,
}

impl Display for ItemWeakness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ItemWeakness::*;
        match self {
            Pitfall => write!(f, "Pitfall"),
            Shock => write!(f, "Shock"),
            Meats => write!(f, "Meats"),
            Flashpod => write!(f, "Flashpod"),
            Sonicpod => write!(f, "Sonicpod"),
            Dungpod => write!(f, "Dungpod"),
        }
    }
}

impl<'de> Deserialize<'de> for ItemWeakness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ItemWeaknessVisitor)
    }
}

pub struct ItemWeaknessVisitor;

impl<'de> Visitor<'de> for ItemWeaknessVisitor {
    type Value = ItemWeakness;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Item Weakness")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use ItemWeakness::*;

        match v {
            "pitfall" => Ok(Pitfall),
            "shock" => Ok(Shock),
            "meats" => Ok(Meats),
            "flashpod" => Ok(Flashpod),
            "sonicpod" => Ok(Sonicpod),
            "dungpod" => Ok(Dungpod),
            _ => Ok(Flashpod),
        }
    }
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct MonsterQuestData {
    pub quest_type: QuestType,
    pub level: u8,
    pub name: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct MonsterStatusAilment {
    pub ailment: StatusAilments,
    pub eff: u8,
}

#[derive(Deserialize, Clone, Default)]
pub struct MonsterStatusItem {
    pub item: ItemWeakness,
    pub is_effective: bool,
}

#[derive(Deserialize, Clone, Default)]
pub struct StatusAilmentItem {
    pub status: [MonsterStatusAilment; 6],
    pub items: [MonsterStatusItem; 6],
}

#[derive(Deserialize, Clone, Default)]
pub struct WeaponDamageData {
    pub cut_damage: u8,
    pub blunt_damage: u8,
    pub ammo_damage: u8,
}

#[derive(Deserialize, Clone, Default)]
pub struct ElementDamageData {
    pub fire_damage: u8,
    pub water_damage: u8,
    pub thunder_damage: u8,
    pub ice_damage: u8,
    pub dragon_damage: u8,
}

#[derive(Deserialize, Clone, Default)]
pub struct DamageData {
    pub monster_part: String,
    pub weapon: WeaponDamageData,
    pub element: ElementDamageData,
}

#[derive(Deserialize, Clone, Default)]
pub struct MonsterWeaknesses {
    pub dmg_data: Vec<DamageData>,
    pub ailment_data: StatusAilmentItem,
}

#[derive(Deserialize, Default, Clone)]
pub struct MaterialDrop {
    pub material: String,
    pub percentage: u8,
}

#[derive(Deserialize, Default, Clone)]
pub struct DropWithPart {
    pub part: String,
    pub percentage: u8,
}

#[derive(Deserialize, Default, Clone)]
pub struct MaterialDropWithPart {
    pub material: String,
    pub carve: Vec<DropWithPart>,
}

#[derive(Deserialize, Clone, Default)]
pub struct MonsterMaterialsDrop {
    pub target: Vec<MaterialDrop>,
    pub broken_part: Vec<MaterialDropWithPart>,
    pub wound_destroy: Vec<MaterialDrop>,
    pub carve: Vec<MaterialDropWithPart>,
}

#[derive(Default, Clone, Deserialize)]
pub struct MonsterDrops {
    pub low_rank: MonsterMaterialsDrop,
    pub high_rank: MonsterMaterialsDrop,
}

#[derive(Deserialize, Default, Clone)]
pub struct MonsterHabitatData {
    pub region: String,
    pub starting_area: Vec<u8>,
    pub visited_area: Vec<u8>,
    pub resting_area: u8,
}

#[derive(Deserialize, Default, Clone)]
pub struct MonsterBasicInfo {
    #[serde(rename = "type")]
    pub m_type: String,
    pub roar: Roarskind,
    pub wind_pressure: WindPressureKind,
    pub tremor: TremorKind,
    pub status_effect: Vec<MonsterStatusEffects>,
}

#[derive(Debug, Default, Clone)]
pub enum MonsterStatusEffects {
    Fireblight,
    Blastblight,
    Waterblight,
    Thunderblight,
    Iceblight,
    Frostblight,
    Dragonblight,
    Frenzy,
    Sleep,
    Poison,
    Paralysis,
    Stench,
    DefenseDown,
    Bleeding,
    Flash,
    MinorBubbleBlight,
    MajorBubbleBlight,
    Webbed,
    NotRegister,
    #[default]
    None,
}

impl MonsterStatusEffects {
    // TODO: fix color: change the dark color to brighter one
    pub fn color(&self) -> Color {
        use MonsterStatusEffects::*;
        match self {
            Fireblight => Color::Red,
            Blastblight => Color::Rgb(217, 98, 24),
            Waterblight => Color::Rgb(66, 135, 245),
            Thunderblight => Color::Yellow,
            Iceblight => Color::Cyan,
            Frostblight => Color::White,
            Dragonblight => Color::Rgb(154, 88, 237),
            Frenzy => Color::Rgb(150, 0, 237),
            Paralysis => Color::Yellow,
            Sleep => Color::Cyan,
            Poison => Color::Rgb(126, 39, 219),
            Stench => Color::Rgb(172, 191, 2),
            DefenseDown => Color::LightBlue,
            Bleeding => Color::Red,
            Flash => Color::White,
            MinorBubbleBlight => Color::White,
            MajorBubbleBlight => Color::White,
            Webbed => Color::White,
            NotRegister => Color::White,
            None => Color::Black,
        }
    }
}

impl Display for MonsterStatusEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MonsterStatusEffects::*;
        match self {
            Fireblight => write!(f, "Fireblight"),
            Blastblight => write!(f, "Blastblight"),
            Waterblight => write!(f, "Waterblight"),
            Thunderblight => write!(f, "Thunderblight"),
            Iceblight => write!(f, "Iceblight"),
            Frostblight => write!(f, "Frostblight"),
            Dragonblight => write!(f, "Dragonblight"),
            Frenzy => write!(f, "Frenzy"),
            Paralysis => write!(f, "Paralysis"),
            Sleep => write!(f, "Sleep"),
            Poison => write!(f, "Poison"),
            Stench => write!(f, "Stench"),
            DefenseDown => write!(f, "Defense Down"),
            Bleeding => write!(f, "Bleeding"),
            Flash => write!(f, "Flash"),
            MinorBubbleBlight => write!(f, "Minor Bubble Blight"),
            MajorBubbleBlight => write!(f, "Major Bubble Blight"),
            Webbed => write!(f, "Webbed"),
            NotRegister => write!(f, "Not Register"),
            None => write!(f, "None"),
        }
    }
}

impl<'de> Deserialize<'de> for MonsterStatusEffects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(MonsterStatusEffectsVisitor)
    }
}

struct MonsterStatusEffectsVisitor;

impl<'de> Visitor<'de> for MonsterStatusEffectsVisitor {
    type Value = MonsterStatusEffects;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Monster Status Effect")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use MonsterStatusEffects::*;

        match v {
            "fireblight" => Ok(Fireblight),
            "blastblight" => Ok(Blastblight),
            "waterblight" => Ok(Waterblight),
            "thunderblight" => Ok(Thunderblight),
            "iceblight" => Ok(Iceblight),
            "frostblight" => Ok(Frostblight),
            "dragonblight" => Ok(Dragonblight),
            "frenzy" => Ok(Frenzy),
            "paralysis" => Ok(Paralysis),
            "sleep" => Ok(Sleep),
            "poison" => Ok(Poison),
            "stench" => Ok(Stench),
            "defense down" => Ok(DefenseDown),
            "bleeding" => Ok(Bleeding),
            "flash" => Ok(Flash),
            "minor bubbleblight" => Ok(MinorBubbleBlight),
            "major bubbleblight" => Ok(MajorBubbleBlight),
            "webbed" => Ok(Webbed),
            "none" => Ok(None),
            _ => Ok(NotRegister),
        }
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct MonsterName {
    pub name: String,
    pub aka: String,
}

#[derive(Debug, Clone, Default)]
pub enum MonsterElements {
    Fire,
    Water,
    Thunder,
    Ice,
    Dragon,
    Poison,
    #[default]
    None,
}

impl MonsterElements {
    // TODO: fix color: change the dark color to brighter one
    pub fn color(&self) -> Color {
        use MonsterElements::*;
        match self {
            Fire => Color::Red,
            Water => Color::Rgb(66, 135, 245),
            Thunder => Color::Yellow,
            Ice => Color::Cyan,
            Dragon => Color::Rgb(154, 88, 237),
            Poison => Color::Rgb(126, 39, 219),
            None => Color::Black,
        }
    }

    pub fn icon(&self) -> String {
        use MonsterElements::*;
        match self {
            Fire => "󰈸".to_string(),
            Water => "󰖌".to_string(),
            Thunder => "".to_string(),
            Ice => "".to_string(),
            Dragon => " ".to_string(),
            Poison => "󰻊".to_string(),
            None => "".to_string(),
        }
    }
}

impl Display for MonsterElements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MonsterElements::*;
        match self {
            Fire => write!(f, "Fire"),
            Water => write!(f, "Water"),
            Thunder => write!(f, "Thunder"),
            Ice => write!(f, "Ice"),
            Dragon => write!(f, "Dragon"),
            Poison => write!(f, "Poison"),
            None => write!(f, "None"),
        }
    }
}

impl<'de> Deserialize<'de> for MonsterElements {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(MonsterElementsVisitor)
    }
}

pub struct MonsterElementsVisitor;

impl<'de> Visitor<'de> for MonsterElementsVisitor {
    type Value = MonsterElements;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Monster Elements")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use MonsterElements::*;

        match v {
            "fire" => Ok(Fire),
            "water" => Ok(Water),
            "thunder" => Ok(Thunder),
            "ice" => Ok(Ice),
            "dragon" => Ok(Dragon),
            "poison" => Ok(Poison),
            "none" => Ok(None),
            _ => Ok(None),
        }
    }
}

pub trait Monster {
    fn name(&self) -> String;
    fn elements(&self) -> MonsterElements;
    fn desc_ability(&self) -> MonsterDescriptionAbility;
}
