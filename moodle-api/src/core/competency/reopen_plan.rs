use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The plan id
    #[serde(rename = "planid")]
    pub r#planid: Option<i64>,
}
