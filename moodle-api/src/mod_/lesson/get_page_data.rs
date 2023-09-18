use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// the page id
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// optional password (the lesson may be protected)
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// if we want to review just after finishing (1 hour margin)
    #[serde(rename = "review")]
    pub r#review: Option<bool>,
    /// if we must return the complete page contents once rendered
    #[serde(rename = "returncontents")]
    pub r#returncontents: Option<bool>,
}

/// Page fields
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPage {
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

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsContentfilesItem {
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

/// List of files.
pub type r#ReturnsContentfiles = Vec<ReturnsContentfilesItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnswersItemAnswerfilesItem {
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

/// List of files.
pub type r#ReturnsAnswersItemAnswerfiles = Vec<ReturnsAnswersItemAnswerfilesItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnswersItemResponsefilesItem {
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

/// List of files.
pub type r#ReturnsAnswersItemResponsefiles = Vec<ReturnsAnswersItemResponsefilesItem>;

/// The page answers
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnswersItem {
    /// The ID of this answer in the database
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// List of files.
    #[serde(rename = "answerfiles")]
    pub r#answerfiles: Option<r#ReturnsAnswersItemAnswerfiles>,
    /// List of files.
    #[serde(rename = "responsefiles")]
    pub r#responsefiles: Option<r#ReturnsAnswersItemResponsefiles>,
    /// Identifies where the user goes upon completing a page with this answer
    #[serde(rename = "jumpto")]
    pub r#jumpto: Option<i64>,
    /// The grade this answer is worth
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
    /// The score this answer will give
    #[serde(rename = "score")]
    pub r#score: Option<i64>,
    /// Used to store options for the answer
    #[serde(rename = "flags")]
    pub r#flags: Option<i64>,
    /// A timestamp of when the answer was created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// A timestamp of when the answer was modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Possible answer text
    #[serde(rename = "answer")]
    pub r#answer: Option<String>,
    /// answer format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "answerformat")]
    pub r#answerformat: Option<i64>,
    /// Response text for the answer
    #[serde(rename = "response")]
    pub r#response: Option<String>,
    /// response format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "responseformat")]
    pub r#responseformat: Option<i64>,
}

pub type r#ReturnsAnswers = Vec<ReturnsAnswersItem>;

/// The lesson generated messages
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMessagesItem {
    /// Message.
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Message type: usually a CSS identifier like: success, info, warning, error, notifyproblem, notifyerror, notifytiny, notifysuccess
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

pub type r#ReturnsMessages = Vec<ReturnsMessagesItem>;

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
    /// Page fields
    #[serde(rename = "page")]
    pub r#page: Option<ReturnsPage>,
    /// New page id (if a jump was made)
    #[serde(rename = "newpageid")]
    pub r#newpageid: Option<i64>,
    /// Page html content
    #[serde(rename = "pagecontent")]
    pub r#pagecontent: Option<String>,
    /// The ongoing score message
    #[serde(rename = "ongoingscore")]
    pub r#ongoingscore: Option<String>,
    /// Progress percentage in the lesson
    #[serde(rename = "progress")]
    pub r#progress: Option<i64>,
    /// List of files.
    #[serde(rename = "contentfiles")]
    pub r#contentfiles: Option<r#ReturnsContentfiles>,
    #[serde(rename = "answers")]
    pub r#answers: Option<r#ReturnsAnswers>,
    #[serde(rename = "messages")]
    pub r#messages: Option<r#ReturnsMessages>,
    /// Whether we should display the menu or not in this page.
    #[serde(rename = "displaymenu")]
    pub r#displaymenu: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lesson_get_page_data", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lesson_get_page_data", params).await
}
