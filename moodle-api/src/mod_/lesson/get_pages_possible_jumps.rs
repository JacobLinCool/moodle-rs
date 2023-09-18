use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
}

/// Jump for a page answer
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsJumpsItem {
    /// The page id
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// The answer id
    #[serde(rename = "answerid")]
    pub r#answerid: Option<i64>,
    /// The jump (page id or type of jump)
    #[serde(rename = "jumpto")]
    pub r#jumpto: Option<i64>,
    /// The real page id (or EOL) to jump
    #[serde(rename = "calculatedjump")]
    pub r#calculatedjump: Option<i64>,
}

pub type r#ReturnsJumps = Vec<ReturnsJumpsItem>;

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
    #[serde(rename = "jumps")]
    pub r#jumps: Option<r#ReturnsJumps>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lesson_get_pages_possible_jumps", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lesson_get_pages_possible_jumps", params)
        .await
}
