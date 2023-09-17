use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The id of the user we want to return the number of received contact requests for
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}
