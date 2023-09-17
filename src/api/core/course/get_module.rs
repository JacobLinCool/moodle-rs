use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// course module id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// section to return to
    #[serde(rename = "sectionreturn")]
    pub r#sectionreturn: Option<i64>,
}
