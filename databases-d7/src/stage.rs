use crate::{CmlString, PairData};
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VictoryCondition {
    pub condition: PairData,
    pub logic: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefeatCondition {
    pub condition: PairData,
    pub logic: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StageData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,

    #[serde(rename = "refID")]
    pub ref_id: i32,

    pub name: CmlString,

    #[serde(rename = "mapID")]
    pub map_id: i32,

    pub map_type: i32,

    #[serde(rename = "areaID")]
    pub area_id: i32,

    pub cond_on_flag: Vec<i32>,
    pub cond_off_flag: Vec<i32>,
    pub bonus_rank: i32,
    pub stage_mission: i32,
    pub victory_list: Vec<VictoryCondition>,
    pub defeat_list: Vec<DefeatCondition>,
    pub condition_text: i32,
    pub bgm_no: i32,

    #[serde(rename = "texID")]
    pub tex_id: i32,

    pub meta_script: String,
    pub start_demo_no: i32,
    pub end_event_no: i32,
    pub lose_event_no: i32,
    pub r#type: i32,
    pub start_pos: Vec<i32>,
    pub goal_pos: Vec<i32>,
    pub sortie_direction: i32,
    pub sortie_stock: i32,

    #[serde(rename = "placementTownID")]
    pub placement_town_id: i32,

    #[serde(rename = "placementBattleID")]
    pub placement_battle_id: i32,

    #[serde(rename = "placementEnemyBasePanelID")]
    pub placement_enemy_base_panel_id: Vec<i32>,

    #[serde(rename = "geoPanelID")]
    pub geo_panel_id: i32,

    pub clear_on_flag: Vec<i32>,
    pub clear_off_flag: Vec<i32>,
    pub difficulty: i32,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for StageData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
