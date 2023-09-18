use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// bigbluebuttonbn group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Can join session
    #[serde(rename = "can_join")]
    pub r#can_join: Option<bool>,
    /// course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_bigbluebuttonbn_can_join", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_bigbluebuttonbn_can_join", params).await
}
