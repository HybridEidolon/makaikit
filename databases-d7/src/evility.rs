use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueInfo {
    pub r#type: i32,
    pub arg: Vec<f64>,
    pub max: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub r#type: i32,
    pub arg: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CondInfo {
    pub r#type: i32,
    pub arg: Vec<f32>,
    pub logic: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectInfo {
    pub r#type: i32,
    pub type_arg: i32,
    pub value: ValueInfo,
    pub target: TargetInfo,
    pub cond: Vec<CondInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvilityData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: CmlString,
    pub help: CmlString,
    pub icon: i32,

    #[serde(rename = "sortID")]
    pub sort_id: i32,

    pub cost: i32,
    pub mana: i32,
    pub is_unique: i32,
    pub is_shop: i32,
    pub is_enemy_only: i32,
    pub is_not_export: i32,
    pub not_rank_battle: i32,
    pub cond_on_flag: Vec<i32>,
    pub cond_off_flag: Vec<i32>,
    pub r#type: Vec<i32>,
    pub effect: Vec<EffectInfo>,
    pub version: i32,
    pub region: u32,
    pub product: u32,
}

impl DatabaseRecord for EvilityData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
