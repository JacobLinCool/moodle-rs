use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Data base record id for the plan
    #[serde(rename = "planid")]
    pub r#planid: Option<i64>,
}
