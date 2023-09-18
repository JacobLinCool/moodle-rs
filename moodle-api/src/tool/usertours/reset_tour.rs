use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tour ID
    #[serde(rename = "tourid")]
    pub r#tourid: Option<i64>,
    /// Context ID
    #[serde(rename = "context")]
    pub r#context: Option<i64>,
    /// Current page location
    #[serde(rename = "pageurl")]
    pub r#pageurl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Tour ID
    #[serde(rename = "startTour")]
    pub r#start_tour: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tool_usertours_reset_tour", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_usertours_reset_tour", params).await
}
