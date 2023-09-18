use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tool proxy id
    #[serde(rename = "toolproxyid")]
    pub r#toolproxyid: Option<i64>,
    /// Orphaned tool types only
    #[serde(rename = "orphanedonly")]
    pub r#orphanedonly: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Total number of tool types and proxies
    #[serde(rename = "count")]
    pub r#count: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lti_get_tool_types_and_proxies_count", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lti_get_tool_types_and_proxies_count", params)
        .await
}
