pub mod characlass;
pub mod gameflag;

pub use self::characlass::*;
pub use self::gameflag::*;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct LocaleStrings {
    pub jp: String,
    pub en: String,
    pub fr: String,
    pub zh_cn: String,
    pub zh_cht: String,
    pub kr: String,
}
