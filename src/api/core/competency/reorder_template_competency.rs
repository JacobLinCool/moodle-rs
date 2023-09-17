use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The template id
    #[serde(rename = "templateid")]
    pub r#templateid: Option<i64>,
    /// The competency id we are moving
    #[serde(rename = "competencyidfrom")]
    pub r#competencyidfrom: Option<i64>,
    /// The competency id we are moving to
    #[serde(rename = "competencyidto")]
    pub r#competencyidto: Option<i64>,
}
