use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Chat instance id.
    #[serde(rename = "chatid")]
    pub r#chatid: Option<i64>,
    /// The session start time (timestamp).
    #[serde(rename = "sessionstart")]
    pub r#sessionstart: Option<i64>,
    /// The session end time (timestamp).
    #[serde(rename = "sessionend")]
    pub r#sessionend: Option<i64>,
    /// Get messages from users in this group. 0 means that the function will determine the user group
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMessagesItem {
    /// The message record id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The chat id.
    #[serde(rename = "chatid")]
    pub r#chatid: Option<i64>,
    /// The user who wrote the message.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The group this message belongs to.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Whether is a system message or not.
    #[serde(rename = "issystem")]
    pub r#issystem: Option<bool>,
    /// The message text.
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// The message timestamp (indicates when the message was sent).
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
}

pub type r#ReturnsMessages = Vec<ReturnsMessagesItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "messages")]
    pub r#messages: Option<r#ReturnsMessages>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_chat_get_session_messages", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_chat_get_session_messages", params).await
}
