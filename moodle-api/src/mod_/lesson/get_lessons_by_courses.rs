use serde::{self, Deserialize, Serialize};

/// Array of course ids
pub type r#ParamsCourseids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of course ids
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsCourseids>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLessonsItemIntrofilesItem {
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
    /// The repository type for the external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// introfiles
pub type r#ReturnsLessonsItemIntrofiles = Vec<ReturnsLessonsItemIntrofilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLessonsItemMediafilesItem {
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
    /// The repository type for the external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// mediafiles
pub type r#ReturnsLessonsItemMediafiles = Vec<ReturnsLessonsItemMediafilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLessonsItem {
    /// Standard Moodle primary key.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Foreign key reference to the course this lesson is part of.
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// Course module id.
    #[serde(rename = "coursemodule")]
    pub r#coursemodule: Option<i64>,
    /// Lesson name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Lesson introduction text.
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// Forced activity language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// Practice lesson?
    #[serde(rename = "practice")]
    pub r#practice: Option<bool>,
    /// Allow student review?
    #[serde(rename = "modattempts")]
    pub r#modattempts: Option<bool>,
    /// Password protected lesson?
    #[serde(rename = "usepassword")]
    pub r#usepassword: Option<bool>,
    /// Password
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// Dependent on (another lesson id)
    #[serde(rename = "dependency")]
    pub r#dependency: Option<i64>,
    /// Conditions to enable the lesson
    #[serde(rename = "conditions")]
    pub r#conditions: Option<String>,
    /// The total that the grade is scaled to be out of
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
    /// Custom scoring?
    #[serde(rename = "custom")]
    pub r#custom: Option<bool>,
    /// Display ongoing score?
    #[serde(rename = "ongoing")]
    pub r#ongoing: Option<bool>,
    /// How to calculate the final grade
    #[serde(rename = "usemaxgrade")]
    pub r#usemaxgrade: Option<i64>,
    /// Maximum answers per page
    #[serde(rename = "maxanswers")]
    pub r#maxanswers: Option<i64>,
    /// Maximum attempts
    #[serde(rename = "maxattempts")]
    pub r#maxattempts: Option<i64>,
    /// Provide option to try a question again
    #[serde(rename = "review")]
    pub r#review: Option<bool>,
    /// Action for a correct answer
    #[serde(rename = "nextpagedefault")]
    pub r#nextpagedefault: Option<i64>,
    /// Display default feedback
    #[serde(rename = "feedback")]
    pub r#feedback: Option<bool>,
    /// Minimum number of questions
    #[serde(rename = "minquestions")]
    pub r#minquestions: Option<i64>,
    /// Number of pages to show
    #[serde(rename = "maxpages")]
    pub r#maxpages: Option<i64>,
    /// Time limit
    #[serde(rename = "timelimit")]
    pub r#timelimit: Option<i64>,
    /// Re-takes allowed
    #[serde(rename = "retake")]
    pub r#retake: Option<bool>,
    /// Id of the next activity to be linked once the lesson is completed
    #[serde(rename = "activitylink")]
    pub r#activitylink: Option<i64>,
    /// Local file path or full external URL
    #[serde(rename = "mediafile")]
    pub r#mediafile: Option<String>,
    /// Popup for media file height
    #[serde(rename = "mediaheight")]
    pub r#mediaheight: Option<i64>,
    /// Popup for media with
    #[serde(rename = "mediawidth")]
    pub r#mediawidth: Option<i64>,
    /// Display a close button in the popup?
    #[serde(rename = "mediaclose")]
    pub r#mediaclose: Option<i64>,
    /// Display lesson as slideshow
    #[serde(rename = "slideshow")]
    pub r#slideshow: Option<bool>,
    /// Slideshow width
    #[serde(rename = "width")]
    pub r#width: Option<i64>,
    /// Slideshow height
    #[serde(rename = "height")]
    pub r#height: Option<i64>,
    /// Slideshow bgcolor
    #[serde(rename = "bgcolor")]
    pub r#bgcolor: Option<String>,
    /// Display left pages menu?
    #[serde(rename = "displayleft")]
    pub r#displayleft: Option<bool>,
    /// Minimum grade to display menu
    #[serde(rename = "displayleftif")]
    pub r#displayleftif: Option<i64>,
    /// Display progress bar?
    #[serde(rename = "progressbar")]
    pub r#progressbar: Option<bool>,
    /// Available from
    #[serde(rename = "available")]
    pub r#available: Option<i64>,
    /// Available until
    #[serde(rename = "deadline")]
    pub r#deadline: Option<i64>,
    /// Last time settings were updated
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Require end reached for completion?
    #[serde(rename = "completionendreached")]
    pub r#completionendreached: Option<i64>,
    /// Student must do this activity at least for
    #[serde(rename = "completiontimespent")]
    pub r#completiontimespent: Option<i64>,
    /// Whether to allow the lesson to be attempted offline in the mobile app
    #[serde(rename = "allowofflineattempts")]
    pub r#allowofflineattempts: Option<bool>,
    /// introfiles
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsLessonsItemIntrofiles>,
    /// mediafiles
    #[serde(rename = "mediafiles")]
    pub r#mediafiles: Option<r#ReturnsLessonsItemMediafiles>,
}

pub type r#ReturnsLessons = Vec<ReturnsLessonsItem>;

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
    #[serde(rename = "lessons")]
    pub r#lessons: Option<r#ReturnsLessons>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lesson_get_lessons_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lesson_get_lessons_by_courses", params)
        .await
}
