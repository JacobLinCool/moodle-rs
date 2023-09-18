use serde::{self, Deserialize, Serialize};

/// Affected ids
pub type r#ParamsIds = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// action: cm_hide, cm_show, section_hide, section_show, cm_moveleft...
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Affected ids
    #[serde(rename = "ids")]
    pub r#ids: Option<r#ParamsIds>,
    /// Optional target section id
    #[serde(rename = "targetsectionid")]
    pub r#targetsectionid: Option<i64>,
    /// Optional target cm id
    #[serde(rename = "targetcmid")]
    pub r#targetcmid: Option<i64>,
}
