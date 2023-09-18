use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// context id
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
    /// file content
    #[serde(rename = "filecontent")]
    pub r#filecontent: Option<String>,
    /// The context level to put the file in, (block, course, coursecat, system, user, module)
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// The Instance id of item associated with the context level
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
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
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_files_upload", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_files_upload", params).await
}
