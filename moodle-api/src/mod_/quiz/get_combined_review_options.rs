use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// quiz instance id
    #[serde(rename = "quizid")]
    pub r#quizid: Option<i64>,
    /// user id (empty for current user)
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSomeoptionsItem {
    /// option name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// option value
    #[serde(rename = "value")]
    pub r#value: Option<i64>,
}

pub type r#ReturnsSomeoptions = Vec<ReturnsSomeoptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAlloptionsItem {
    /// option name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// option value
    #[serde(rename = "value")]
    pub r#value: Option<i64>,
}

pub type r#ReturnsAlloptions = Vec<ReturnsAlloptionsItem>;

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
    #[serde(rename = "someoptions")]
    pub r#someoptions: Option<r#ReturnsSomeoptions>,
    #[serde(rename = "alloptions")]
    pub r#alloptions: Option<r#ReturnsAlloptions>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_quiz_get_combined_review_options", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_quiz_get_combined_review_options", params)
        .await
}
