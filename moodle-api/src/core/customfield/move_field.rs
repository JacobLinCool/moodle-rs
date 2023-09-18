use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Id of the field to move
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// New parent category id
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// Id of the field before which it needs to be moved
    #[serde(rename = "beforeid")]
    pub r#beforeid: Option<i64>,
}
