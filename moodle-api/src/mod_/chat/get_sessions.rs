use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Chat instance id.
    #[serde(rename = "chatid")]
    pub r#chatid: Option<i64>,
    /// Get messages from users in this group. 0 means that the function will determine the user group
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Whether to show completed sessions or not.
    #[serde(rename = "showall")]
    pub r#showall: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSessionsItemSessionusersItem {
    /// User id.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Number of messages in the session.
    #[serde(rename = "messagecount")]
    pub r#messagecount: Option<i64>,
}

/// Session users.
pub type r#ReturnsSessionsItemSessionusers = Vec<ReturnsSessionsItemSessionusersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSessionsItem {
    /// Session start time.
    #[serde(rename = "sessionstart")]
    pub r#sessionstart: Option<i64>,
    /// Session end time.
    #[serde(rename = "sessionend")]
    pub r#sessionend: Option<i64>,
    /// Session users.
    #[serde(rename = "sessionusers")]
    pub r#sessionusers: Option<r#ReturnsSessionsItemSessionusers>,
    /// Whether the session is completed or not.
    #[serde(rename = "iscomplete")]
    pub r#iscomplete: Option<bool>,
}

/// list of users
pub type r#ReturnsSessions = Vec<ReturnsSessionsItem>;

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
    #[serde(rename = "sessions")]
    pub r#sessions: Option<r#ReturnsSessions>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_chat_get_sessions", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_chat_get_sessions", params).await
}
