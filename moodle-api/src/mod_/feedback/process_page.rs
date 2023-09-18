use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsResponsesItem {
    /// The response name (usually type[index]_id).
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The response value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// The data to be processed.
pub type r#ParamsResponses = Vec<ParamsResponsesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Feedback instance id.
    #[serde(rename = "feedbackid")]
    pub r#feedbackid: Option<i64>,
    /// The page being processed.
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The data to be processed.
    #[serde(rename = "responses")]
    pub r#responses: Option<r#ParamsResponses>,
    /// Whether we want to jump to previous page.
    #[serde(rename = "goprevious")]
    pub r#goprevious: Option<bool>,
    /// Course where user completes the feedback (for site feedbacks only).
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
    /// The page to jump to.
    #[serde(rename = "jumpto")]
    pub r#jumpto: Option<i64>,
    /// If the user completed the feedback.
    #[serde(rename = "completed")]
    pub r#completed: Option<bool>,
    /// The completion page contents.
    #[serde(rename = "completionpagecontents")]
    pub r#completionpagecontents: Option<String>,
    /// The link (could be relative) to show after submit.
    #[serde(rename = "siteaftersubmit")]
    pub r#siteaftersubmit: Option<String>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_feedback_process_page", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_feedback_process_page", params).await
}
