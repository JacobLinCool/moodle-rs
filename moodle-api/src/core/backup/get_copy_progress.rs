use serde::{self, Deserialize, Serialize};

/// Copy data
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCopiesItem {
    /// Backup id
    #[serde(rename = "backupid")]
    pub r#backupid: Option<String>,
    /// Restore id
    #[serde(rename = "restoreid")]
    pub r#restoreid: Option<String>,
    /// Operation type
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
}

/// Copy data
pub type r#ParamsCopies = Vec<ParamsCopiesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Copy data
    #[serde(rename = "copies")]
    pub r#copies: Option<r#ParamsCopies>,
}

/// Copy completion status
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Copy Status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// Copy progress
    #[serde(rename = "progress")]
    pub r#progress: Option<f64>,
    /// Copy id
    #[serde(rename = "backupid")]
    pub r#backupid: Option<String>,
    /// Operation type
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
}

/// Copy data
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_backup_get_copy_progress", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_backup_get_copy_progress", params).await
}
