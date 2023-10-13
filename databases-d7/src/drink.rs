use crate::CmlString;
use makaikit_databases_serde::DatabaseRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DrinkData {
    #[serde(rename = "ID")]
    pub id: i32,

    pub enum_name: String,
    pub name: String,

    #[serde(rename = "HLrate")]
    pub hl_rate: i32,

    #[serde(rename = "KarmaRate")]
    pub karma_rate: i32,

    #[serde(rename = "seqID")]
    pub seq_id: i32,

    #[serde(rename = "selectSeqID")]
    pub select_seq_id: i32,

    #[serde(rename = "UseKarmaRate")]
    pub use_karma_rate: i32,

    #[serde(rename = "UseManaRate")]
    pub use_mana_rate: i32,

    pub version: i32,
    pub region: u32,
    pub product: u32,
    pub platform: u32,
}

impl DatabaseRecord for DrinkData {
    fn database_id(&self) -> i32 {
        self.id
    }

    fn database_enum_name(&self) -> &str {
        &self.enum_name
    }
}
