use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The id of the attempt to reopen
    #[serde(rename = "attemptid")]
    pub r#attemptid: Option<i64>,
}
