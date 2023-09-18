use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The question id
    #[serde(rename = "questionid")]
    pub r#questionid: Option<i64>,
    /// The updated question status
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// status: true if success
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// statusname: name of the status
    #[serde(rename = "statusname")]
    pub r#statusname: Option<String>,
    /// Error message if error exists
    #[serde(rename = "error")]
    pub r#error: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("qbank_editquestion_set_status", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("qbank_editquestion_set_status", params).await
}
