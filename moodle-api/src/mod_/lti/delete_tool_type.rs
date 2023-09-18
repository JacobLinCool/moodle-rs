use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tool type id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Tool type id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lti_delete_tool_type", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lti_delete_tool_type", params).await
}
