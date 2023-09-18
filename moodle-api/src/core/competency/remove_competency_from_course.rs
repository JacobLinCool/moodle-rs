use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// The competency id
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
}
