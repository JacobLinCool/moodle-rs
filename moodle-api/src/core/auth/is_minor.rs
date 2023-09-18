use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Age
    #[serde(rename = "age")]
    pub r#age: Option<i64>,
    /// Country of residence
    #[serde(rename = "country")]
    pub r#country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// True if the user is considered to be a digital minor, false if not
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_auth_is_minor", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_auth_is_minor", params).await
}
