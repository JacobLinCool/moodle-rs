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
pub struct ReturnsLtisItemIntrofilesItem {
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
pub type r#ReturnsLtisItemIntrofiles = Vec<ReturnsLtisItemIntrofilesItem>;

/// Tool
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLtisItem {
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
    pub r#introfiles: Option<r#ReturnsLtisItemIntrofiles>,
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
    /// Time of creation
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time of last modification
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Type id
    #[serde(rename = "typeid")]
    pub r#typeid: Option<i64>,
    /// Tool url
    #[serde(rename = "toolurl")]
    pub r#toolurl: Option<String>,
    /// Secure tool url
    #[serde(rename = "securetoolurl")]
    pub r#securetoolurl: Option<String>,
    /// Instructor choice send name
    #[serde(rename = "instructorchoicesendname")]
    pub r#instructorchoicesendname: Option<String>,
    /// instructor choice send mail address
    #[serde(rename = "instructorchoicesendemailaddr")]
    pub r#instructorchoicesendemailaddr: Option<i64>,
    /// Instructor choice allow roster
    #[serde(rename = "instructorchoiceallowroster")]
    pub r#instructorchoiceallowroster: Option<i64>,
    /// Instructor choice allow setting
    #[serde(rename = "instructorchoiceallowsetting")]
    pub r#instructorchoiceallowsetting: Option<i64>,
    /// instructor custom parameters
    #[serde(rename = "instructorcustomparameters")]
    pub r#instructorcustomparameters: Option<String>,
    /// instructor choice accept grades
    #[serde(rename = "instructorchoiceacceptgrades")]
    pub r#instructorchoiceacceptgrades: Option<i64>,
    /// Enable grades
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
    /// Launch container mode
    #[serde(rename = "launchcontainer")]
    pub r#launchcontainer: Option<i64>,
    /// Resource key
    #[serde(rename = "resourcekey")]
    pub r#resourcekey: Option<String>,
    /// Shared secret
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// Debug launch
    #[serde(rename = "debuglaunch")]
    pub r#debuglaunch: Option<i64>,
    /// Show title launch
    #[serde(rename = "showtitlelaunch")]
    pub r#showtitlelaunch: Option<i64>,
    /// Show description launch
    #[serde(rename = "showdescriptionlaunch")]
    pub r#showdescriptionlaunch: Option<i64>,
    /// Service salt
    #[serde(rename = "servicesalt")]
    pub r#servicesalt: Option<String>,
    /// Alternative icon URL
    #[serde(rename = "icon")]
    pub r#icon: Option<String>,
    /// Secure icon URL
    #[serde(rename = "secureicon")]
    pub r#secureicon: Option<String>,
}

pub type r#ReturnsLtis = Vec<ReturnsLtisItem>;

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
    #[serde(rename = "ltis")]
    pub r#ltis: Option<r#ReturnsLtis>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lti_get_ltis_by_courses", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lti_get_ltis_by_courses", params).await
}
