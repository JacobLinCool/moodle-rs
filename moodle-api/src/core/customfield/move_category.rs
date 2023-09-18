use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Category ID to move
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Id of the category before which it needs to be moved
    #[serde(rename = "beforeid")]
    pub r#beforeid: Option<i64>,
}
