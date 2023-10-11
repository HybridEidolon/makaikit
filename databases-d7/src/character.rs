use crate::{CmlString, PairData};
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvilityInfo {
    #[serde(rename = "ID")]
    pub id: i32,

    pub learn_lv: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActInfo {
    #[serde(rename = "ID")]
    pub id: i32,

    pub learn_lv: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharaData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,

    pub name: CmlString,
    pub tribal_name: CmlString,

    #[serde(rename = "refID")]
    pub ref_id: i32,

    #[serde(rename = "classID")]
    pub class_id: i32,

    #[serde(rename = "jobID")]
    pub job_id: i32,

    pub job_rank: i32,
    pub sex: i32,
    pub looks: i32,
    pub width: f32,
    pub height: f32,
    pub cell_size: i32,
    pub cell_type: i32,
    pub hit_radius: f32,
    pub activate_radius: f32,
    pub chara_rank: i32,
    pub is_pile_up: i32,

    #[serde(rename = "baseHP")]
    pub base_hp: i32,

    #[serde(rename = "baseSP")]
    pub base_sp: i32,

    pub base_param: [i32; 6],
    pub equip_rate_param: [i32; 8],
    pub attribute_rate: [i32; 4],
    pub weapon_resist: [i32; 8],
    pub weapon_mastarly: [i32; 10],
    pub move_range: i32,
    pub move_type: i32,
    pub jump: i32,
    pub attack_range: i32,
    pub counter: i32,
    pub throw_range: i32,
    pub critical: i32,
    pub level_up_need_exp_correct: i32,
    pub exp_coefficient: i32,
    pub money_coefficient: i32,
    pub mana_coefficient: i32,
    pub job_coefficient: i32,
    pub good_weapon: Vec<i32>,

    #[serde(rename = "modelID")]
    pub model_id: i32,

    pub weapon_hand_type: i32,

    #[serde(rename = "colorID")]
    pub color_id: i32,

    #[serde(rename = "partsColorID")]
    pub parts_color_id: i32,

    #[serde(rename = "exColorID")]
    pub ex_color_id: i32,

    #[serde(rename = "scale2D")]
    pub scale_2d: f32,

    #[serde(rename = "windowFaceID")]
    pub window_face_id: i32,

    #[serde(rename = "cutInID")]
    pub cut_in_id: i32,

    pub feature: Vec<i32>,
    pub feature_add: Vec<i32>,

    #[serde(rename = "helpID")]
    pub help_id: i32,

    pub help_index: i32,

    #[serde(rename = "voiceID")]
    pub voice_id: i32,

    #[serde(rename = "talkID")]
    pub talk_id: i32,

    #[serde(rename = "uniqueEvilityID")]
    pub unique_evility_id: i32,

    pub evility: Vec<EvilityInfo>,

    pub normal_attack: i32,

    #[serde(rename = "normalAttack_Giant")]
    pub normal_attack_giant: i32,

    #[serde(rename = "attack_Giant")]
    pub attack_giant: i32,

    pub act: Vec<ActInfo>,
    pub act_add: Vec<ActInfo>,

    #[serde(rename = "giantEffectID")]
    pub giant_effect_id: i32,

    #[serde(rename = "makaiJingiID")]
    pub makai_jingi_id: i32,

    pub personal: Vec<i32>,
    pub togather_rate: Vec<PairData>,
    pub dead_effect_script_module: String,
    pub dead_effect_script_entry: String,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for CharaData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
