use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Workshop instance id.
    #[serde(rename = "workshopid")]
    pub r#workshopid: Option<i64>,
    /// User id (empty or 0 for current user).
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
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
    /// The assessment raw (numeric) grade.
    #[serde(rename = "assessmentrawgrade")]
    pub r#assessmentrawgrade: Option<f64>,
    /// The assessment string grade.
    #[serde(rename = "assessmentlongstrgrade")]
    pub r#assessmentlongstrgrade: Option<String>,
    /// Whether the grade is hidden or not.
    #[serde(rename = "assessmentgradehidden")]
    pub r#assessmentgradehidden: Option<bool>,
    /// The submission raw (numeric) grade.
    #[serde(rename = "submissionrawgrade")]
    pub r#submissionrawgrade: Option<f64>,
    /// The submission string grade.
    #[serde(rename = "submissionlongstrgrade")]
    pub r#submissionlongstrgrade: Option<String>,
    /// Whether the grade is hidden or not.
    #[serde(rename = "submissiongradehidden")]
    pub r#submissiongradehidden: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_workshop_get_grades", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_workshop_get_grades", params).await
}
