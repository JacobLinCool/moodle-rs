use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsMessagesItem {
    /// the text of the message
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "textformat")]
    pub r#textformat: Option<i64>,
}

pub type r#ParamsMessages = Vec<ParamsMessagesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of the conversation
    #[serde(rename = "conversationid")]
    pub r#conversationid: Option<i64>,
    #[serde(rename = "messages")]
    pub r#messages: Option<r#ParamsMessages>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// The id of the message
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The id of the user who sent the message
    #[serde(rename = "useridfrom")]
    pub r#useridfrom: Option<i64>,
    /// The text of the message
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// The timecreated timestamp for the message
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_send_messages_to_conversation", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_send_messages_to_conversation", params)
        .await
}
