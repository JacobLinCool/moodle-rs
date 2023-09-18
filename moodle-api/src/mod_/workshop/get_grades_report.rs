use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Workshop instance id.
    #[serde(rename = "workshopid")]
    pub r#workshopid: Option<i64>,
    /// Group id, 0 means that the function will determine the user group.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// sort by this element: lastname, firstname, submissiontitle, submissionmodified, submissiongrade, gradinggrade.
    #[serde(rename = "sortby")]
    pub r#sortby: Option<String>,
    /// sort direction: ASC or DESC
    #[serde(rename = "sortdirection")]
    pub r#sortdirection: Option<String>,
    /// The page of records to return.
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The number of records to return per page.
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsReportGradesItemReviewedbyItem {
    /// The id of the user (0 when is configured to do not display names).
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The id of the assessment.
    #[serde(rename = "assessmentid")]
    pub r#assessmentid: Option<i64>,
    /// The id of the submission assessed.
    #[serde(rename = "submissionid")]
    pub r#submissionid: Option<i64>,
    /// The grade for submission.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// The grade for assessment.
    #[serde(rename = "gradinggrade")]
    pub r#gradinggrade: Option<f64>,
    /// The aggregated grade overrided.
    #[serde(rename = "gradinggradeover")]
    pub r#gradinggradeover: Option<f64>,
    /// The weight of the assessment for aggregation.
    #[serde(rename = "weight")]
    pub r#weight: Option<i64>,
}

/// The users who reviewed the

/// user submission.
pub type r#ReturnsReportGradesItemReviewedby = Vec<ReturnsReportGradesItemReviewedbyItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsReportGradesItemReviewerofItem {
    /// The id of the user (0 when is configured to do not display names).
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The id of the assessment.
    #[serde(rename = "assessmentid")]
    pub r#assessmentid: Option<i64>,
    /// The id of the submission assessed.
    #[serde(rename = "submissionid")]
    pub r#submissionid: Option<i64>,
    /// The grade for submission.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// The grade for assessment.
    #[serde(rename = "gradinggrade")]
    pub r#gradinggrade: Option<f64>,
    /// The aggregated grade overrided.
    #[serde(rename = "gradinggradeover")]
    pub r#gradinggradeover: Option<f64>,
    /// The weight of the assessment for aggregation.
    #[serde(rename = "weight")]
    pub r#weight: Option<i64>,
}

/// The assessments the user

/// reviewed.
pub type r#ReturnsReportGradesItemReviewerof = Vec<ReturnsReportGradesItemReviewerofItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsReportGradesItem {
    /// The id of the user being displayed in the report.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Submission id.
    #[serde(rename = "submissionid")]
    pub r#submissionid: Option<i64>,
    /// Submission title.
    #[serde(rename = "submissiontitle")]
    pub r#submissiontitle: Option<String>,
    /// Timestamp submission was updated.
    #[serde(rename = "submissionmodified")]
    pub r#submissionmodified: Option<i64>,
    /// Aggregated grade for the submission.
    #[serde(rename = "submissiongrade")]
    pub r#submissiongrade: Option<f64>,
    /// Computed grade for the assessment.
    #[serde(rename = "gradinggrade")]
    pub r#gradinggrade: Option<f64>,
    /// Grade for the assessment overrided by the teacher.
    #[serde(rename = "submissiongradeover")]
    pub r#submissiongradeover: Option<f64>,
    /// The id of the user who overrided the grade.
    #[serde(rename = "submissiongradeoverby")]
    pub r#submissiongradeoverby: Option<i64>,
    /// Whether is a submission published.
    #[serde(rename = "submissionpublished")]
    pub r#submissionpublished: Option<i64>,
    /// The users who reviewed the user submission.
    #[serde(rename = "reviewedby")]
    pub r#reviewedby: Option<r#ReturnsReportGradesItemReviewedby>,
    /// The assessments the user reviewed.
    #[serde(rename = "reviewerof")]
    pub r#reviewerof: Option<r#ReturnsReportGradesItemReviewerof>,
}

pub type r#ReturnsReportGrades = Vec<ReturnsReportGradesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsReport {
    #[serde(rename = "grades")]
    pub r#grades: Option<r#ReturnsReportGrades>,
    /// Number of total submissions.
    #[serde(rename = "totalcount")]
    pub r#totalcount: Option<i64>,
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
    #[serde(rename = "report")]
    pub r#report: Option<ReturnsReport>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_workshop_get_grades_report", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_workshop_get_grades_report", params).await
}
