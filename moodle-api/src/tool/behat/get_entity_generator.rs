use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Entity type that can be created by a generator.
    #[serde(rename = "entitytype")]
    pub r#entitytype: Option<String>,
}

/// Required fields
pub type r#ReturnsRequired = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Required fields
    #[serde(rename = "required")]
    pub r#required: Option<r#ReturnsRequired>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_behat_get_entity_generator", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_behat_get_entity_generator", params).await
}
