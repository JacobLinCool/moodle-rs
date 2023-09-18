use serde::{self, Deserialize, Serialize};

/// Array of course ids.
pub type r#ParamsCourseids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of course ids.
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsCourseids>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWikisItemIntrofilesItem {
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
pub type r#ReturnsWikisItemIntrofiles = Vec<ReturnsWikisItemIntrofilesItem>;

/// Wikis
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWikisItem {
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
    pub r#introfiles: Option<r#ReturnsWikisItemIntrofiles>,
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
    /// Time of creation.
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time of last modification.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// First page title.
    #[serde(rename = "firstpagetitle")]
    pub r#firstpagetitle: Option<String>,
    /// Wiki mode (individual, collaborative).
    #[serde(rename = "wikimode")]
    pub r#wikimode: Option<String>,
    /// Wiki's default format (html, creole, nwiki).
    #[serde(rename = "defaultformat")]
    pub r#defaultformat: Option<String>,
    /// 1 if format is forced, 0 otherwise.
    #[serde(rename = "forceformat")]
    pub r#forceformat: Option<i64>,
    /// Edit begin.
    #[serde(rename = "editbegin")]
    pub r#editbegin: Option<i64>,
    /// Edit end.
    #[serde(rename = "editend")]
    pub r#editend: Option<i64>,
    /// True if user can create pages.
    #[serde(rename = "cancreatepages")]
    pub r#cancreatepages: Option<bool>,
}

pub type r#ReturnsWikis = Vec<ReturnsWikisItem>;

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
    #[serde(rename = "wikis")]
    pub r#wikis: Option<r#ReturnsWikis>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_wiki_get_wikis_by_courses", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_wiki_get_wikis_by_courses", params).await
}
