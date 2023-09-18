use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Component name
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// xAPI activity ID IRI
    #[serde(rename = "activityId")]
    pub r#activity_id: Option<String>,
    /// The xAPI agent json
    #[serde(rename = "agent")]
    pub r#agent: Option<String>,
    /// The xAPI registration UUID
    #[serde(rename = "registration")]
    pub r#registration: Option<String>,
    /// Filter ids stored since the timestamp (exclusive)
    #[serde(rename = "since")]
    pub r#since: Option<String>,
}

/// List of state Ids
pub type r#Returns = Vec<String>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_xapi_get_states", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_xapi_get_states", params).await
}
