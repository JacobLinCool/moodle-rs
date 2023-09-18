use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsRequestsItem {
    /// Function name
    #[serde(rename = "function")]
    pub r#function: Option<String>,
    /// JSON-encoded object with named arguments
    #[serde(rename = "arguments")]
    pub r#arguments: Option<String>,
    /// Return raw text
    #[serde(rename = "settingraw")]
    pub r#settingraw: Option<bool>,
    /// Filter text
    #[serde(rename = "settingfilter")]
    pub r#settingfilter: Option<bool>,
    /// Rewrite plugin file URLs
    #[serde(rename = "settingfileurl")]
    pub r#settingfileurl: Option<bool>,
    /// Session language
    #[serde(rename = "settinglang")]
    pub r#settinglang: Option<String>,
}

pub type r#ParamsRequests = Vec<ParamsRequestsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "requests")]
    pub r#requests: Option<r#ParamsRequests>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsResponsesItem {
    /// Whether an exception was thrown.
    #[serde(rename = "error")]
    pub r#error: Option<bool>,
    /// JSON-encoded response data
    #[serde(rename = "data")]
    pub r#data: Option<String>,
    /// JSON-encoed exception info
    #[serde(rename = "exception")]
    pub r#exception: Option<String>,
}

pub type r#ReturnsResponses = Vec<ReturnsResponsesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "responses")]
    pub r#responses: Option<r#ReturnsResponses>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_mobile_call_external_functions", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_mobile_call_external_functions", params)
        .await
}
