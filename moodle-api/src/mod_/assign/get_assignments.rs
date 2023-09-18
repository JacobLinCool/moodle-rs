use serde::{self, Deserialize, Serialize};

/// 0 or more course ids
pub type r#ParamsCourseids = Vec<i64>;

/// list of capabilities used to filter courses
pub type r#ParamsCapabilities = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// 0 or more course ids
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsCourseids>,
    /// list of capabilities used to filter courses
    #[serde(rename = "capabilities")]
    pub r#capabilities: Option<r#ParamsCapabilities>,
    /// whether to return courses that the user can see even if is not enroled in. This requires the parameter courseids to not be empty.
    #[serde(rename = "includenotenrolledcourses")]
    pub r#includenotenrolledcourses: Option<bool>,
}

/// assignment configuration object
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemAssignmentsItemConfigsItem {
    /// assign_plugin_config id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// assignment id
    #[serde(rename = "assignment")]
    pub r#assignment: Option<i64>,
    /// plugin
    #[serde(rename = "plugin")]
    pub r#plugin: Option<String>,
    /// subtype
    #[serde(rename = "subtype")]
    pub r#subtype: Option<String>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// configuration settings
pub type r#ReturnsCoursesItemAssignmentsItemConfigs =
    Vec<ReturnsCoursesItemAssignmentsItemConfigsItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemAssignmentsItemIntrofilesItem {
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
pub type r#ReturnsCoursesItemAssignmentsItemIntrofiles =
    Vec<ReturnsCoursesItemAssignmentsItemIntrofilesItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemAssignmentsItemIntroattachmentsItem {
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

/// intro attachments files
pub type r#ReturnsCoursesItemAssignmentsItemIntroattachments =
    Vec<ReturnsCoursesItemAssignmentsItemIntroattachmentsItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemAssignmentsItemActivityattachmentsItem {
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

/// Files from activity field
pub type r#ReturnsCoursesItemAssignmentsItemActivityattachments =
    Vec<ReturnsCoursesItemAssignmentsItemActivityattachmentsItem>;

/// assignment information object
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemAssignmentsItem {
    /// assignment id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// course id
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// assignment name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// no submissions
    #[serde(rename = "nosubmissions")]
    pub r#nosubmissions: Option<i64>,
    /// submissions drafts
    #[serde(rename = "submissiondrafts")]
    pub r#submissiondrafts: Option<i64>,
    /// send notifications
    #[serde(rename = "sendnotifications")]
    pub r#sendnotifications: Option<i64>,
    /// send notifications
    #[serde(rename = "sendlatenotifications")]
    pub r#sendlatenotifications: Option<i64>,
    /// send student notifications (default)
    #[serde(rename = "sendstudentnotifications")]
    pub r#sendstudentnotifications: Option<i64>,
    /// assignment due date
    #[serde(rename = "duedate")]
    pub r#duedate: Option<i64>,
    /// allow submissions from date
    #[serde(rename = "allowsubmissionsfromdate")]
    pub r#allowsubmissionsfromdate: Option<i64>,
    /// grade type
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
    /// last time assignment was modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// if enabled, set activity as complete following submission
    #[serde(rename = "completionsubmit")]
    pub r#completionsubmit: Option<i64>,
    /// date after which submission is not accepted without an extension
    #[serde(rename = "cutoffdate")]
    pub r#cutoffdate: Option<i64>,
    /// the expected date for marking the submissions
    #[serde(rename = "gradingduedate")]
    pub r#gradingduedate: Option<i64>,
    /// if enabled, students submit as a team
    #[serde(rename = "teamsubmission")]
    pub r#teamsubmission: Option<i64>,
    /// if enabled, all team members must submit
    #[serde(rename = "requireallteammemberssubmit")]
    pub r#requireallteammemberssubmit: Option<i64>,
    /// the grouping id for the team submission groups
    #[serde(rename = "teamsubmissiongroupingid")]
    pub r#teamsubmissiongroupingid: Option<i64>,
    /// if enabled, hide identities until reveal identities actioned
    #[serde(rename = "blindmarking")]
    pub r#blindmarking: Option<i64>,
    /// If enabled, hide grader to student
    #[serde(rename = "hidegrader")]
    pub r#hidegrader: Option<i64>,
    /// show identities for a blind marking assignment
    #[serde(rename = "revealidentities")]
    pub r#revealidentities: Option<i64>,
    /// method used to control opening new attempts
    #[serde(rename = "attemptreopenmethod")]
    pub r#attemptreopenmethod: Option<String>,
    /// maximum number of attempts allowed
    #[serde(rename = "maxattempts")]
    pub r#maxattempts: Option<i64>,
    /// enable marking workflow
    #[serde(rename = "markingworkflow")]
    pub r#markingworkflow: Option<i64>,
    /// enable marking allocation
    #[serde(rename = "markingallocation")]
    pub r#markingallocation: Option<i64>,
    /// student must accept submission statement
    #[serde(rename = "requiresubmissionstatement")]
    pub r#requiresubmissionstatement: Option<i64>,
    /// Prevent submission not in group
    #[serde(rename = "preventsubmissionnotingroup")]
    pub r#preventsubmissionnotingroup: Option<i64>,
    /// Submission statement formatted.
    #[serde(rename = "submissionstatement")]
    pub r#submissionstatement: Option<String>,
    /// submissionstatement format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "submissionstatementformat")]
    pub r#submissionstatementformat: Option<i64>,
    /// configuration settings
    #[serde(rename = "configs")]
    pub r#configs: Option<r#ReturnsCoursesItemAssignmentsItemConfigs>,
    /// assignment intro, not allways returned because it deppends on the activity configuration
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// Files in the introduction text
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsCoursesItemAssignmentsItemIntrofiles>,
    /// intro attachments files
    #[serde(rename = "introattachments")]
    pub r#introattachments: Option<r#ReturnsCoursesItemAssignmentsItemIntroattachments>,
    /// Description of activity
    #[serde(rename = "activity")]
    pub r#activity: Option<String>,
    /// activity format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "activityformat")]
    pub r#activityformat: Option<i64>,
    /// Files from activity field
    #[serde(rename = "activityattachments")]
    pub r#activityattachments: Option<r#ReturnsCoursesItemAssignmentsItemActivityattachments>,
    /// Time limit to complete assigment
    #[serde(rename = "timelimit")]
    pub r#timelimit: Option<i64>,
    /// Flag to only show files during submission
    #[serde(rename = "submissionattachments")]
    pub r#submissionattachments: Option<i64>,
}

/// assignment info
pub type r#ReturnsCoursesItemAssignments = Vec<ReturnsCoursesItemAssignmentsItem>;

/// course information object
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItem {
    /// course id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// course full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// course short name
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// last time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// assignment info
    #[serde(rename = "assignments")]
    pub r#assignments: Option<r#ReturnsCoursesItemAssignments>,
}

/// list of courses
pub type r#ReturnsCourses = Vec<ReturnsCoursesItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item can be 'course' (errorcode 1 or 2) or 'module' (errorcode 1)
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// When item is a course then itemid is a course id. When the item is a module then itemid is a module id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// errorcode can be 1 (no access rights) or 2 (not enrolled or no permissions)
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
    /// list of courses
    #[serde(rename = "courses")]
    pub r#courses: Option<r#ReturnsCourses>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_assign_get_assignments", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_get_assignments", params).await
}
