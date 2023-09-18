use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The model id
    #[serde(rename = "query")]
    pub r#query: Option<String>,
    /// The model id
    #[serde(rename = "modelid")]
    pub r#modelid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// ID of the context
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The context name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_analytics_potential_contexts", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_analytics_potential_contexts", params)
        .await
}
