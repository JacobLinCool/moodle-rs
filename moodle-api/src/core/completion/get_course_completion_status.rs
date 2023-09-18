use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course ID
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// User ID
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

/// details
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompletionstatusCompletionsItemDetails {
    /// Type description
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Criteria description
    #[serde(rename = "criteria")]
    pub r#criteria: Option<String>,
    /// Requirement description
    #[serde(rename = "requirement")]
    pub r#requirement: Option<String>,
    /// Status description, can be anything
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

/// Completions
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompletionstatusCompletionsItem {
    /// Completion criteria type
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// Completion criteria Title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Completion status (Yes/No) a % or number
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Completion status (true/false)
    #[serde(rename = "complete")]
    pub r#complete: Option<bool>,
    /// Timestamp for criteria completetion
    #[serde(rename = "timecompleted")]
    pub r#timecompleted: Option<i64>,
    /// details
    #[serde(rename = "details")]
    pub r#details: Option<ReturnsCompletionstatusCompletionsItemDetails>,
}

pub type r#ReturnsCompletionstatusCompletions = Vec<ReturnsCompletionstatusCompletionsItem>;

/// Course status
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompletionstatus {
    /// true if the course is complete, false otherwise
    #[serde(rename = "completed")]
    pub r#completed: Option<bool>,
    /// aggregation method 1 means all, 2 means any
    #[serde(rename = "aggregation")]
    pub r#aggregation: Option<i64>,
    #[serde(rename = "completions")]
    pub r#completions: Option<r#ReturnsCompletionstatusCompletions>,
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

/// Course completion status
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Course status
    #[serde(rename = "completionstatus")]
    pub r#completionstatus: Option<ReturnsCompletionstatus>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_completion_get_course_completion_status", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_completion_get_course_completion_status", params)
        .await
}
