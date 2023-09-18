use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The user evidence ID.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetenciesItem {
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// parentid
    #[serde(rename = "parentid")]
    pub r#parentid: Option<i64>,
    /// path
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// ruleoutcome
    #[serde(rename = "ruleoutcome")]
    pub r#ruleoutcome: Option<i64>,
    /// ruletype
    #[serde(rename = "ruletype")]
    pub r#ruletype: Option<String>,
    /// ruleconfig
    #[serde(rename = "ruleconfig")]
    pub r#ruleconfig: Option<String>,
    /// scaleid
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// scaleconfiguration
    #[serde(rename = "scaleconfiguration")]
    pub r#scaleconfiguration: Option<String>,
    /// competencyframeworkid
    #[serde(rename = "competencyframeworkid")]
    pub r#competencyframeworkid: Option<i64>,
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// timecreated
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// usermodified
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
}

/// competencies
pub type r#ReturnsCompetencies = Vec<ReturnsCompetenciesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFilesItem {
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// filearea
    #[serde(rename = "filearea")]
    pub r#filearea: Option<String>,
    /// itemid
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// filepath
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// filename
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// isdir
    #[serde(rename = "isdir")]
    pub r#isdir: Option<bool>,
    /// isimage
    #[serde(rename = "isimage")]
    pub r#isimage: Option<bool>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// timecreated
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// filesize
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// author
    #[serde(rename = "author")]
    pub r#author: Option<String>,
    /// license
    #[serde(rename = "license")]
    pub r#license: Option<String>,
    /// filenameshort
    #[serde(rename = "filenameshort")]
    pub r#filenameshort: Option<String>,
    /// filesizeformatted
    #[serde(rename = "filesizeformatted")]
    pub r#filesizeformatted: Option<String>,
    /// icon
    #[serde(rename = "icon")]
    pub r#icon: Option<String>,
    /// timecreatedformatted
    #[serde(rename = "timecreatedformatted")]
    pub r#timecreatedformatted: Option<String>,
    /// timemodifiedformatted
    #[serde(rename = "timemodifiedformatted")]
    pub r#timemodifiedformatted: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

/// files
pub type r#ReturnsFiles = Vec<ReturnsFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// timecreated
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// usermodified
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// canmanage
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// competencycount
    #[serde(rename = "competencycount")]
    pub r#competencycount: Option<i64>,
    /// competencies
    #[serde(rename = "competencies")]
    pub r#competencies: Option<r#ReturnsCompetencies>,
    /// filecount
    #[serde(rename = "filecount")]
    pub r#filecount: Option<i64>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsFiles>,
    /// hasurlorfiles
    #[serde(rename = "hasurlorfiles")]
    pub r#hasurlorfiles: Option<bool>,
    /// urlshort
    #[serde(rename = "urlshort")]
    pub r#urlshort: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_competency_read_user_evidence", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_competency_read_user_evidence", params)
        .await
}
