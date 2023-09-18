use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPreflightdataItem {
    /// data name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// data value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Preflight required data (like passwords)
pub type r#ParamsPreflightdata = Vec<ParamsPreflightdataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// attempt id
    #[serde(rename = "attemptid")]
    pub r#attemptid: Option<i64>,
    /// Preflight required data (like passwords)
    #[serde(rename = "preflightdata")]
    pub r#preflightdata: Option<r#ParamsPreflightdata>,
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
    /// status: true if success
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
    let json = client.post("mod_quiz_view_attempt_summary", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_quiz_view_attempt_summary", params).await
}
