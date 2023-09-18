use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// context id Set to -1 to use contextlevel and instanceid.
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// file area
    #[serde(rename = "filearea")]
    pub r#filearea: Option<String>,
    /// associated id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// file path
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// file name
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// timestamp to return files changed after this time.
    #[serde(rename = "modified")]
    pub r#modified: Option<i64>,
    /// The context level for the file location.
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// The instance id for where the file is located.
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsParentsItem {
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    #[serde(rename = "filearea")]
    pub r#filearea: Option<String>,
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
}

pub type r#ReturnsParents = Vec<ReturnsParentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFilesItem {
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    #[serde(rename = "filearea")]
    pub r#filearea: Option<String>,
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    #[serde(rename = "isdir")]
    pub r#isdir: Option<bool>,
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// File size
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// File owner
    #[serde(rename = "author")]
    pub r#author: Option<String>,
    /// File license
    #[serde(rename = "license")]
    pub r#license: Option<String>,
}

pub type r#ReturnsFiles = Vec<ReturnsFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "parents")]
    pub r#parents: Option<r#ReturnsParents>,
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsFiles>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_files_get_files", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_files_get_files", params).await
}
