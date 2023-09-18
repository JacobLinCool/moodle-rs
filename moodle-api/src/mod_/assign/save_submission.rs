use serde::{self, Deserialize, Serialize};

/// Editor structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPlugindataOnlinetextEditor {
    /// The text for this submission.
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// The format for this submission
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// The draft area id for files attached to the submission
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPlugindata {
    /// Editor structure
    #[serde(rename = "onlinetext_editor")]
    pub r#onlinetext_editor: Option<ParamsPlugindataOnlinetextEditor>,
    /// The id of a draft area containing files for this submission.
    #[serde(rename = "files_filemanager")]
    pub r#files_filemanager: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The assignment id to operate on
    #[serde(rename = "assignmentid")]
    pub r#assignmentid: Option<i64>,
    #[serde(rename = "plugindata")]
    pub r#plugindata: Option<ParamsPlugindata>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
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
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_assign_save_submission", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_save_submission", params).await
}
