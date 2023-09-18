use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// bigbluebuttonbn instance id
    #[serde(rename = "bigbluebuttonbnid")]
    pub r#bigbluebuttonbnid: Option<i64>,
    /// a set of enabled tools
    #[serde(rename = "tools")]
    pub r#tools: Option<String>,
    /// Group ID
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

pub type r#ReturnsTabledataProfileFeatures = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTabledataColumnsItem {
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    #[serde(rename = "width")]
    pub r#width: Option<String>,
    /// Column type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Whether this column is sortable
    #[serde(rename = "sortable")]
    pub r#sortable: Option<bool>,
    /// Whether this column contains HTML
    #[serde(rename = "allowHTML")]
    pub r#allow_h_t_m_l: Option<bool>,
    /// Formatter name
    #[serde(rename = "formatter")]
    pub r#formatter: Option<String>,
}

pub type r#ReturnsTabledataColumns = Vec<ReturnsTabledataColumnsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTabledata {
    #[serde(rename = "activity")]
    pub r#activity: Option<String>,
    #[serde(rename = "ping_interval")]
    pub r#ping_interval: Option<i64>,
    #[serde(rename = "locale")]
    pub r#locale: Option<String>,
    #[serde(rename = "profile_features")]
    pub r#profile_features: Option<r#ReturnsTabledataProfileFeatures>,
    #[serde(rename = "columns")]
    pub r#columns: Option<r#ReturnsTabledataColumns>,
    #[serde(rename = "data")]
    pub r#data: Option<String>,
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
    /// Whether the fetch was successful
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    #[serde(rename = "tabledata")]
    pub r#tabledata: Option<ReturnsTabledata>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_bigbluebuttonbn_get_recordings", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_bigbluebuttonbn_get_recordings", params)
        .await
}
