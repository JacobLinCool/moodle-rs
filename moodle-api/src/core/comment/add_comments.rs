use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCommentsItem {
    /// contextlevel system, course, user...
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// the id of item associated with the contextlevel
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// component
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// associated id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// string comment area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
}

pub type r#ParamsComments = Vec<ParamsCommentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "comments")]
    pub r#comments: Option<r#ParamsComments>,
}

/// comment
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
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

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_comment_add_comments", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_comment_add_comments", params).await
}
