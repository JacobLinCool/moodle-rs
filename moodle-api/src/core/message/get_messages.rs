use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// the user id who received the message, 0 for any user
    #[serde(rename = "useridto")]
    pub r#useridto: Option<i64>,
    /// the user id who send the message, 0 for any user. -10 or -20 for no-reply or support user
    #[serde(rename = "useridfrom")]
    pub r#useridfrom: Option<i64>,
    /// type of message to return, expected values are: notifications, conversations and both
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// 1 for getting read messages, 0 for unread, 2 for both
    #[serde(rename = "read")]
    pub r#read: Option<i64>,
    /// true for ordering by newest first, false for oldest first
    #[serde(rename = "newestfirst")]
    pub r#newestfirst: Option<bool>,
    /// limit from
    #[serde(rename = "limitfrom")]
    pub r#limitfrom: Option<i64>,
    /// limit number
    #[serde(rename = "limitnum")]
    pub r#limitnum: Option<i64>,
}

/// message
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMessagesItem {
    /// Message id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// User from id
    #[serde(rename = "useridfrom")]
    pub r#useridfrom: Option<i64>,
    /// User to id
    #[serde(rename = "useridto")]
    pub r#useridto: Option<i64>,
    /// The message subject
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// The message text formated
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// The message
    #[serde(rename = "fullmessage")]
    pub r#fullmessage: Option<String>,
    /// fullmessage format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "fullmessageformat")]
    pub r#fullmessageformat: Option<i64>,
    /// The message in html
    #[serde(rename = "fullmessagehtml")]
    pub r#fullmessagehtml: Option<String>,
    /// The shorten message
    #[serde(rename = "smallmessage")]
    pub r#smallmessage: Option<String>,
    /// Is a notification?
    #[serde(rename = "notification")]
    pub r#notification: Option<i64>,
    /// Context URL
    #[serde(rename = "contexturl")]
    pub r#contexturl: Option<String>,
    /// Context URL link name
    #[serde(rename = "contexturlname")]
    pub r#contexturlname: Option<String>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time read
    #[serde(rename = "timeread")]
    pub r#timeread: Option<i64>,
    /// User to full name
    #[serde(rename = "usertofullname")]
    pub r#usertofullname: Option<String>,
    /// User from full name
    #[serde(rename = "userfromfullname")]
    pub r#userfromfullname: Option<String>,
    /// The component that generated the notification
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// The type of notification
    #[serde(rename = "eventtype")]
    pub r#eventtype: Option<String>,
    /// Custom data to be passed to the message processor. The data here is serialised using json_encode().
    #[serde(rename = "customdata")]
    pub r#customdata: Option<String>,
    /// URL for icon, only for notifications.
    #[serde(rename = "iconurl")]
    pub r#iconurl: Option<String>,
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
    let json = client.post("core_message_get_messages", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_message_get_messages", params).await
}
