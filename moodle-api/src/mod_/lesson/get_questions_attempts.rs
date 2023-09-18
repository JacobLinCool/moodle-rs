use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// lesson attempt number
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
    /// only fetch correct attempts
    #[serde(rename = "correct")]
    pub r#correct: Option<bool>,
    /// only fetch attempts at the given page
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// only fetch attempts of the given user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

/// The question page attempts
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttemptsItem {
    /// The attempt id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The attempt lessonid
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// The attempt pageid
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// The user who did the attempt
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The attempt answerid
    #[serde(rename = "answerid")]
    pub r#answerid: Option<i64>,
    /// The lesson attempt number
    #[serde(rename = "retry")]
    pub r#retry: Option<i64>,
    /// If it was the correct answer
    #[serde(rename = "correct")]
    pub r#correct: Option<i64>,
    /// The complete user answer
    #[serde(rename = "useranswer")]
    pub r#useranswer: Option<String>,
    /// The time the question was seen
    #[serde(rename = "timeseen")]
    pub r#timeseen: Option<i64>,
}

pub type r#ReturnsAttempts = Vec<ReturnsAttemptsItem>;

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
    #[serde(rename = "attempts")]
    pub r#attempts: Option<r#ReturnsAttempts>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lesson_get_questions_attempts", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lesson_get_questions_attempts", params)
        .await
}
