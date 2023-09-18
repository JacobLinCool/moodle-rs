use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Backup filename
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// Context id
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// Backup id
    #[serde(rename = "backupid")]
    pub r#backupid: Option<String>,
}

/// Table row data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Backup file size
    #[serde(rename = "filesize")]
    pub r#filesize: Option<String>,
    /// Backup file URL
    #[serde(rename = "fileurl")]
    pub r#fileurl: Option<String>,
    /// Backup restore URL
    #[serde(rename = "restoreurl")]
    pub r#restoreurl: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_backup_get_async_backup_links_backup", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_backup_get_async_backup_links_backup", params)
        .await
}
