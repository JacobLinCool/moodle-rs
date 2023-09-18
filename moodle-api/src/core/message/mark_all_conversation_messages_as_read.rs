use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The user id who who we are marking the messages as read for
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The conversation id who who we are marking the messages as read for
    #[serde(rename = "conversationid")]
    pub r#conversationid: Option<i64>,
}
