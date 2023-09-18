use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsDataItem {
    /// The assessment data (use WS get_assessment_form_definition for obtaining the data to sent). Apart from that data, you can optionally send: feedbackauthor (str); the feedback for the submission author feedbackauthorformat (int); the format of the feedbackauthor feedbackauthorinlineattachmentsid (int); the draft file area for the editor attachments feedbackauthorattachmentsid (int); the draft file area id for the feedback attachments
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the option.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Assessment data
pub type r#ParamsData = Vec<ParamsDataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Assessment id.
    #[serde(rename = "assessmentid")]
    pub r#assessmentid: Option<i64>,
    /// Assessment data
    #[serde(rename = "data")]
    pub r#data: Option<r#ParamsData>,
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
    /// status: true if the assessment was added or updated false otherwise.
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// Raw percentual grade (0.00000 to 100.00000) for submission.
    #[serde(rename = "rawgrade")]
    pub r#rawgrade: Option<f64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_workshop_update_assessment", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_workshop_update_assessment", params).await
}
