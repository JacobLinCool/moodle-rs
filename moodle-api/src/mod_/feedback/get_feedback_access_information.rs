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
    /// Whether the user can view the analysis or not.
    #[serde(rename = "canviewanalysis")]
    pub r#canviewanalysis: Option<bool>,
    /// Whether the user can complete the feedback or not.
    #[serde(rename = "cancomplete")]
    pub r#cancomplete: Option<bool>,
    /// Whether the user can submit the feedback or not.
    #[serde(rename = "cansubmit")]
    pub r#cansubmit: Option<bool>,
    /// Whether the user can delete submissions or not.
    #[serde(rename = "candeletesubmissions")]
    pub r#candeletesubmissions: Option<bool>,
    /// Whether the user can view the feedback reports or not.
    #[serde(rename = "canviewreports")]
    pub r#canviewreports: Option<bool>,
    /// Whether the user can edit feedback items or not.
    #[serde(rename = "canedititems")]
    pub r#canedititems: Option<bool>,
    /// Whether the feedback has questions or not.
    #[serde(rename = "isempty")]
    pub r#isempty: Option<bool>,
    /// Whether the feedback has active access time restrictions or not.
    #[serde(rename = "isopen")]
    pub r#isopen: Option<bool>,
    /// Whether the feedback is already submitted or not.
    #[serde(rename = "isalreadysubmitted")]
    pub r#isalreadysubmitted: Option<bool>,
    /// Whether the feedback is anonymous or not.
    #[serde(rename = "isanonymous")]
    pub r#isanonymous: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_feedback_get_feedback_access_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_feedback_get_feedback_access_information", params)
        .await
}
