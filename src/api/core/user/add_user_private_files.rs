use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// draft area id
    #[serde(rename = "draftid")]
    pub r#draftid: Option<i64>,
}
