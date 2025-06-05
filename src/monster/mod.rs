use serde::Deserialize;

#[allow(clippy::module_inception)]
pub mod monster;

#[derive(Deserialize, Default, Clone)]
pub struct MonsterDescText {
    pub original: String,
    pub desc: Vec<String>,
}
