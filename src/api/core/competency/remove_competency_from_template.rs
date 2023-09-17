use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The template id
    #[serde(rename = "templateid")]
    pub r#templateid: Option<i64>,
    /// The competency id
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
}
