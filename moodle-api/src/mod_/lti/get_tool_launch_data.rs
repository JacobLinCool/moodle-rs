use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// external tool instance id
    #[serde(rename = "toolid")]
    pub r#toolid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsParametersItem {
    /// Parameter name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Parameter value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ReturnsParameters = Vec<ReturnsParametersItem>;

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
    /// Endpoint URL
    #[serde(rename = "endpoint")]
    pub r#endpoint: Option<String>,
    #[serde(rename = "parameters")]
    pub r#parameters: Option<r#ReturnsParameters>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lti_get_tool_launch_data", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lti_get_tool_launch_data", params).await
}
