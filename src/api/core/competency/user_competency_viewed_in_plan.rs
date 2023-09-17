use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The competency id
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// The user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The plan id
    #[serde(rename = "planid")]
    pub r#planid: Option<i64>,
}
