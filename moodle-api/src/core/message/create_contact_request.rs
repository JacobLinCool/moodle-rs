use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The id of the user making the request
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The id of the user being requested
    #[serde(rename = "requesteduserid")]
    pub r#requesteduserid: Option<i64>,
}

/// request record
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsRequest {
    /// Message id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// User from id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// User to id
    #[serde(rename = "requesteduserid")]
    pub r#requesteduserid: Option<i64>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
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
    /// request record
    #[serde(rename = "request")]
    pub r#request: Option<ReturnsRequest>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_create_contact_request", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_create_contact_request", params)
        .await
}
