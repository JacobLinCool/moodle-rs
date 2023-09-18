use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptionsItem {
    /// The allowed keys (value format) are: discussionsubscribe (bool); subscribe to the discussion?, default to true discussionpinned    (bool); is the discussion pinned, default to false inlineattachmentsid              (int); the draft file area id for inline attachments attachmentsid       (int); the draft file area id for attachments
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the option, This param is validated in the external function.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Options
pub type r#ParamsOptions = Vec<ParamsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Forum instance ID
    #[serde(rename = "forumid")]
    pub r#forumid: Option<i64>,
    /// New Discussion subject
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// New Discussion message (only html format allowed)
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// The group, default to 0
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Options
    #[serde(rename = "options")]
    pub r#options: Option<r#ParamsOptions>,
}

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
    /// New Discussion ID
    #[serde(rename = "discussionid")]
    pub r#discussionid: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_forum_add_discussion", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_forum_add_discussion", params).await
}
