use serde::{Deserialize, Serialize};

pub mod act;
pub mod acteffect;
pub mod actfeature;
pub mod actlearn;
pub mod actmap;
pub mod ai;
pub mod aiparts;
pub mod anime;
pub mod animebank;
pub mod archive;
pub mod area;
pub mod battleflag;
pub mod bgm;
pub mod bu;
pub mod characlass;
pub mod character;
pub mod charafeature;
pub mod charazukan;
pub mod cheatsetting;
pub mod doping;
pub mod drink;
pub mod dungeon;
pub mod evility;
pub mod item;
pub mod itemstrengthen;
pub mod job;
pub mod string;
pub mod wish;

pub use self::act::ActData;
pub use self::acteffect::ActEffectData;
pub use self::actfeature::ActFeatureData;
pub use self::actlearn::ActLearnData;
pub use self::actmap::ActMapData;
pub use self::ai::AiData;
pub use self::aiparts::AiPartsData;
pub use self::anime::AnimeData;
pub use self::animebank::AnimeBankData;
pub use self::archive::ArchiveData;
pub use self::area::AreaData;
pub use self::battleflag::BattleFlagData;
pub use self::bgm::BgmData;
pub use self::bu::BuData;
pub use self::characlass::CharaClassData;
pub use self::character::CharaData;
pub use self::charafeature::CharaFeatureData;
pub use self::charazukan::CharaZukanData;
pub use self::cheatsetting::CheatSettingData;
pub use self::doping::DopingData;
pub use self::drink::DrinkData;
pub use self::dungeon::DungeonData;
pub use self::evility::EvilityData;
pub use self::item::ItemData;
pub use self::itemstrengthen::ItemStrengthenData;
pub use self::job::JobData;
pub use self::string::StringData;
pub use self::wish::WishData;

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
