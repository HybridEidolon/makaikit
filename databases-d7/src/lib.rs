use serde::{Deserialize, Serialize};

pub mod act;
pub mod acteffect;
pub mod actfeature;
pub mod actlearn;
pub mod actmap;
pub mod battleflag;
pub mod bgm;
pub mod characlass;
pub mod character;
pub mod charafeature;
pub mod charazukan;
pub mod cheatsetting;
pub mod job;
pub mod string;

pub use self::act::ActData;
pub use self::acteffect::ActEffectData;
pub use self::actfeature::ActFeatureData;
pub use self::actlearn::ActLearnData;
pub use self::actmap::ActMapData;
pub use self::battleflag::BattleFlagData;
pub use self::bgm::BgmData;
pub use self::characlass::CharaClassData;
pub use self::character::CharaData;
pub use self::charafeature::CharaFeatureData;
pub use self::charazukan::CharaZukanData;
pub use self::cheatsetting::CheatSettingData;
pub use self::job::JobData;
pub use self::string::StringData;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CmlString {
    pub jp: String,
    pub en: String,
    pub fr: String,
    pub zh_cn: String,
    pub zh_cht: String,
    pub kr: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PairData {
    pub key: i32,
    pub value: i32,
}
