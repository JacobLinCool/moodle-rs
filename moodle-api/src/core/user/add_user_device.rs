use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// the app id, usually something like com.moodle.moodlemobile
    #[serde(rename = "appid")]
    pub r#appid: Option<String>,
    /// the device name, 'occam' or 'iPhone' etc.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// the device model 'Nexus4' or 'iPad1,1' etc.
    #[serde(rename = "model")]
    pub r#model: Option<String>,
    /// the device platform 'iOS' or 'Android' etc.
    #[serde(rename = "platform")]
    pub r#platform: Option<String>,
    /// the device version '6.1.2' or '4.2.2' etc.
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// the device PUSH token/key/identifier/registration id
    #[serde(rename = "pushid")]
    pub r#pushid: Option<String>,
    /// the device UUID
    #[serde(rename = "uuid")]
    pub r#uuid: Option<String>,
    /// the app generated public key
    #[serde(rename = "publickey")]
    pub r#publickey: Option<String>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsItem = Vec<ReturnsItemItem>;

pub type r#Returns = Vec<r#ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_user_add_user_device", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_user_add_user_device", params).await
}
