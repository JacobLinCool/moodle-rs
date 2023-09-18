use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// contextlevel system, course, user...
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// the Instance id of item associated with the context level
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// associated id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// string comment area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// page number (0 based)
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// Sort direction: ASC or DESC
    #[serde(rename = "sortdirection")]
    pub r#sortdirection: Option<String>,
}

/// comment
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCommentsItem {
    /// Comment ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The content text formatted
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// content format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// Time created (timestamp)
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time format
    #[serde(rename = "strftimeformat")]
    pub r#strftimeformat: Option<String>,
    /// URL profile
    #[serde(rename = "profileurl")]
    pub r#profileurl: Option<String>,
    /// fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// Time in human format
    #[serde(rename = "time")]
    pub r#time: Option<String>,
    /// HTML user picture
    #[serde(rename = "avatar")]
    pub r#avatar: Option<String>,
    /// User ID
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Permission to delete=true/false
    #[serde(rename = "delete")]
    pub r#delete: Option<bool>,
}

/// List of comments
pub type r#ReturnsComments = Vec<ReturnsCommentsItem>;

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
    /// List of comments
    #[serde(rename = "comments")]
    pub r#comments: Option<r#ReturnsComments>,
    /// Total number of comments.
    #[serde(rename = "count")]
    pub r#count: Option<i64>,
    /// Number of comments per page.
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
    /// Whether the user can post in this comment area.
    #[serde(rename = "canpost")]
    pub r#canpost: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_comment_get_comments", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_comment_get_comments", params).await
}
