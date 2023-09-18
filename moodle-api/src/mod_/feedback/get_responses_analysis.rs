use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Feedback instance id
    #[serde(rename = "feedbackid")]
    pub r#feedbackid: Option<i64>,
    /// Group id, 0 means that the function will determine the user group
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// The page of records to return.
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The number of records to return per page
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
    /// Course where user completes the feedback (for site feedbacks only).
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttemptsItemResponsesItem {
    /// Response id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Response name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Response ready for output
    #[serde(rename = "printval")]
    pub r#printval: Option<String>,
    /// Response raw value
    #[serde(rename = "rawval")]
    pub r#rawval: Option<String>,
}

pub type r#ReturnsAttemptsItemResponses = Vec<ReturnsAttemptsItemResponsesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttemptsItem {
    /// Completed id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// User who responded
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Time modified for the response
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// User full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    #[serde(rename = "responses")]
    pub r#responses: Option<r#ReturnsAttemptsItemResponses>,
}

pub type r#ReturnsAttempts = Vec<ReturnsAttemptsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnonattemptsItemResponsesItem {
    /// Response id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Response name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Response ready for output
    #[serde(rename = "printval")]
    pub r#printval: Option<String>,
    /// Response raw value
    #[serde(rename = "rawval")]
    pub r#rawval: Option<String>,
}

pub type r#ReturnsAnonattemptsItemResponses = Vec<ReturnsAnonattemptsItemResponsesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnonattemptsItem {
    /// Completed id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Response number
    #[serde(rename = "number")]
    pub r#number: Option<i64>,
    #[serde(rename = "responses")]
    pub r#responses: Option<r#ReturnsAnonattemptsItemResponses>,
}

pub type r#ReturnsAnonattempts = Vec<ReturnsAnonattemptsItem>;

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
    /// Total responses count.
    #[serde(rename = "totalattempts")]
    pub r#totalattempts: Option<i64>,
    #[serde(rename = "anonattempts")]
    pub r#anonattempts: Option<r#ReturnsAnonattempts>,
    /// Total anonymous responses count.
    #[serde(rename = "totalanonattempts")]
    pub r#totalanonattempts: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_feedback_get_responses_analysis", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_feedback_get_responses_analysis", params)
        .await
}
