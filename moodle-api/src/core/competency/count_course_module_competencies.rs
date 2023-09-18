use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
}
