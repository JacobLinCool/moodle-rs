use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFilestokeepItem {
    /// File name.
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// File path.
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
}

/// Only keep these files in the draft file area. Empty for keeping all.
pub type r#ParamsFilestokeep = Vec<ParamsFilestokeepItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Post to prepare the draft area for.
    #[serde(rename = "postid")]
    pub r#postid: Option<i64>,
    /// Area to prepare: attachment or post.
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// The draft item id to use. 0 to generate one.
    #[serde(rename = "draftitemid")]
    pub r#draftitemid: Option<i64>,
    /// Only keep these files in the draft file area. Empty for keeping all.
    #[serde(rename = "filestokeep")]
    pub r#filestokeep: Option<r#ParamsFilestokeep>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFilesItem {
    /// File name.
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// File path.
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// File size.
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// Downloadable file url.
    #[serde(rename = "fileurl")]
    pub r#fileurl: Option<String>,
    /// Time modified.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// File mime type.
    #[serde(rename = "mimetype")]
    pub r#mimetype: Option<String>,
    /// Whether is an external file.
    #[serde(rename = "isexternalfile")]
    pub r#isexternalfile: Option<bool>,
    /// The repository type for external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// Draft area files.
pub type r#ReturnsFiles = Vec<ReturnsFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreaoptionsItem {
    /// Name of option.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Value of option.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Draft file area options.
pub type r#ReturnsAreaoptions = Vec<ReturnsAreaoptionsItem>;

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
    /// Draft item id for the file area.
    #[serde(rename = "draftitemid")]
    pub r#draftitemid: Option<i64>,
    /// Draft area files.
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsFiles>,
    /// Draft file area options.
    #[serde(rename = "areaoptions")]
    pub r#areaoptions: Option<r#ReturnsAreaoptions>,
    /// Message text with URLs rewritten.
    #[serde(rename = "messagetext")]
    pub r#messagetext: Option<String>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_forum_prepare_draft_area_for_post", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_forum_prepare_draft_area_for_post", params)
        .await
}
