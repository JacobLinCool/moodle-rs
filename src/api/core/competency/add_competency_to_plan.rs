use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The plan id
    #[serde(rename = "planid")]
    pub r#planid: Option<i64>,
    /// The competency id
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
}
