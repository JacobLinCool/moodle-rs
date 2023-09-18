use serde::{self, Deserialize, Serialize};

/// Array of course ids
pub type r#ParamsCourseids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of course ids
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsCourseids>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsScormsItemIntrofilesItem {
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

/// Files in the introduction
pub type r#ReturnsScormsItemIntrofiles = Vec<ReturnsScormsItemIntrofilesItem>;

/// SCORM
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsScormsItem {
    /// Activity instance id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course module id
    #[serde(rename = "coursemodule")]
    pub r#coursemodule: Option<i64>,
    /// Course id
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// Activity name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Activity introduction
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// Files in the introduction
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsScormsItemIntrofiles>,
    /// Course section id
    #[serde(rename = "section")]
    pub r#section: Option<i64>,
    /// Visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// Group mode
    #[serde(rename = "groupmode")]
    pub r#groupmode: Option<i64>,
    /// Group id
    #[serde(rename = "groupingid")]
    pub r#groupingid: Option<i64>,
    /// Forced activity language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// SCORM zip package size
    #[serde(rename = "packagesize")]
    pub r#packagesize: Option<i64>,
    /// SCORM zip package URL
    #[serde(rename = "packageurl")]
    pub r#packageurl: Option<String>,
    /// SCORM version (SCORM_12, SCORM_13, SCORM_AICC)
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// Max grade
    #[serde(rename = "maxgrade")]
    pub r#maxgrade: Option<i64>,
    /// Grade method
    #[serde(rename = "grademethod")]
    pub r#grademethod: Option<i64>,
    /// What grade
    #[serde(rename = "whatgrade")]
    pub r#whatgrade: Option<i64>,
    /// Maximum number of attemtps
    #[serde(rename = "maxattempt")]
    pub r#maxattempt: Option<i64>,
    /// Status current attempt is forced to "completed"
    #[serde(rename = "forcecompleted")]
    pub r#forcecompleted: Option<bool>,
    /// Controls re-entry behaviour
    #[serde(rename = "forcenewattempt")]
    pub r#forcenewattempt: Option<i64>,
    /// Prevents to launch new attempts once finished
    #[serde(rename = "lastattemptlock")]
    pub r#lastattemptlock: Option<bool>,
    /// How to display attempt status
    #[serde(rename = "displayattemptstatus")]
    pub r#displayattemptstatus: Option<i64>,
    /// Display contents structure
    #[serde(rename = "displaycoursestructure")]
    pub r#displaycoursestructure: Option<bool>,
    /// Package content or ext path hash
    #[serde(rename = "sha1hash")]
    pub r#sha1hash: Option<String>,
    /// MD5 Hash of package file
    #[serde(rename = "md5hash")]
    pub r#md5hash: Option<String>,
    /// Revison number
    #[serde(rename = "revision")]
    pub r#revision: Option<i64>,
    /// First content to launch
    #[serde(rename = "launch")]
    pub r#launch: Option<i64>,
    /// How to skip the content structure page
    #[serde(rename = "skipview")]
    pub r#skipview: Option<i64>,
    /// Disable preview mode?
    #[serde(rename = "hidebrowse")]
    pub r#hidebrowse: Option<bool>,
    /// How to display the SCORM structure in player
    #[serde(rename = "hidetoc")]
    pub r#hidetoc: Option<i64>,
    /// Show navigation buttons
    #[serde(rename = "nav")]
    pub r#nav: Option<i64>,
    /// Navigation position left
    #[serde(rename = "navpositionleft")]
    pub r#navpositionleft: Option<i64>,
    /// Navigation position top
    #[serde(rename = "navpositiontop")]
    pub r#navpositiontop: Option<i64>,
    /// Auto continue?
    #[serde(rename = "auto")]
    pub r#auto: Option<bool>,
    /// Display in current or new window
    #[serde(rename = "popup")]
    pub r#popup: Option<i64>,
    /// Frame width
    #[serde(rename = "width")]
    pub r#width: Option<i64>,
    /// Frame height
    #[serde(rename = "height")]
    pub r#height: Option<i64>,
    /// Available from
    #[serde(rename = "timeopen")]
    pub r#timeopen: Option<i64>,
    /// Available to
    #[serde(rename = "timeclose")]
    pub r#timeclose: Option<i64>,
    /// SCORM type
    #[serde(rename = "scormtype")]
    pub r#scormtype: Option<String>,
    /// Reference to the package
    #[serde(rename = "reference")]
    pub r#reference: Option<String>,
    /// Protect package downloads?
    #[serde(rename = "protectpackagedownloads")]
    pub r#protectpackagedownloads: Option<bool>,
    /// Auto-update frequency for remote packages
    #[serde(rename = "updatefreq")]
    pub r#updatefreq: Option<i64>,
    /// Additional options
    #[serde(rename = "options")]
    pub r#options: Option<String>,
    /// Status passed/completed required?
    #[serde(rename = "completionstatusrequired")]
    pub r#completionstatusrequired: Option<i64>,
    /// Minimum score required
    #[serde(rename = "completionscorerequired")]
    pub r#completionscorerequired: Option<i64>,
    /// Require all scos to return completion status
    #[serde(rename = "completionstatusallscos")]
    pub r#completionstatusallscos: Option<i64>,
    /// Save track data automatically?
    #[serde(rename = "autocommit")]
    pub r#autocommit: Option<bool>,
    /// Time of last modification
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

pub type r#ReturnsScorms = Vec<ReturnsScormsItem>;

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
    #[serde(rename = "scorms")]
    pub r#scorms: Option<r#ReturnsScorms>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_scorm_get_scorms_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_scorm_get_scorms_by_courses", params).await
}
