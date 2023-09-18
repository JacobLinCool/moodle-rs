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
pub struct ReturnsFeedbacksItemIntrofilesItem {
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
pub type r#ReturnsFeedbacksItemIntrofiles = Vec<ReturnsFeedbacksItemIntrofilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedbacksItemPageaftersubmitfilesItem {
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

/// pageaftersubmitfiles
pub type r#ReturnsFeedbacksItemPageaftersubmitfiles =
    Vec<ReturnsFeedbacksItemPageaftersubmitfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedbacksItem {
    /// The primary key of the record.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course id this feedback is part of.
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// Feedback name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Feedback introduction text.
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// Forced activity language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// Whether the feedback is anonymous.
    #[serde(rename = "anonymous")]
    pub r#anonymous: Option<i64>,
    /// Whether email notifications will be sent to teachers.
    #[serde(rename = "email_notification")]
    pub r#email_notification: Option<bool>,
    /// Whether multiple submissions are allowed.
    #[serde(rename = "multiple_submit")]
    pub r#multiple_submit: Option<bool>,
    /// Whether questions should be auto-numbered.
    #[serde(rename = "autonumbering")]
    pub r#autonumbering: Option<bool>,
    /// Link to next page after submission.
    #[serde(rename = "site_after_submit")]
    pub r#site_after_submit: Option<String>,
    /// Text to display after submission.
    #[serde(rename = "page_after_submit")]
    pub r#page_after_submit: Option<String>,
    /// page_after_submit format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "page_after_submitformat")]
    pub r#page_after_submitformat: Option<i64>,
    /// Whether stats should be published.
    #[serde(rename = "publish_stats")]
    pub r#publish_stats: Option<bool>,
    /// Allow answers from this time.
    #[serde(rename = "timeopen")]
    pub r#timeopen: Option<i64>,
    /// Allow answers until this time.
    #[serde(rename = "timeclose")]
    pub r#timeclose: Option<i64>,
    /// The time this record was modified.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// If this field is set to 1, then the activity will be automatically marked as complete on submission.
    #[serde(rename = "completionsubmit")]
    pub r#completionsubmit: Option<bool>,
    /// coursemodule
    #[serde(rename = "coursemodule")]
    pub r#coursemodule: Option<i64>,
    /// introfiles
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsFeedbacksItemIntrofiles>,
    /// pageaftersubmitfiles
    #[serde(rename = "pageaftersubmitfiles")]
    pub r#pageaftersubmitfiles: Option<r#ReturnsFeedbacksItemPageaftersubmitfiles>,
}

pub type r#ReturnsFeedbacks = Vec<ReturnsFeedbacksItem>;

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
    #[serde(rename = "feedbacks")]
    pub r#feedbacks: Option<r#ReturnsFeedbacks>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_feedback_get_feedbacks_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_feedback_get_feedbacks_by_courses", params)
        .await
}
