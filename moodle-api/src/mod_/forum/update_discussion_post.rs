use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptionsItem {
    /// The allowed keys (value format) are: pinned (bool); (only for discussions) whether to pin this discussion or not discussionsubscribe (bool); whether to subscribe to the post or not inlineattachmentsid (int); the draft file area id for inline attachments in the text attachmentsid (int); the draft file area id for attachments
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the option.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Configuration options for the post.
pub type r#ParamsOptions = Vec<ParamsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Post to be updated. It can be a discussion topic post.
    #[serde(rename = "postid")]
    pub r#postid: Option<i64>,
    /// Updated post subject
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// Updated post message (HTML assumed if messageformat is not provided)
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// message format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "messageformat")]
    pub r#messageformat: Option<i64>,
    /// Configuration options for the post.
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
    /// True if the post/discussion was updated, false otherwise.
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_forum_update_discussion_post", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_forum_update_discussion_post", params)
        .await
}
