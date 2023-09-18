use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Id of the user, default to current user.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
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
    /// Number of files in the area.
    #[serde(rename = "filecount")]
    pub r#filecount: Option<i64>,
    /// Number of folders in the area.
    #[serde(rename = "foldercount")]
    pub r#foldercount: Option<i64>,
    /// Total size of the files in the area.
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// Total size of the area excluding file references
    #[serde(rename = "filesizewithoutreferences")]
    pub r#filesizewithoutreferences: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_user_get_private_files_info", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_user_get_private_files_info", params)
        .await
}
