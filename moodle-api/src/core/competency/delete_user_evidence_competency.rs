use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The user evidence ID.
    #[serde(rename = "userevidenceid")]
    pub r#userevidenceid: Option<i64>,
    /// The competency ID.
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
}
