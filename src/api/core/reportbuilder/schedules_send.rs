use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report ID
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
    /// Schedule ID
    #[serde(rename = "scheduleid")]
    pub r#scheduleid: Option<i64>,
}
