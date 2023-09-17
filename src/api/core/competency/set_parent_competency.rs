use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The competency id
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// The new competency parent id
    #[serde(rename = "parentid")]
    pub r#parentid: Option<i64>,
}
