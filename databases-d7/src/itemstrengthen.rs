use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemStrengthenData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: CmlString,
    pub help_text: CmlString,
    pub cost: i32,
    pub growth_factor: f32,
    pub required_level: i32,
    pub value: i32,

    #[serde(rename = "StrengthCount")]
    pub strength_count: i32,

    pub is_reset: i32,
    pub condition_on_flag: Vec<i32>,
    pub condition_off_flag: Vec<i32>,
    pub set_flag_on: Vec<i32>,
    pub set_flag_off: Vec<i32>,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for ItemStrengthenData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
