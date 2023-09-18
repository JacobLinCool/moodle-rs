use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// sco id
    #[serde(rename = "scoid")]
    pub r#scoid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// attempt number (0 for last attempt)
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
}

/// Tracks data
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDataTracksItem {
    /// Element name
    #[serde(rename = "element")]
    pub r#element: Option<String>,
    /// Element value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ReturnsDataTracks = Vec<ReturnsDataTracksItem>;

/// SCO data
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsData {
    /// Attempt number
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
    #[serde(rename = "tracks")]
    pub r#tracks: Option<r#ReturnsDataTracks>,
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
    /// SCO data
    #[serde(rename = "data")]
    pub r#data: Option<ReturnsData>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_scorm_get_scorm_sco_tracks", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_scorm_get_scorm_sco_tracks", params).await
}
