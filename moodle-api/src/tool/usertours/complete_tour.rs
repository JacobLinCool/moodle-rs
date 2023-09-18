use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tour ID
    #[serde(rename = "tourid")]
    pub r#tourid: Option<i64>,
    /// Context ID
    #[serde(rename = "context")]
    pub r#context: Option<i64>,
    /// Page URL
    #[serde(rename = "pageurl")]
    pub r#pageurl: Option<String>,
    /// Step ID
    #[serde(rename = "stepid")]
    pub r#stepid: Option<i64>,
    /// Step Number
    #[serde(rename = "stepindex")]
    pub r#stepindex: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tool_usertours_complete_tour", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_usertours_complete_tour", params).await
}
