use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// the question usage id.
    #[serde(rename = "qubaid")]
    pub r#qubaid: Option<i64>,
    /// the question id
    #[serde(rename = "questionid")]
    pub r#questionid: Option<i64>,
    /// the question_attempt id
    #[serde(rename = "qaid")]
    pub r#qaid: Option<i64>,
    /// the slot number within the usage
    #[serde(rename = "slot")]
    pub r#slot: Option<i64>,
    /// computed checksum with the last three arguments and the users username
    #[serde(rename = "checksum")]
    pub r#checksum: Option<String>,
    /// the new state of the flag. true = flagged
    #[serde(rename = "newstate")]
    pub r#newstate: Option<bool>,
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
    let json = client.post("core_question_update_flag", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_question_update_flag", params).await
}
