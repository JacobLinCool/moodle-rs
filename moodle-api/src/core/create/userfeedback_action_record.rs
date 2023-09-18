use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The action taken by user
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// The context id of the page the user is in
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
}
