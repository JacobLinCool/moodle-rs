use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// quiz instance id
    #[serde(rename = "quizid")]
    pub r#quizid: Option<i64>,
    /// attempt id, 0 for the user last attempt if exists
    #[serde(rename = "attemptid")]
    pub r#attemptid: Option<i64>,
}

/// list of reasons
pub type r#ReturnsPreventnewattemptreasons = Vec<String>;

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
    /// When the attempt must be submitted (determined by rules).
    #[serde(rename = "endtime")]
    pub r#endtime: Option<i64>,
    /// Whether there is no way the user will ever be allowed to attempt.
    #[serde(rename = "isfinished")]
    pub r#isfinished: Option<bool>,
    /// whether a check is required before the user starts/continues his attempt.
    #[serde(rename = "ispreflightcheckrequired")]
    pub r#ispreflightcheckrequired: Option<bool>,
    /// list of reasons
    #[serde(rename = "preventnewattemptreasons")]
    pub r#preventnewattemptreasons: Option<r#ReturnsPreventnewattemptreasons>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_quiz_get_attempt_access_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_quiz_get_attempt_access_information", params)
        .await
}
