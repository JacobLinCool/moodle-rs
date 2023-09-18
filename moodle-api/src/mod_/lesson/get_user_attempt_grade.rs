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

/// Attempt grade
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGrade {
    /// Number of questions answered
    #[serde(rename = "nquestions")]
    pub r#nquestions: Option<i64>,
    /// Number of question attempts
    #[serde(rename = "attempts")]
    pub r#attempts: Option<i64>,
    /// Max points possible
    #[serde(rename = "total")]
    pub r#total: Option<f64>,
    /// Points earned by student
    #[serde(rename = "earned")]
    pub r#earned: Option<f64>,
    /// Calculated percentage grade
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// Number of manually graded questions
    #[serde(rename = "nmanual")]
    pub r#nmanual: Option<i64>,
    /// Point value for manually graded questions
    #[serde(rename = "manualpoints")]
    pub r#manualpoints: Option<f64>,
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
    /// Attempt grade
    #[serde(rename = "grade")]
    pub r#grade: Option<ReturnsGrade>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lesson_get_user_attempt_grade", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lesson_get_user_attempt_grade", params)
        .await
}
