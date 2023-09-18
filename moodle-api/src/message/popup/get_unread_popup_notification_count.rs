use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// the user id who received the message, 0 for any user
    #[serde(rename = "useridto")]
    pub r#useridto: Option<i64>,
}
