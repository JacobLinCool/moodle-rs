use serde::{self, Deserialize, Serialize};

/// Array of Course IDs
pub type r#ParamsCourseids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of Course IDs
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsCourseids>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemIntrofilesItem {
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

/// Files in the introduction text
pub type r#ReturnsItemIntrofiles = Vec<ReturnsItemIntrofilesItem>;

/// forum
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Forum id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course id
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// The forum type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Forum name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The forum intro
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// Files in the introduction text
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsItemIntrofiles>,
    /// Forced activity language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// duedate for the user
    #[serde(rename = "duedate")]
    pub r#duedate: Option<i64>,
    /// cutoffdate for the user
    #[serde(rename = "cutoffdate")]
    pub r#cutoffdate: Option<i64>,
    /// Aggregate type
    #[serde(rename = "assessed")]
    pub r#assessed: Option<i64>,
    /// Assess start time
    #[serde(rename = "assesstimestart")]
    pub r#assesstimestart: Option<i64>,
    /// Assess finish time
    #[serde(rename = "assesstimefinish")]
    pub r#assesstimefinish: Option<i64>,
    /// Scale
    #[serde(rename = "scale")]
    pub r#scale: Option<i64>,
    /// Whole forum grade
    #[serde(rename = "grade_forum")]
    pub r#grade_forum: Option<i64>,
    /// Whether to send notifications to students upon grading by default
    #[serde(rename = "grade_forum_notify")]
    pub r#grade_forum_notify: Option<i64>,
    /// Maximum attachment size
    #[serde(rename = "maxbytes")]
    pub r#maxbytes: Option<i64>,
    /// Maximum number of attachments
    #[serde(rename = "maxattachments")]
    pub r#maxattachments: Option<i64>,
    /// Force users to subscribe
    #[serde(rename = "forcesubscribe")]
    pub r#forcesubscribe: Option<i64>,
    /// Subscription mode
    #[serde(rename = "trackingtype")]
    pub r#trackingtype: Option<i64>,
    /// RSS feed for this activity
    #[serde(rename = "rsstype")]
    pub r#rsstype: Option<i64>,
    /// Number of RSS recent articles
    #[serde(rename = "rssarticles")]
    pub r#rssarticles: Option<i64>,
    /// Time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Post threshold for warning
    #[serde(rename = "warnafter")]
    pub r#warnafter: Option<i64>,
    /// Post threshold for blocking
    #[serde(rename = "blockafter")]
    pub r#blockafter: Option<i64>,
    /// Time period for blocking
    #[serde(rename = "blockperiod")]
    pub r#blockperiod: Option<i64>,
    /// Student must create discussions
    #[serde(rename = "completiondiscussions")]
    pub r#completiondiscussions: Option<i64>,
    /// Student must post replies
    #[serde(rename = "completionreplies")]
    pub r#completionreplies: Option<i64>,
    /// Student must post discussions or replies
    #[serde(rename = "completionposts")]
    pub r#completionposts: Option<i64>,
    /// Course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// Number of discussions in the forum
    #[serde(rename = "numdiscussions")]
    pub r#numdiscussions: Option<i64>,
    /// If the user can create discussions
    #[serde(rename = "cancreatediscussions")]
    pub r#cancreatediscussions: Option<bool>,
    /// After what period a discussion is locked
    #[serde(rename = "lockdiscussionafter")]
    pub r#lockdiscussionafter: Option<i64>,
    /// If the user is tracking the forum
    #[serde(rename = "istracked")]
    pub r#istracked: Option<bool>,
    /// The number of unread posts for tracked forums
    #[serde(rename = "unreadpostscount")]
    pub r#unreadpostscount: Option<i64>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_forum_get_forums_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_forum_get_forums_by_courses", params).await
}
