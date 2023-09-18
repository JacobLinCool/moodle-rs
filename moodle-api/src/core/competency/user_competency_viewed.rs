use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The user competency id
    #[serde(rename = "usercompetencyid")]
    pub r#usercompetencyid: Option<i64>,
}
