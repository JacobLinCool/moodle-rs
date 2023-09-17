use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// the user id who received the message, 0 for any user
    #[serde(rename = "useridto")]
    pub r#useridto: Option<i64>,
    /// the user id who send the message, 0 for any user. -10 or -20 for no-reply or support user
    #[serde(rename = "useridfrom")]
    pub r#useridfrom: Option<i64>,
    /// mark messages created before this time as read, 0 for all messages
    #[serde(rename = "timecreatedto")]
    pub r#timecreatedto: Option<i64>,
}
