use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Feedback instance id
    #[serde(rename = "feedbackid")]
    pub r#feedbackid: Option<i64>,
    /// Course where user completes the feedback (for site feedbacks only).
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedback {
    /// The record id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The feedback instance id this records belongs to.
    #[serde(rename = "feedback")]
    pub r#feedback: Option<i64>,
    /// The user who completed the feedback (0 for anonymous).
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// For guests, this is the session key.
    #[serde(rename = "guestid")]
    pub r#guestid: Option<String>,
    /// The last time the feedback was completed.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// The response number (used when shuffling anonymous responses).
    #[serde(rename = "random_response")]
    pub r#random_response: Option<i64>,
    /// Whether is an anonymous response.
    #[serde(rename = "anonymous_response")]
    pub r#anonymous_response: Option<i64>,
    /// The course id where the feedback was completed.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
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
    #[serde(rename = "feedback")]
    pub r#feedback: Option<ReturnsFeedback>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_feedback_get_current_completed_tmp", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_feedback_get_current_completed_tmp", params)
        .await
}
