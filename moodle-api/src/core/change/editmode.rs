use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Set edit mode to
    #[serde(rename = "setmode")]
    pub r#setmode: Option<bool>,
    /// Page context id
    #[serde(rename = "context")]
    pub r#context: Option<i64>,
}

/// editmode
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// The edit mode was changed
    #[serde(rename = "success")]
    pub r#success: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_change_editmode", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_change_editmode", params).await
}
