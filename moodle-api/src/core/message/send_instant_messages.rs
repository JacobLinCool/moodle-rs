use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsMessagesItem {
    /// id of the user to send the private message
    #[serde(rename = "touserid")]
    pub r#touserid: Option<i64>,
    /// the text of the message
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "textformat")]
    pub r#textformat: Option<i64>,
    /// your own client id for the message. If this id is provided, the fail message id will be returned to you
    #[serde(rename = "clientmsgid")]
    pub r#clientmsgid: Option<String>,
}

pub type r#ParamsMessages = Vec<ParamsMessagesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "messages")]
    pub r#messages: Option<r#ParamsMessages>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// test this to know if it succeeds:  id of the created message if it succeeded, -1 when failed
    #[serde(rename = "msgid")]
    pub r#msgid: Option<i64>,
    /// your own id for the message
    #[serde(rename = "clientmsgid")]
    pub r#clientmsgid: Option<String>,
    /// error message - if it failed
    #[serde(rename = "errormessage")]
    pub r#errormessage: Option<String>,
    /// The text of the message
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// The timecreated timestamp for the message
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// The conversation id for this message
    #[serde(rename = "conversationid")]
    pub r#conversationid: Option<i64>,
    /// The user id who sent the message
    #[serde(rename = "useridfrom")]
    pub r#useridfrom: Option<i64>,
    /// If the user can delete messages in the conversation for all users
    #[serde(rename = "candeletemessagesforallusers")]
    pub r#candeletemessagesforallusers: Option<bool>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_send_instant_messages", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_send_instant_messages", params)
        .await
}
