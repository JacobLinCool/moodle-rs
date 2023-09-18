use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of the user, 0 for current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The name of the message processor
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Site configuration status
    #[serde(rename = "systemconfigured")]
    pub r#systemconfigured: Option<bool>,
    /// The user configuration status
    #[serde(rename = "userconfigured")]
    pub r#userconfigured: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_get_message_processor", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_get_message_processor", params)
        .await
}
