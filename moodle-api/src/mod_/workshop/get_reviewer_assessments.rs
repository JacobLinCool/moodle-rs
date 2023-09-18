use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Workshop instance id.
    #[serde(rename = "workshopid")]
    pub r#workshopid: Option<i64>,
    /// User id who did the assessment review (empty or 0 for current user).
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssessmentsItemFeedbackcontentfilesItem {
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

/// feedbackcontentfiles
pub type r#ReturnsAssessmentsItemFeedbackcontentfiles =
    Vec<ReturnsAssessmentsItemFeedbackcontentfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssessmentsItemFeedbackattachmentfilesItem {
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

/// feedbackattachmentfiles
pub type r#ReturnsAssessmentsItemFeedbackattachmentfiles =
    Vec<ReturnsAssessmentsItemFeedbackattachmentfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssessmentsItem {
    /// The primary key of the record.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The id of the assessed submission
    #[serde(rename = "submissionid")]
    pub r#submissionid: Option<i64>,
    /// The id of the reviewer who makes this assessment
    #[serde(rename = "reviewerid")]
    pub r#reviewerid: Option<i64>,
    /// The weight of the assessment for the purposes of aggregation
    #[serde(rename = "weight")]
    pub r#weight: Option<i64>,
    /// If 0 then the assessment was allocated but the reviewer has not assessed yet. If greater than 0 then the timestamp of when the reviewer assessed for the first time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// If 0 then the assessment was allocated but the reviewer has not assessed yet. If greater than 0 then the timestamp of when the reviewer assessed for the last time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// The aggregated grade for submission suggested by the reviewer. The grade 0..100 is computed from the values assigned to the assessment dimensions fields. If NULL then it has not been aggregated yet.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// The computed grade 0..100 for this assessment. If NULL then it has not been computed yet.
    #[serde(rename = "gradinggrade")]
    pub r#gradinggrade: Option<f64>,
    /// Grade for the assessment manually overridden by a teacher. Grade is always from interval 0..100. If NULL then the grade is not overriden.
    #[serde(rename = "gradinggradeover")]
    pub r#gradinggradeover: Option<f64>,
    /// The id of the user who has overridden the grade for submission.
    #[serde(rename = "gradinggradeoverby")]
    pub r#gradinggradeoverby: Option<i64>,
    /// The comment/feedback from the reviewer for the author.
    #[serde(rename = "feedbackauthor")]
    pub r#feedbackauthor: Option<String>,
    /// feedbackauthor format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "feedbackauthorformat")]
    pub r#feedbackauthorformat: Option<i64>,
    /// Are there some files attached to the feedbackauthor field? Sets to 1 by file_postupdate_standard_filemanager().
    #[serde(rename = "feedbackauthorattachment")]
    pub r#feedbackauthorattachment: Option<i64>,
    /// The comment/feedback from the teacher for the reviewer. For example the reason why the grade for assessment was overridden
    #[serde(rename = "feedbackreviewer")]
    pub r#feedbackreviewer: Option<String>,
    /// feedbackreviewer format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "feedbackreviewerformat")]
    pub r#feedbackreviewerformat: Option<i64>,
    /// feedbackcontentfiles
    #[serde(rename = "feedbackcontentfiles")]
    pub r#feedbackcontentfiles: Option<r#ReturnsAssessmentsItemFeedbackcontentfiles>,
    /// feedbackattachmentfiles
    #[serde(rename = "feedbackattachmentfiles")]
    pub r#feedbackattachmentfiles: Option<r#ReturnsAssessmentsItemFeedbackattachmentfiles>,
}

pub type r#ReturnsAssessments = Vec<ReturnsAssessmentsItem>;

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
    #[serde(rename = "assessments")]
    pub r#assessments: Option<r#ReturnsAssessments>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_workshop_get_reviewer_assessments", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_workshop_get_reviewer_assessments", params)
        .await
}
