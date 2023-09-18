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
pub struct ReturnsChoicesItemIntrofilesItem {
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
pub type r#ReturnsChoicesItemIntrofiles = Vec<ReturnsChoicesItemIntrofilesItem>;

/// Choices
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsChoicesItem {
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
    pub r#introfiles: Option<r#ReturnsChoicesItemIntrofiles>,
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
    /// If choice is published
    #[serde(rename = "publish")]
    pub r#publish: Option<bool>,
    /// 0 never, 1 after answer, 2 after close, 3 always
    #[serde(rename = "showresults")]
    pub r#showresults: Option<i64>,
    /// Display mode (vertical, horizontal)
    #[serde(rename = "display")]
    pub r#display: Option<i64>,
    /// Allow update
    #[serde(rename = "allowupdate")]
    pub r#allowupdate: Option<bool>,
    /// Allow multiple choices
    #[serde(rename = "allowmultiple")]
    pub r#allowmultiple: Option<bool>,
    /// Show users who not answered yet
    #[serde(rename = "showunanswered")]
    pub r#showunanswered: Option<bool>,
    /// Include inactive users
    #[serde(rename = "includeinactive")]
    pub r#includeinactive: Option<bool>,
    /// Limit unswers
    #[serde(rename = "limitanswers")]
    pub r#limitanswers: Option<bool>,
    /// Date of opening validity
    #[serde(rename = "timeopen")]
    pub r#timeopen: Option<i64>,
    /// Date of closing validity
    #[serde(rename = "timeclose")]
    pub r#timeclose: Option<i64>,
    /// Show preview before timeopen
    #[serde(rename = "showpreview")]
    pub r#showpreview: Option<bool>,
    /// Time of last modification
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Completion on user submission
    #[serde(rename = "completionsubmit")]
    pub r#completionsubmit: Option<bool>,
    /// Show available spaces
    #[serde(rename = "showavailable")]
    pub r#showavailable: Option<bool>,
}

pub type r#ReturnsChoices = Vec<ReturnsChoicesItem>;

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
    #[serde(rename = "choices")]
    pub r#choices: Option<r#ReturnsChoices>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_choice_get_choices_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_choice_get_choices_by_courses", params)
        .await
}
