use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// chat session id (obtained via mod_chat_login_user)
    #[serde(rename = "chatsid")]
    pub r#chatsid: Option<String>,
    /// last time messages were retrieved (epoch time)
    #[serde(rename = "chatlasttime")]
    pub r#chatlasttime: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMessagesItem {
    /// message id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// true if is a system message (like user joined)
    #[serde(rename = "system")]
    pub r#system: Option<bool>,
    /// message text
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// timestamp for the message
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
}

/// list of users
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
    /// list of users
    #[serde(rename = "messages")]
    pub r#messages: Option<r#ReturnsMessages>,
    /// new last time
    #[serde(rename = "chatnewlasttime")]
    pub r#chatnewlasttime: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_chat_get_chat_latest_messages", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_chat_get_chat_latest_messages", params)
        .await
}
