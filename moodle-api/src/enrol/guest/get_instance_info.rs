use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Instance id of guest enrolment plugin.
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsInstanceinfo {
    /// Id of course enrolment instance
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Id of course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Type of enrolment plugin
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Name of enrolment plugin
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Is the enrolment enabled?
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// Is a password required?
    #[serde(rename = "passwordrequired")]
    pub r#passwordrequired: Option<bool>,
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
    #[serde(rename = "instanceinfo")]
    pub r#instanceinfo: Option<ReturnsInstanceinfo>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("enrol_guest_get_instance_info", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("enrol_guest_get_instance_info", params).await
}
