use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Whether to return the block contents.
    #[serde(rename = "returncontents")]
    pub r#returncontents: Option<bool>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBlocksItemContentsFilesItem {
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

/// Block files.
pub type r#ReturnsBlocksItemContentsFiles = Vec<ReturnsBlocksItemContentsFilesItem>;

/// Block contents (if required).
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBlocksItemContents {
    /// Block title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Block contents.
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// content format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "contentformat")]
    pub r#contentformat: Option<i64>,
    /// Block footer.
    #[serde(rename = "footer")]
    pub r#footer: Option<String>,
    /// Block files.
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsBlocksItemContentsFiles>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBlocksItemConfigsItem {
    /// Name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// JSON encoded representation of the config value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// Type (instance or plugin).
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

/// Block instance and plugin configuration settings.
pub type r#ReturnsBlocksItemConfigs = Vec<ReturnsBlocksItemConfigsItem>;

/// Block information.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBlocksItem {
    /// Block instance id.
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
    /// Block name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Block region.
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// Position id.
    #[serde(rename = "positionid")]
    pub r#positionid: Option<i64>,
    /// Whether the block is collapsible.
    #[serde(rename = "collapsible")]
    pub r#collapsible: Option<bool>,
    /// Whether the block is dockable.
    #[serde(rename = "dockable")]
    pub r#dockable: Option<bool>,
    /// Used to order blocks within a region.
    #[serde(rename = "weight")]
    pub r#weight: Option<i64>,
    /// Whether the block is visible.
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// Block contents (if required).
    #[serde(rename = "contents")]
    pub r#contents: Option<ReturnsBlocksItemContents>,
    /// Block instance and plugin configuration settings.
    #[serde(rename = "configs")]
    pub r#configs: Option<r#ReturnsBlocksItemConfigs>,
}

/// List of blocks in the course.
pub type r#ReturnsBlocks = Vec<ReturnsBlocksItem>;

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
    /// List of blocks in the course.
    #[serde(rename = "blocks")]
    pub r#blocks: Option<r#ReturnsBlocks>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_block_get_course_blocks", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_block_get_course_blocks", params).await
}
