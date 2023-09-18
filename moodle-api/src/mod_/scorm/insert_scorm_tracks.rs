use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsTracksItem {
    /// element name
    #[serde(rename = "element")]
    pub r#element: Option<String>,
    /// element value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ParamsTracks = Vec<ParamsTracksItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// SCO id
    #[serde(rename = "scoid")]
    pub r#scoid: Option<i64>,
    /// attempt number
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
    #[serde(rename = "tracks")]
    pub r#tracks: Option<r#ParamsTracks>,
}

pub type r#ReturnsTrackids = Vec<i64>;

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
    #[serde(rename = "trackids")]
    pub r#trackids: Option<r#ReturnsTrackids>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_scorm_insert_scorm_tracks", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_scorm_insert_scorm_tracks", params).await
}
