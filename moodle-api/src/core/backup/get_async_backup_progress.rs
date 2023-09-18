use serde::{self, Deserialize, Serialize};

/// Backup id to get progress for
pub type r#ParamsBackupids = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Backup id to get progress for
    #[serde(rename = "backupids")]
    pub r#backupids: Option<r#ParamsBackupids>,
    /// Context id
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
}

/// Backup completion status
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Backup Status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// Backup progress
    #[serde(rename = "progress")]
    pub r#progress: Option<f64>,
    /// Backup id
    #[serde(rename = "backupid")]
    pub r#backupid: Option<String>,
    /// operation type
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
}

/// Backup data
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_backup_get_async_backup_progress", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_backup_get_async_backup_progress", params)
        .await
}
