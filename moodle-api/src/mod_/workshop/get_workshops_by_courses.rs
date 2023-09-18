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
pub struct ReturnsWorkshopsItemIntrofilesItem {
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
pub type r#ReturnsWorkshopsItemIntrofiles = Vec<ReturnsWorkshopsItemIntrofilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWorkshopsItemInstructauthorsfilesItem {
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

/// instructauthorsfiles
pub type r#ReturnsWorkshopsItemInstructauthorsfiles =
    Vec<ReturnsWorkshopsItemInstructauthorsfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWorkshopsItemInstructreviewersfilesItem {
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

/// instructreviewersfiles
pub type r#ReturnsWorkshopsItemInstructreviewersfiles =
    Vec<ReturnsWorkshopsItemInstructreviewersfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWorkshopsItemConclusionfilesItem {
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

/// conclusionfiles
pub type r#ReturnsWorkshopsItemConclusionfiles = Vec<ReturnsWorkshopsItemConclusionfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWorkshopsItem {
    /// The primary key of the record.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course id this workshop is part of.
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// Workshop name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Workshop introduction text.
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// Forced activity language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// Instructions for the submission phase.
    #[serde(rename = "instructauthors")]
    pub r#instructauthors: Option<String>,
    /// instructauthors format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "instructauthorsformat")]
    pub r#instructauthorsformat: Option<i64>,
    /// Instructions for the assessment phase.
    #[serde(rename = "instructreviewers")]
    pub r#instructreviewers: Option<String>,
    /// instructreviewers format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "instructreviewersformat")]
    pub r#instructreviewersformat: Option<i64>,
    /// The timestamp when the module was modified.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// The current phase of workshop (0 = not available, 1 = submission, 2 = assessment, 3 = closed).
    #[serde(rename = "phase")]
    pub r#phase: Option<i64>,
    /// Optional feature: students practise evaluating on example submissions from teacher.
    #[serde(rename = "useexamples")]
    pub r#useexamples: Option<bool>,
    /// Optional feature: students perform peer assessment of others' work.
    #[serde(rename = "usepeerassessment")]
    pub r#usepeerassessment: Option<bool>,
    /// Optional feature: students perform self assessment of their own work.
    #[serde(rename = "useselfassessment")]
    pub r#useselfassessment: Option<bool>,
    /// The maximum grade for submission.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// The maximum grade for assessment.
    #[serde(rename = "gradinggrade")]
    pub r#gradinggrade: Option<f64>,
    /// The type of the current grading strategy used in this workshop.
    #[serde(rename = "strategy")]
    pub r#strategy: Option<String>,
    /// The recently used grading evaluation method.
    #[serde(rename = "evaluation")]
    pub r#evaluation: Option<String>,
    /// Number of digits that should be shown after the decimal point when displaying grades.
    #[serde(rename = "gradedecimals")]
    pub r#gradedecimals: Option<i64>,
    /// Indicates whether text is required as part of each submission. 0 for no, 1 for optional, 2 for required.
    #[serde(rename = "submissiontypetext")]
    pub r#submissiontypetext: Option<i64>,
    /// Indicates whether a file upload is required as part of each submission. 0 for no, 1 for optional, 2 for required.
    #[serde(rename = "submissiontypefile")]
    pub r#submissiontypefile: Option<i64>,
    /// Maximum number of submission attachments.
    #[serde(rename = "nattachments")]
    pub r#nattachments: Option<i64>,
    /// Comma separated list of file extensions.
    #[serde(rename = "submissionfiletypes")]
    pub r#submissionfiletypes: Option<String>,
    /// Allow submitting the work after the deadline.
    #[serde(rename = "latesubmissions")]
    pub r#latesubmissions: Option<bool>,
    /// Maximum size of the one attached file.
    #[serde(rename = "maxbytes")]
    pub r#maxbytes: Option<i64>,
    /// 0 = example assessments are voluntary, 1 = examples must be assessed before submission, 2 = examples are available after own submission and must be assessed before peer/self assessment phase.
    #[serde(rename = "examplesmode")]
    pub r#examplesmode: Option<i64>,
    /// 0 = will be started manually, greater than 0 the timestamp of the start of the submission phase.
    #[serde(rename = "submissionstart")]
    pub r#submissionstart: Option<i64>,
    /// 0 = will be closed manually, greater than 0 the timestamp of the end of the submission phase.
    #[serde(rename = "submissionend")]
    pub r#submissionend: Option<i64>,
    /// 0 = will be started manually, greater than 0 the timestamp of the start of the assessment phase.
    #[serde(rename = "assessmentstart")]
    pub r#assessmentstart: Option<i64>,
    /// 0 = will be closed manually, greater than 0 the timestamp of the end of the assessment phase.
    #[serde(rename = "assessmentend")]
    pub r#assessmentend: Option<i64>,
    /// Automatically switch to the assessment phase after the submissions deadline.
    #[serde(rename = "phaseswitchassessment")]
    pub r#phaseswitchassessment: Option<bool>,
    /// A text to be displayed at the end of the workshop.
    #[serde(rename = "conclusion")]
    pub r#conclusion: Option<String>,
    /// conclusion format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "conclusionformat")]
    pub r#conclusionformat: Option<i64>,
    /// Mode of the overall feedback support.
    #[serde(rename = "overallfeedbackmode")]
    pub r#overallfeedbackmode: Option<i64>,
    /// Number of allowed attachments to the overall feedback.
    #[serde(rename = "overallfeedbackfiles")]
    pub r#overallfeedbackfiles: Option<i64>,
    /// Comma separated list of file extensions.
    #[serde(rename = "overallfeedbackfiletypes")]
    pub r#overallfeedbackfiletypes: Option<String>,
    /// Maximum size of one file attached to the overall feedback.
    #[serde(rename = "overallfeedbackmaxbytes")]
    pub r#overallfeedbackmaxbytes: Option<i64>,
    /// coursemodule
    #[serde(rename = "coursemodule")]
    pub r#coursemodule: Option<i64>,
    /// introfiles
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsWorkshopsItemIntrofiles>,
    /// instructauthorsfiles
    #[serde(rename = "instructauthorsfiles")]
    pub r#instructauthorsfiles: Option<r#ReturnsWorkshopsItemInstructauthorsfiles>,
    /// instructreviewersfiles
    #[serde(rename = "instructreviewersfiles")]
    pub r#instructreviewersfiles: Option<r#ReturnsWorkshopsItemInstructreviewersfiles>,
    /// conclusionfiles
    #[serde(rename = "conclusionfiles")]
    pub r#conclusionfiles: Option<r#ReturnsWorkshopsItemConclusionfiles>,
}

pub type r#ReturnsWorkshops = Vec<ReturnsWorkshopsItem>;

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
    #[serde(rename = "workshops")]
    pub r#workshops: Option<r#ReturnsWorkshops>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_workshop_get_workshops_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_workshop_get_workshops_by_courses", params)
        .await
}
