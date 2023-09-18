use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report ID
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
    /// Column ID
    #[serde(rename = "columnid")]
    pub r#columnid: Option<i64>,
    /// New column position
    #[serde(rename = "position")]
    pub r#position: Option<i64>,
}
