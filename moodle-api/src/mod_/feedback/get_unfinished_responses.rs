use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Feedback instance id.
    #[serde(rename = "feedbackid")]
    pub r#feedbackid: Option<i64>,
    /// Course where user completes the feedback (for site feedbacks only).
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsResponsesItem {
    /// The record id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The course id this record belongs to.
    #[serde(rename = "course_id")]
    pub r#course_id: Option<i64>,
    /// The item id that was responded.
    #[serde(rename = "item")]
    pub r#item: Option<i64>,
    /// Reference to the feedback_completedtmp table.
    #[serde(rename = "completed")]
    pub r#completed: Option<i64>,
    /// Old field - not used anymore.
    #[serde(rename = "tmp_completed")]
    pub r#tmp_completed: Option<i64>,
    /// The response value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ReturnsResponses = Vec<ReturnsResponsesItem>;

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
    #[serde(rename = "responses")]
    pub r#responses: Option<r#ReturnsResponses>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_feedback_get_unfinished_responses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_feedback_get_unfinished_responses", params)
        .await
}
