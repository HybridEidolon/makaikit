use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharaZukanData {
    #[serde(rename = "ID")]
    pub id: i32,

    #[serde(rename = "charaID")]
    pub chara_id: i32,

    pub name: CmlString,
    pub use_on_flag: Vec<i32>,
    pub use_off_flag: Vec<i32>,
    pub cond_on_flag: Vec<i32>,
    pub cond_off_flag: Vec<i32>,
    pub r#type: i32,
    pub is_model: i32,
    pub description: Vec<CmlString>,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for CharaZukanData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        ""
    }
}
