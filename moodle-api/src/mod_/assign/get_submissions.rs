use serde::{self, Deserialize, Serialize};

/// 1 or more assignment ids
pub type r#ParamsAssignmentids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// 1 or more assignment ids
    #[serde(rename = "assignmentids")]
    pub r#assignmentids: Option<r#ParamsAssignmentids>,
    /// status
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// submitted since
    #[serde(rename = "since")]
    pub r#since: Option<i64>,
    /// submitted before
    #[serde(rename = "before")]
    pub r#before: Option<i64>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItemSubmissionsItemPluginsItemFileareasItemFilesItem {
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

/// files
pub type r#ReturnsAssignmentsItemSubmissionsItemPluginsItemFileareasItemFiles =
    Vec<ReturnsAssignmentsItemSubmissionsItemPluginsItemFileareasItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItemSubmissionsItemPluginsItemFileareasItem {
    /// file area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsAssignmentsItemSubmissionsItemPluginsItemFileareasItemFiles>,
}

/// fileareas
pub type r#ReturnsAssignmentsItemSubmissionsItemPluginsItemFileareas =
    Vec<ReturnsAssignmentsItemSubmissionsItemPluginsItemFileareasItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItemSubmissionsItemPluginsItemEditorfieldsItem {
    /// field name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// field description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// field value
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
}

/// editorfields
pub type r#ReturnsAssignmentsItemSubmissionsItemPluginsItemEditorfields =
    Vec<ReturnsAssignmentsItemSubmissionsItemPluginsItemEditorfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItemSubmissionsItemPluginsItem {
    /// submission plugin type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// submission plugin name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// fileareas
    #[serde(rename = "fileareas")]
    pub r#fileareas: Option<r#ReturnsAssignmentsItemSubmissionsItemPluginsItemFileareas>,
    /// editorfields
    #[serde(rename = "editorfields")]
    pub r#editorfields: Option<r#ReturnsAssignmentsItemSubmissionsItemPluginsItemEditorfields>,
}

/// plugins
pub type r#ReturnsAssignmentsItemSubmissionsItemPlugins =
    Vec<ReturnsAssignmentsItemSubmissionsItemPluginsItem>;

/// submission info
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItemSubmissionsItem {
    /// submission id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// student id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// attempt number
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// submission creation time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// submission last modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// submission start time
    #[serde(rename = "timestarted")]
    pub r#timestarted: Option<i64>,
    /// submission status
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// assignment id
    #[serde(rename = "assignment")]
    pub r#assignment: Option<i64>,
    /// latest attempt
    #[serde(rename = "latest")]
    pub r#latest: Option<i64>,
    /// plugins
    #[serde(rename = "plugins")]
    pub r#plugins: Option<r#ReturnsAssignmentsItemSubmissionsItemPlugins>,
    /// Grading status.
    #[serde(rename = "gradingstatus")]
    pub r#gradingstatus: Option<String>,
}

pub type r#ReturnsAssignmentsItemSubmissions = Vec<ReturnsAssignmentsItemSubmissionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItem {
    /// assignment id
    #[serde(rename = "assignmentid")]
    pub r#assignmentid: Option<i64>,
    #[serde(rename = "submissions")]
    pub r#submissions: Option<r#ReturnsAssignmentsItemSubmissions>,
}

/// assignment submissions
pub type r#ReturnsAssignments = Vec<ReturnsAssignmentsItem>;

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
    /// assignment submissions
    #[serde(rename = "assignments")]
    pub r#assignments: Option<r#ReturnsAssignments>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_assign_get_submissions", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_get_submissions", params).await
}
