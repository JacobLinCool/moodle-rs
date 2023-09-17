use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Backup id
    #[serde(rename = "backupid")]
    pub r#backupid: Option<String>,
    /// Context id
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
}

/// Table row data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Restore url
    #[serde(rename = "restoreurl")]
    pub r#restoreurl: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_backup_get_async_backup_links_restore", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
