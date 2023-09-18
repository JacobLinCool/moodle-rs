use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The plan id
    #[serde(rename = "planid")]
    pub r#planid: Option<i64>,
    /// The competency id we are moving
    #[serde(rename = "competencyidfrom")]
    pub r#competencyidfrom: Option<i64>,
    /// The competency id we are moving to
    #[serde(rename = "competencyidto")]
    pub r#competencyidto: Option<i64>,
}
