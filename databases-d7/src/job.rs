use crate::CmlString;

use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankInfo {
    pub rank: i32,
    pub need_exp: i32,
    pub grow_param: [i32; 8],

    #[serde(rename = "learnEvilityID")]
    pub learn_evility_id: i32,

    pub open_flag: i32,

    #[serde(rename = "characterID")]
    pub character_id: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: CmlString,
    pub open_flag: i32,
    pub is_make: i32,
    pub rank_info: Vec<RankInfo>,

    #[serde(rename = "ico_raceSelectCutin")]
    pub ico_race_select_cutin: i32,

    #[serde(rename = "seq_raceSelectClassName")]
    pub seq_race_select_class_name: i32,

    #[serde(rename = "seq_raceSelectClassNameRight")]
    pub seq_race_select_class_name_right: i32,

    #[serde(rename = "seq_raceSelectRankNameRight")]
    pub seq_race_select_rank_name_right: Vec<i32>,

    pub chara_make_cost_base: i32,
    pub chara_make_talent_cost_base: i32,
    pub master_on_flag: i32,
    pub ex_color_flags: Vec<i32>,
    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for JobData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
