use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// bigbluebuttonbn instance id, this might be a different one from the one set in recordingid in case of importing
    #[serde(rename = "bigbluebuttonbnid")]
    pub r#bigbluebuttonbnid: Option<i64>,
    /// The moodle internal recording ID
    #[serde(rename = "recordingid")]
    pub r#recordingid: Option<i64>,
    /// The action to perform
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// Additional options
    #[serde(rename = "additionaloptions")]
    pub r#additionaloptions: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_bigbluebuttonbn_update_recording", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_bigbluebuttonbn_update_recording", params)
        .await
}
