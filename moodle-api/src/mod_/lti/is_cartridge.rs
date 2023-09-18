use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tool url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// True if the URL is a cartridge
    #[serde(rename = "iscartridge")]
    pub r#iscartridge: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lti_is_cartridge", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lti_is_cartridge", params).await
}
