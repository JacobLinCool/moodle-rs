use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Workshop instance id.
    #[serde(rename = "workshopid")]
    pub r#workshopid: Option<i64>,
    /// Get submissions done by this user. Use 0 or empty for the current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Group id, 0 means that the function will determine the user group. It will return submissions done by users in the given group.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// The page of records to return.
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The number of records to return per page.
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSubmissionsItemContentfilesItem {
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

/// contentfiles
pub type r#ReturnsSubmissionsItemContentfiles = Vec<ReturnsSubmissionsItemContentfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSubmissionsItemAttachmentfilesItem {
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

/// attachmentfiles
pub type r#ReturnsSubmissionsItemAttachmentfiles = Vec<ReturnsSubmissionsItemAttachmentfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSubmissionsItem {
    /// The primary key of the record.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The id of the workshop instance.
    #[serde(rename = "workshopid")]
    pub r#workshopid: Option<i64>,
    /// Is this submission an example from teacher.
    #[serde(rename = "example")]
    pub r#example: Option<bool>,
    /// The author of the submission.
    #[serde(rename = "authorid")]
    pub r#authorid: Option<i64>,
    /// Timestamp when the work was submitted for the first time.
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Timestamp when the submission has been updated.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// The submission title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Submission text.
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// content format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "contentformat")]
    pub r#contentformat: Option<i64>,
    /// The trust mode of the data.
    #[serde(rename = "contenttrust")]
    pub r#contenttrust: Option<i64>,
    /// Used by File API file_postupdate_standard_filemanager.
    #[serde(rename = "attachment")]
    pub r#attachment: Option<i64>,
    /// Aggregated grade for the submission. The grade is a decimal number from interval 0..100. If NULL then the grade for submission has not been aggregated yet.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// Grade for the submission manually overridden by a teacher. Grade is always from interval 0..100. If NULL then the grade is not overriden.
    #[serde(rename = "gradeover")]
    pub r#gradeover: Option<f64>,
    /// The id of the user who has overridden the grade for submission.
    #[serde(rename = "gradeoverby")]
    pub r#gradeoverby: Option<i64>,
    /// Teacher comment/feedback for the author of the submission, for example describing the reasons for the grade overriding.
    #[serde(rename = "feedbackauthor")]
    pub r#feedbackauthor: Option<String>,
    /// feedbackauthor format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "feedbackauthorformat")]
    pub r#feedbackauthorformat: Option<i64>,
    /// The timestamp when grade or gradeover was recently modified.
    #[serde(rename = "timegraded")]
    pub r#timegraded: Option<i64>,
    /// Shall the submission be available to other when the workshop is closed.
    #[serde(rename = "published")]
    pub r#published: Option<bool>,
    /// Has this submission been submitted after the deadline or during the assessment phase?
    #[serde(rename = "late")]
    pub r#late: Option<i64>,
    /// contentfiles
    #[serde(rename = "contentfiles")]
    pub r#contentfiles: Option<r#ReturnsSubmissionsItemContentfiles>,
    /// attachmentfiles
    #[serde(rename = "attachmentfiles")]
    pub r#attachmentfiles: Option<r#ReturnsSubmissionsItemAttachmentfiles>,
}

pub type r#ReturnsSubmissions = Vec<ReturnsSubmissionsItem>;

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
    #[serde(rename = "submissions")]
    pub r#submissions: Option<r#ReturnsSubmissions>,
    /// Total count of submissions.
    #[serde(rename = "totalcount")]
    pub r#totalcount: Option<i64>,
    /// Total size (bytes) of the files attached to all the submissions (even the ones not returned due to pagination).
    #[serde(rename = "totalfilesize")]
    pub r#totalfilesize: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_workshop_get_submissions", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_workshop_get_submissions", params).await
}
