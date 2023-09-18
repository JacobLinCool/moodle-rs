use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// scorm instance id
    #[serde(rename = "scormid")]
    pub r#scormid: Option<i64>,
    /// attempt number
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDataItemUserdataItem {
    /// element name
    #[serde(rename = "element")]
    pub r#element: Option<String>,
    /// element value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ReturnsDataItemUserdata = Vec<ReturnsDataItemUserdataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDataItemDefaultdataItem {
    /// element name
    #[serde(rename = "element")]
    pub r#element: Option<String>,
    /// element value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ReturnsDataItemDefaultdata = Vec<ReturnsDataItemDefaultdataItem>;

/// SCO data
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDataItem {
    /// sco id
    #[serde(rename = "scoid")]
    pub r#scoid: Option<i64>,
    #[serde(rename = "userdata")]
    pub r#userdata: Option<r#ReturnsDataItemUserdata>,
    #[serde(rename = "defaultdata")]
    pub r#defaultdata: Option<r#ReturnsDataItemDefaultdata>,
}

pub type r#ReturnsData = Vec<ReturnsDataItem>;

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
    #[serde(rename = "data")]
    pub r#data: Option<r#ReturnsData>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_scorm_get_scorm_user_data", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_scorm_get_scorm_user_data", params).await
}
