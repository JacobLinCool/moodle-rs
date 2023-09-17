use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// the user id who received the message, 0 for current user
    #[serde(rename = "useridto")]
    pub r#useridto: Option<i64>,
    /// true for ordering by newest first, false for oldest first
    #[serde(rename = "newestfirst")]
    pub r#newestfirst: Option<bool>,
    /// the number of results to return
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
    /// offset the result set by a given amount
    #[serde(rename = "offset")]
    pub r#offset: Option<i64>,
}

/// message
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsNotificationsItem {
    /// Notification id (this is not guaranteed to be unique within this result set)
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// User from id
    #[serde(rename = "useridfrom")]
    pub r#useridfrom: Option<i64>,
    /// User to id
    #[serde(rename = "useridto")]
    pub r#useridto: Option<i64>,
    /// The notification subject
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// The notification subject shortened with ellipsis
    #[serde(rename = "shortenedsubject")]
    pub r#shortenedsubject: Option<String>,
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
    /// Context URL
    #[serde(rename = "contexturl")]
    pub r#contexturl: Option<String>,
    /// Context URL link name
    #[serde(rename = "contexturlname")]
    pub r#contexturlname: Option<String>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time created in a pretty format
    #[serde(rename = "timecreatedpretty")]
    pub r#timecreatedpretty: Option<String>,
    /// Time read
    #[serde(rename = "timeread")]
    pub r#timeread: Option<i64>,
    /// notification read status
    #[serde(rename = "read")]
    pub r#read: Option<bool>,
    /// notification deletion status
    #[serde(rename = "deleted")]
    pub r#deleted: Option<bool>,
    /// URL for notification icon
    #[serde(rename = "iconurl")]
    pub r#iconurl: Option<String>,
    /// The component that generated the notification
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// The type of notification
    #[serde(rename = "eventtype")]
    pub r#eventtype: Option<String>,
    /// Custom data to be passed to the message processor. The data here is serialised using json_encode().
    #[serde(rename = "customdata")]
    pub r#customdata: Option<String>,
}

pub type r#ReturnsNotifications = Vec<ReturnsNotificationsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "notifications")]
    pub r#notifications: ReturnsNotifications,
    /// the number of unread message for the given user
    #[serde(rename = "unreadcount")]
    pub r#unreadcount: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("message_popup_get_popup_notifications", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
