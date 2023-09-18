use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The cmid of the activity
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
}

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
    /// Activity name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Activity type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// MoodleNet server
    #[serde(rename = "server")]
    pub r#server: Option<String>,
    /// Support page URL
    #[serde(rename = "supportpageurl")]
    pub r#supportpageurl: Option<String>,
    /// MoodleNet issuer id
    #[serde(rename = "issuerid")]
    pub r#issuerid: Option<i64>,
    /// status: true if success
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_moodlenet_get_share_info_activity", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_moodlenet_get_share_info_activity", params)
        .await
}
