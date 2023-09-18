use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// lesson attempt number
    #[serde(rename = "lessonattempt")]
    pub r#lessonattempt: Option<i64>,
    /// the user id (empty for current user)
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

/// The content pages viewed.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesItem {
    /// The attempt id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The lesson id.
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// The page id.
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// The user who viewed the page.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The lesson attempt number.
    #[serde(rename = "retry")]
    pub r#retry: Option<i64>,
    /// 1 if the next page was calculated randomly.
    #[serde(rename = "flag")]
    pub r#flag: Option<i64>,
    /// The time the page was seen.
    #[serde(rename = "timeseen")]
    pub r#timeseen: Option<i64>,
    /// The next page chosen id.
    #[serde(rename = "nextpageid")]
    pub r#nextpageid: Option<i64>,
}

pub type r#ReturnsPages = Vec<ReturnsPagesItem>;

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
    #[serde(rename = "pages")]
    pub r#pages: Option<r#ReturnsPages>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lesson_get_content_pages_viewed", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lesson_get_content_pages_viewed", params)
        .await
}
