use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WishData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: CmlString,
    pub help_text: CmlString,
    pub no: i32,
    pub rank: i32,
    pub difficulty: i32,
    pub bonus_rank: i32,
    pub cost: i32,
    pub correction: i32,
    pub required_level: i32,
    pub bribe_base: i64,
    pub bribe_correction: i32,
    pub is_vote: i32,
    pub is_use_once: i32,
    pub condition_on_flag: Vec<i32>,
    pub condition_off_flag: Vec<i32>,
    pub set_flag_on: Vec<i32>,
    pub set_flag_off: Vec<i32>,

    pub version: i32,
    pub region: u32,
    pub product: u32,
}

impl DatabaseRecord for WishData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
