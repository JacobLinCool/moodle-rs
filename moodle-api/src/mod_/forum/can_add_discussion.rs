use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Forum instance ID
    #[serde(rename = "forumid")]
    pub r#forumid: Option<i64>,
    /// The group to check, default to active group. Use -1 to check if the user can post in all the groups.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
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
    /// True if the user can add discussions, false otherwise.
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// True if the user can pin discussions, false otherwise.
    #[serde(rename = "canpindiscussions")]
    pub r#canpindiscussions: Option<bool>,
    /// True if the user can add attachments, false otherwise.
    #[serde(rename = "cancreateattachment")]
    pub r#cancreateattachment: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_forum_can_add_discussion", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_forum_can_add_discussion", params).await
}
