use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The user ID
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The competency ID
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
}
