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
pub struct ReturnsPagesItemIntrofilesItem {
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
pub type r#ReturnsPagesItemIntrofiles = Vec<ReturnsPagesItemIntrofilesItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesItemContentfilesItem {
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

/// Files in the content
pub type r#ReturnsPagesItemContentfiles = Vec<ReturnsPagesItemContentfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesItem {
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
    pub r#introfiles: Option<r#ReturnsPagesItemIntrofiles>,
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
    /// Page content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// content format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "contentformat")]
    pub r#contentformat: Option<i64>,
    /// Files in the content
    #[serde(rename = "contentfiles")]
    pub r#contentfiles: Option<r#ReturnsPagesItemContentfiles>,
    /// Legacy files flag
    #[serde(rename = "legacyfiles")]
    pub r#legacyfiles: Option<i64>,
    /// Legacy files last control flag
    #[serde(rename = "legacyfileslast")]
    pub r#legacyfileslast: Option<i64>,
    /// How to display the page
    #[serde(rename = "display")]
    pub r#display: Option<i64>,
    /// Display options (width, height)
    #[serde(rename = "displayoptions")]
    pub r#displayoptions: Option<String>,
    /// Incremented when after each file changes, to avoid cache
    #[serde(rename = "revision")]
    pub r#revision: Option<i64>,
    /// Last time the page was modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

pub type r#ReturnsPages = Vec<ReturnsPagesItem>;

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
    #[serde(rename = "pages")]
    pub r#pages: Option<r#ReturnsPages>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_page_get_pages_by_courses", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_page_get_pages_by_courses", params).await
}
