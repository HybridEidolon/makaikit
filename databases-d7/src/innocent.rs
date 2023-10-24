use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnocentData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: CmlString,
    pub help_text: CmlString,
    pub max_value: i64,

    #[serde(rename = "maxValue_Disobedience")]
    pub max_value_disobedience: i64,

    pub sell_point: i64,
    pub sell_point_correction: i32,
    pub probability: [i32; 18],
    pub pwer_type: i32,
    pub r#type: i32,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for InnocentData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
