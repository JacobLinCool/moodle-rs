use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Data base record id for the template
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Boolean to indicate if plans must be deleted
    #[serde(rename = "deleteplans")]
    pub r#deleteplans: Option<bool>,
}
