use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Submission id
    #[serde(rename = "submissionid")]
    pub r#submissionid: Option<i64>,
    /// Submission title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Submission text content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// The format used for the content
    #[serde(rename = "contentformat")]
    pub r#contentformat: Option<i64>,
    /// The draft file area id for inline attachments in the content
    #[serde(rename = "inlineattachmentsid")]
    pub r#inlineattachmentsid: Option<i64>,
    /// The draft file area id for attachments
    #[serde(rename = "attachmentsid")]
    pub r#attachmentsid: Option<i64>,
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
    /// True if the submission was updated false otherwise.
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_workshop_update_submission", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_workshop_update_submission", params).await
}
