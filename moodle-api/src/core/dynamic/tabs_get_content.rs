use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tab class
    #[serde(rename = "tab")]
    pub r#tab: Option<String>,
    /// Json-encoded data
    #[serde(rename = "jsondata")]
    pub r#jsondata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Template name
    #[serde(rename = "template")]
    pub r#template: Option<String>,
    /// JSON-encoded data for template
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// JavaScript fragment
    #[serde(rename = "javascript")]
    pub r#javascript: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_dynamic_tabs_get_content", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_dynamic_tabs_get_content", params).await
}
