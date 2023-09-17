use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// itemid
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
}
