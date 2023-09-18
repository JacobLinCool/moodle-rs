use serde::{self, Deserialize, Serialize};

pub type r#ParamsConversationids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The id of the user who is blocking
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    #[serde(rename = "conversationids")]
    pub r#conversationids: Option<r#ParamsConversationids>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
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
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_mute_conversations", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_message_mute_conversations", params).await
}
