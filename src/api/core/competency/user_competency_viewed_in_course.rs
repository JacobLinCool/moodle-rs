use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The competency id
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// The user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}
