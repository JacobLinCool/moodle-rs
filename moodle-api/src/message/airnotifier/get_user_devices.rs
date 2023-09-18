use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// App unique id (usually a reversed domain)
    #[serde(rename = "appid")]
    pub r#appid: Option<String>,
    /// User id, 0 for current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDevicesItem {
    /// Device id (in the message_airnotifier table)
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The app id, something like com.moodle.moodlemobile
    #[serde(rename = "appid")]
    pub r#appid: Option<String>,
    /// The device name, 'occam' or 'iPhone' etc.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The device model 'Nexus4' or 'iPad1,1' etc.
    #[serde(rename = "model")]
    pub r#model: Option<String>,
    /// The device platform 'iOS' or 'Android' etc.
    #[serde(rename = "platform")]
    pub r#platform: Option<String>,
    /// The device version '6.1.2' or '4.2.2' etc.
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// The device PUSH token/key/identifier/registration id
    #[serde(rename = "pushid")]
    pub r#pushid: Option<String>,
    /// The device UUID
    #[serde(rename = "uuid")]
    pub r#uuid: Option<String>,
    /// Whether the device is enabled or not
    #[serde(rename = "enable")]
    pub r#enable: Option<i64>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

/// List of devices
pub type r#ReturnsDevices = Vec<ReturnsDevicesItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
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
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// List of devices
    #[serde(rename = "devices")]
    pub r#devices: Option<r#ReturnsDevices>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("message_airnotifier_get_user_devices", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("message_airnotifier_get_user_devices", params)
        .await
}
