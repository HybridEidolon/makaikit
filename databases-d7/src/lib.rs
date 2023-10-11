use serde::{Deserialize, Serialize};

pub mod characlass;
pub mod cheatsetting;
pub mod job;
pub mod string;

pub use self::characlass::CharaClassData;
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
