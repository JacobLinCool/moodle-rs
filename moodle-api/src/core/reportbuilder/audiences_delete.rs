use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report id
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
    /// Audience instance id
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}
