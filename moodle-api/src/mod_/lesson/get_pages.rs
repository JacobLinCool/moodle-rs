use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// optional password (the lesson may be protected)
    #[serde(rename = "password")]
    pub r#password: Option<String>,
}

/// Page fields
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesItemPage {
    /// The id of this lesson page
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The id of the lesson this page belongs to
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// The id of the page before this one
    #[serde(rename = "prevpageid")]
    pub r#prevpageid: Option<i64>,
    /// The id of the next page in the page sequence
    #[serde(rename = "nextpageid")]
    pub r#nextpageid: Option<i64>,
    /// Identifies the page type of this page
    #[serde(rename = "qtype")]
    pub r#qtype: Option<i64>,
    /// Used to record page type specific options
    #[serde(rename = "qoption")]
    pub r#qoption: Option<i64>,
    /// Used to record page specific layout selections
    #[serde(rename = "layout")]
    pub r#layout: Option<i64>,
    /// Used to record page specific display selections
    #[serde(rename = "display")]
    pub r#display: Option<i64>,
    /// Timestamp for when the page was created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Timestamp for when the page was last modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// The title of this page
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// The contents of this page
    #[serde(rename = "contents")]
    pub r#contents: Option<String>,
    /// contents format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "contentsformat")]
    pub r#contentsformat: Option<i64>,
    /// Toggles display in the left menu block
    #[serde(rename = "displayinmenublock")]
    pub r#displayinmenublock: Option<bool>,
    /// The type of the page [question | structure]
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// The unique identifier for the page type
    #[serde(rename = "typeid")]
    pub r#typeid: Option<i64>,
    /// The string that describes this page type
    #[serde(rename = "typestring")]
    pub r#typestring: Option<String>,
}

/// List of answers ids (empty for content pages in  Moodle 1.9)
pub type r#ReturnsPagesItemAnswerids = Vec<i64>;

/// List of possible page jumps
pub type r#ReturnsPagesItemJumps = Vec<i64>;

/// The lesson pages
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesItem {
    /// Page fields
    #[serde(rename = "page")]
    pub r#page: Option<ReturnsPagesItemPage>,
    /// List of answers ids (empty for content pages in  Moodle 1.9)
    #[serde(rename = "answerids")]
    pub r#answerids: Option<r#ReturnsPagesItemAnswerids>,
    /// List of possible page jumps
    #[serde(rename = "jumps")]
    pub r#jumps: Option<r#ReturnsPagesItemJumps>,
    /// The total number of files attached to the page
    #[serde(rename = "filescount")]
    pub r#filescount: Option<i64>,
    /// The total size of the files
    #[serde(rename = "filessizetotal")]
    pub r#filessizetotal: Option<i64>,
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
    let json = client.post("mod_lesson_get_pages", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lesson_get_pages", params).await
}
