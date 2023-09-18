use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The competency id
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// The related competency id
    #[serde(rename = "relatedcompetencyid")]
    pub r#relatedcompetencyid: Option<i64>,
}
