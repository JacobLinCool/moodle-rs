use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// group id, 0 means that the function will determine the user group
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDataStudentsItemAttemptsItem {
    /// Attempt number.
    #[serde(rename = "try")]
    pub r#try: Option<i64>,
    /// Attempt grade.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// Attempt time started.
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// Attempt last time continued.
    #[serde(rename = "timeend")]
    pub r#timeend: Option<i64>,
    /// Attempt time ended.
    #[serde(rename = "end")]
    pub r#end: Option<i64>,
}

pub type r#ReturnsDataStudentsItemAttempts = Vec<ReturnsDataStudentsItemAttemptsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDataStudentsItem {
    /// User id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// User full name.
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// Best grade.
    #[serde(rename = "bestgrade")]
    pub r#bestgrade: Option<f64>,
    #[serde(rename = "attempts")]
    pub r#attempts: Option<r#ReturnsDataStudentsItemAttempts>,
}

/// Students data, including attempts.
pub type r#ReturnsDataStudents = Vec<ReturnsDataStudentsItem>;

/// Attempts overview data (empty for no attemps).
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsData {
    /// True if the lesson was scored.
    #[serde(rename = "lessonscored")]
    pub r#lessonscored: Option<bool>,
    /// Number of attempts.
    #[serde(rename = "numofattempts")]
    pub r#numofattempts: Option<i64>,
    /// Average score.
    #[serde(rename = "avescore")]
    pub r#avescore: Option<f64>,
    /// High score.
    #[serde(rename = "highscore")]
    pub r#highscore: Option<f64>,
    /// Low score.
    #[serde(rename = "lowscore")]
    pub r#lowscore: Option<f64>,
    /// Average time (spent in taking the lesson).
    #[serde(rename = "avetime")]
    pub r#avetime: Option<i64>,
    /// High time.
    #[serde(rename = "hightime")]
    pub r#hightime: Option<i64>,
    /// Low time.
    #[serde(rename = "lowtime")]
    pub r#lowtime: Option<i64>,
    /// Students data, including attempts.
    #[serde(rename = "students")]
    pub r#students: Option<r#ReturnsDataStudents>,
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
    /// Attempts overview data (empty for no attemps).
    #[serde(rename = "data")]
    pub r#data: Option<ReturnsData>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lesson_get_attempts_overview", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lesson_get_attempts_overview", params)
        .await
}
