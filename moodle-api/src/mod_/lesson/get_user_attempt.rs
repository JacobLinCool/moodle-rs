use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Lesson instance id.
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// The user id. 0 for current user.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The attempt number.
    #[serde(rename = "lessonattempt")]
    pub r#lessonattempt: Option<i64>,
}

/// Page fields
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnswerpagesItemPage {
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

pub type r#ReturnsAnswerpagesItemAnswerdataAnswersItem = Vec<String>;

/// User answers
pub type r#ReturnsAnswerpagesItemAnswerdataAnswers =
    Vec<r#ReturnsAnswerpagesItemAnswerdataAnswersItem>;

/// Answer data (empty in content pages created in Moodle 1.x).
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnswerpagesItemAnswerdata {
    /// The score (text version).
    #[serde(rename = "score")]
    pub r#score: Option<String>,
    /// The response text.
    #[serde(rename = "response")]
    pub r#response: Option<String>,
    /// response. format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "responseformat")]
    pub r#responseformat: Option<i64>,
    /// User answers
    #[serde(rename = "answers")]
    pub r#answers: Option<r#ReturnsAnswerpagesItemAnswerdataAnswers>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnswerpagesItem {
    /// Page fields
    #[serde(rename = "page")]
    pub r#page: Option<ReturnsAnswerpagesItemPage>,
    /// Page title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Page contents.
    #[serde(rename = "contents")]
    pub r#contents: Option<String>,
    /// Identifies the page type of this page.
    #[serde(rename = "qtype")]
    pub r#qtype: Option<String>,
    /// If is required to apply a grayout.
    #[serde(rename = "grayout")]
    pub r#grayout: Option<i64>,
    /// Answer data (empty in content pages created in Moodle 1.x).
    #[serde(rename = "answerdata")]
    pub r#answerdata: Option<ReturnsAnswerpagesItemAnswerdata>,
}

pub type r#ReturnsAnswerpages = Vec<ReturnsAnswerpagesItem>;

/// Attempt grade
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserstatsGradeinfo {
    /// Number of questions answered
    #[serde(rename = "nquestions")]
    pub r#nquestions: Option<i64>,
    /// Number of question attempts
    #[serde(rename = "attempts")]
    pub r#attempts: Option<i64>,
    /// Max points possible
    #[serde(rename = "total")]
    pub r#total: Option<f64>,
    /// Points earned by student
    #[serde(rename = "earned")]
    pub r#earned: Option<f64>,
    /// Calculated percentage grade
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// Number of manually graded questions
    #[serde(rename = "nmanual")]
    pub r#nmanual: Option<i64>,
    /// Point value for manually graded questions
    #[serde(rename = "manualpoints")]
    pub r#manualpoints: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserstats {
    /// Attempt final grade.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// Time completed.
    #[serde(rename = "completed")]
    pub r#completed: Option<i64>,
    /// Time taken.
    #[serde(rename = "timetotake")]
    pub r#timetotake: Option<i64>,
    /// Attempt grade
    #[serde(rename = "gradeinfo")]
    pub r#gradeinfo: Option<ReturnsUserstatsGradeinfo>,
}

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
    #[serde(rename = "answerpages")]
    pub r#answerpages: Option<r#ReturnsAnswerpages>,
    #[serde(rename = "userstats")]
    pub r#userstats: Option<ReturnsUserstats>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lesson_get_user_attempt", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lesson_get_user_attempt", params).await
}
