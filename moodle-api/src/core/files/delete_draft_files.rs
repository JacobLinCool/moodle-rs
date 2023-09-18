use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFilesItem {
    /// Path to the file or directory to delete.
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// Name of the file to delete.
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
}

/// Files or directories to be deleted.
pub type r#ParamsFiles = Vec<ParamsFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Item id of the draft file area
    #[serde(rename = "draftitemid")]
    pub r#draftitemid: Option<i64>,
    /// Files or directories to be deleted.
    #[serde(rename = "files")]
    pub r#files: Option<r#ParamsFiles>,
}

pub type r#ReturnsParentpaths = Vec<String>;

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
    #[serde(rename = "parentpaths")]
    pub r#parentpaths: Option<r#ReturnsParentpaths>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_files_delete_draft_files", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_files_delete_draft_files", params).await
}
