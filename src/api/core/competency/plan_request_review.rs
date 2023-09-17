use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The plan ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}
