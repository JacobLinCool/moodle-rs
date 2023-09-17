use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Data base record id for the template
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}
