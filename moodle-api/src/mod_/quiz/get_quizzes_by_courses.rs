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
pub struct ReturnsQuizzesItemIntrofilesItem {
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
pub type r#ReturnsQuizzesItemIntrofiles = Vec<ReturnsQuizzesItemIntrofilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsQuizzesItem {
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
    pub r#introfiles: Option<r#ReturnsQuizzesItemIntrofiles>,
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
    /// The time when this quiz opens. (0 = no restriction.)
    #[serde(rename = "timeopen")]
    pub r#timeopen: Option<i64>,
    /// The time when this quiz closes. (0 = no restriction.)
    #[serde(rename = "timeclose")]
    pub r#timeclose: Option<i64>,
    /// The time limit for quiz attempts, in seconds.
    #[serde(rename = "timelimit")]
    pub r#timelimit: Option<i64>,
    /// The method used to handle overdue attempts. 'autosubmit', 'graceperiod' or 'autoabandon'.
    #[serde(rename = "overduehandling")]
    pub r#overduehandling: Option<String>,
    /// The amount of time (in seconds) after the time limit runs out during which attempts can still be submitted, if overduehandling is set to allow it.
    #[serde(rename = "graceperiod")]
    pub r#graceperiod: Option<i64>,
    /// The behaviour to ask questions to use.
    #[serde(rename = "preferredbehaviour")]
    pub r#preferredbehaviour: Option<String>,
    /// Allows students to redo any completed question within a quiz attempt.
    #[serde(rename = "canredoquestions")]
    pub r#canredoquestions: Option<i64>,
    /// The maximum number of attempts a student is allowed.
    #[serde(rename = "attempts")]
    pub r#attempts: Option<i64>,
    /// Whether subsequent attempts start from the answer to the previous attempt (1) or start blank (0).
    #[serde(rename = "attemptonlast")]
    pub r#attemptonlast: Option<i64>,
    /// One of the values QUIZ_GRADEHIGHEST, QUIZ_GRADEAVERAGE, QUIZ_ATTEMPTFIRST or QUIZ_ATTEMPTLAST.
    #[serde(rename = "grademethod")]
    pub r#grademethod: Option<i64>,
    /// Number of decimal points to use when displaying grades.
    #[serde(rename = "decimalpoints")]
    pub r#decimalpoints: Option<i64>,
    /// Number of decimal points to use when displaying question grades. (-1 means use decimalpoints.)
    #[serde(rename = "questiondecimalpoints")]
    pub r#questiondecimalpoints: Option<i64>,
    /// Whether users are allowed to review their quiz attempts at various times. This is a bit field, decoded by the \mod_quiz\question\display_options class. It is formed by ORing together the constants defined there.
    #[serde(rename = "reviewattempt")]
    pub r#reviewattempt: Option<i64>,
    /// Whether users are allowed to review their quiz attempts at various times. A bit field, like reviewattempt.
    #[serde(rename = "reviewcorrectness")]
    pub r#reviewcorrectness: Option<i64>,
    /// Whether users are allowed to review their quiz attempts at various times. A bit field, like reviewattempt.
    #[serde(rename = "reviewmarks")]
    pub r#reviewmarks: Option<i64>,
    /// Whether users are allowed to review their quiz attempts at various times. A bit field, like reviewattempt.
    #[serde(rename = "reviewspecificfeedback")]
    pub r#reviewspecificfeedback: Option<i64>,
    /// Whether users are allowed to review their quiz attempts at various times. A bit field, like reviewattempt.
    #[serde(rename = "reviewgeneralfeedback")]
    pub r#reviewgeneralfeedback: Option<i64>,
    /// Whether users are allowed to review their quiz attempts at various times. A bit field, like reviewattempt.
    #[serde(rename = "reviewrightanswer")]
    pub r#reviewrightanswer: Option<i64>,
    /// Whether users are allowed to review their quiz attempts at various times. A bit field, like reviewattempt.
    #[serde(rename = "reviewoverallfeedback")]
    pub r#reviewoverallfeedback: Option<i64>,
    /// How often to insert a page break when editing the quiz, or when shuffling the question order.
    #[serde(rename = "questionsperpage")]
    pub r#questionsperpage: Option<i64>,
    /// Any constraints on how the user is allowed to navigate around the quiz. Currently recognised values are 'free' and 'seq'.
    #[serde(rename = "navmethod")]
    pub r#navmethod: Option<String>,
    /// Whether the parts of the question should be shuffled, in those question types that support it.
    #[serde(rename = "shuffleanswers")]
    pub r#shuffleanswers: Option<i64>,
    /// The total of all the question instance maxmarks.
    #[serde(rename = "sumgrades")]
    pub r#sumgrades: Option<f64>,
    /// The total that the quiz overall grade is scaled to be out of.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// The time when the quiz was added to the course.
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Last modified time.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// A password that the student must enter before starting or continuing a quiz attempt.
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// Used to restrict the IP addresses from which this quiz can be attempted. The format is as requried by the address_in_subnet function.
    #[serde(rename = "subnet")]
    pub r#subnet: Option<String>,
    /// Restriciton on the browser the student must use. E.g. 'securewindow'.
    #[serde(rename = "browsersecurity")]
    pub r#browsersecurity: Option<String>,
    /// Delay that must be left between the first and second attempt, in seconds.
    #[serde(rename = "delay1")]
    pub r#delay1: Option<i64>,
    /// Delay that must be left between the second and subsequent attempt, in seconds.
    #[serde(rename = "delay2")]
    pub r#delay2: Option<i64>,
    /// Option to show the user's picture during the attempt and on the review page.
    #[serde(rename = "showuserpicture")]
    pub r#showuserpicture: Option<i64>,
    /// Whether blocks should be shown on the attempt.php and review.php pages.
    #[serde(rename = "showblocks")]
    pub r#showblocks: Option<i64>,
    /// Mark quiz complete when the student has exhausted the maximum number of attempts
    #[serde(rename = "completionattemptsexhausted")]
    pub r#completionattemptsexhausted: Option<i64>,
    /// Whether to require passing grade
    #[serde(rename = "completionpass")]
    pub r#completionpass: Option<i64>,
    /// Whether to allow the quiz to be attempted offline in the mobile app
    #[serde(rename = "allowofflineattempts")]
    pub r#allowofflineattempts: Option<i64>,
    /// Auto-save delay
    #[serde(rename = "autosaveperiod")]
    pub r#autosaveperiod: Option<i64>,
    /// Whether the quiz has any non-blank feedback text
    #[serde(rename = "hasfeedback")]
    pub r#hasfeedback: Option<i64>,
    /// Whether the quiz has questions
    #[serde(rename = "hasquestions")]
    pub r#hasquestions: Option<i64>,
}

pub type r#ReturnsQuizzes = Vec<ReturnsQuizzesItem>;

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
    #[serde(rename = "quizzes")]
    pub r#quizzes: Option<r#ReturnsQuizzes>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_quiz_get_quizzes_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_quiz_get_quizzes_by_courses", params).await
}
