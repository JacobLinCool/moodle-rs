use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report ID
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
    /// JSON encoded report parameters
    #[serde(rename = "parameters")]
    pub r#parameters: Option<String>,
    /// JSON encoded filter values
    #[serde(rename = "values")]
    pub r#values: Option<String>,
}
