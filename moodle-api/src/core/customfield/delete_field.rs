use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Custom field ID to delete
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}
