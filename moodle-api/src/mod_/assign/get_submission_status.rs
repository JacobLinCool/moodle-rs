use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// assignment instance id
    #[serde(rename = "assignid")]
    pub r#assignid: Option<i64>,
    /// user id (empty for current user)
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// filter by users in group (used for generating the grading summary). 0 for all groups information, any other empty value will calculate currrent group.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

/// Grading information.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGradingsummary {
    /// Number of users who can submit.
    #[serde(rename = "participantcount")]
    pub r#participantcount: Option<i64>,
    /// Number of submissions in draft status.
    #[serde(rename = "submissiondraftscount")]
    pub r#submissiondraftscount: Option<i64>,
    /// Whether submissions are enabled or not.
    #[serde(rename = "submissionsenabled")]
    pub r#submissionsenabled: Option<bool>,
    /// Number of submissions in submitted status.
    #[serde(rename = "submissionssubmittedcount")]
    pub r#submissionssubmittedcount: Option<i64>,
    /// Number of submissions that need grading.
    #[serde(rename = "submissionsneedgradingcount")]
    pub r#submissionsneedgradingcount: Option<i64>,
    /// Whether we need to warn people that there are users without groups ('warningrequired'), warn people there are users who will submit in the default group ('warningoptional') or no warning ('').
    #[serde(rename = "warnofungroupedusers")]
    pub r#warnofungroupedusers: Option<String>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptSubmissionPluginsItemFileareasItemFilesItem {
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

/// files
pub type r#ReturnsLastattemptSubmissionPluginsItemFileareasItemFiles =
    Vec<ReturnsLastattemptSubmissionPluginsItemFileareasItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptSubmissionPluginsItemFileareasItem {
    /// file area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsLastattemptSubmissionPluginsItemFileareasItemFiles>,
}

/// fileareas
pub type r#ReturnsLastattemptSubmissionPluginsItemFileareas =
    Vec<ReturnsLastattemptSubmissionPluginsItemFileareasItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptSubmissionPluginsItemEditorfieldsItem {
    /// field name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// field description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// field value
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
}

/// editorfields
pub type r#ReturnsLastattemptSubmissionPluginsItemEditorfields =
    Vec<ReturnsLastattemptSubmissionPluginsItemEditorfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptSubmissionPluginsItem {
    /// submission plugin type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// submission plugin name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// fileareas
    #[serde(rename = "fileareas")]
    pub r#fileareas: Option<r#ReturnsLastattemptSubmissionPluginsItemFileareas>,
    /// editorfields
    #[serde(rename = "editorfields")]
    pub r#editorfields: Option<r#ReturnsLastattemptSubmissionPluginsItemEditorfields>,
}

/// plugins
pub type r#ReturnsLastattemptSubmissionPlugins = Vec<ReturnsLastattemptSubmissionPluginsItem>;

/// submission info
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptSubmission {
    /// submission id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// student id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// attempt number
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// submission creation time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// submission last modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// submission start time
    #[serde(rename = "timestarted")]
    pub r#timestarted: Option<i64>,
    /// submission status
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// assignment id
    #[serde(rename = "assignment")]
    pub r#assignment: Option<i64>,
    /// latest attempt
    #[serde(rename = "latest")]
    pub r#latest: Option<i64>,
    /// plugins
    #[serde(rename = "plugins")]
    pub r#plugins: Option<r#ReturnsLastattemptSubmissionPlugins>,
    /// Grading status.
    #[serde(rename = "gradingstatus")]
    pub r#gradingstatus: Option<String>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptTeamsubmissionPluginsItemFileareasItemFilesItem {
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

/// files
pub type r#ReturnsLastattemptTeamsubmissionPluginsItemFileareasItemFiles =
    Vec<ReturnsLastattemptTeamsubmissionPluginsItemFileareasItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptTeamsubmissionPluginsItemFileareasItem {
    /// file area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsLastattemptTeamsubmissionPluginsItemFileareasItemFiles>,
}

/// fileareas
pub type r#ReturnsLastattemptTeamsubmissionPluginsItemFileareas =
    Vec<ReturnsLastattemptTeamsubmissionPluginsItemFileareasItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptTeamsubmissionPluginsItemEditorfieldsItem {
    /// field name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// field description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// field value
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
}

/// editorfields
pub type r#ReturnsLastattemptTeamsubmissionPluginsItemEditorfields =
    Vec<ReturnsLastattemptTeamsubmissionPluginsItemEditorfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptTeamsubmissionPluginsItem {
    /// submission plugin type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// submission plugin name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// fileareas
    #[serde(rename = "fileareas")]
    pub r#fileareas: Option<r#ReturnsLastattemptTeamsubmissionPluginsItemFileareas>,
    /// editorfields
    #[serde(rename = "editorfields")]
    pub r#editorfields: Option<r#ReturnsLastattemptTeamsubmissionPluginsItemEditorfields>,
}

/// plugins
pub type r#ReturnsLastattemptTeamsubmissionPlugins =
    Vec<ReturnsLastattemptTeamsubmissionPluginsItem>;

/// submission info
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattemptTeamsubmission {
    /// submission id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// student id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// attempt number
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// submission creation time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// submission last modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// submission start time
    #[serde(rename = "timestarted")]
    pub r#timestarted: Option<i64>,
    /// submission status
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// assignment id
    #[serde(rename = "assignment")]
    pub r#assignment: Option<i64>,
    /// latest attempt
    #[serde(rename = "latest")]
    pub r#latest: Option<i64>,
    /// plugins
    #[serde(rename = "plugins")]
    pub r#plugins: Option<r#ReturnsLastattemptTeamsubmissionPlugins>,
    /// Grading status.
    #[serde(rename = "gradingstatus")]
    pub r#gradingstatus: Option<String>,
}

/// List of users who still need to submit (for group submissions only).
pub type r#ReturnsLastattemptSubmissiongroupmemberswhoneedtosubmit = Vec<i64>;

/// User groups in the course.
pub type r#ReturnsLastattemptUsergroups = Vec<i64>;

/// Last attempt information.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLastattempt {
    /// submission info
    #[serde(rename = "submission")]
    pub r#submission: Option<ReturnsLastattemptSubmission>,
    /// submission info
    #[serde(rename = "teamsubmission")]
    pub r#teamsubmission: Option<ReturnsLastattemptTeamsubmission>,
    /// The submission group id (for group submissions only).
    #[serde(rename = "submissiongroup")]
    pub r#submissiongroup: Option<i64>,
    /// List of users who still need to submit (for group submissions only).
    #[serde(rename = "submissiongroupmemberswhoneedtosubmit")]
    pub r#submissiongroupmemberswhoneedtosubmit:
        Option<r#ReturnsLastattemptSubmissiongroupmemberswhoneedtosubmit>,
    /// Whether submissions are enabled or not.
    #[serde(rename = "submissionsenabled")]
    pub r#submissionsenabled: Option<bool>,
    /// Whether new submissions are locked.
    #[serde(rename = "locked")]
    pub r#locked: Option<bool>,
    /// Whether the submission is graded.
    #[serde(rename = "graded")]
    pub r#graded: Option<bool>,
    /// Whether the user can edit the current submission.
    #[serde(rename = "canedit")]
    pub r#canedit: Option<bool>,
    /// Whether the owner of the submission can edit it.
    #[serde(rename = "caneditowner")]
    pub r#caneditowner: Option<bool>,
    /// Whether the user can submit.
    #[serde(rename = "cansubmit")]
    pub r#cansubmit: Option<bool>,
    /// Extension due date.
    #[serde(rename = "extensionduedate")]
    pub r#extensionduedate: Option<i64>,
    /// Time limit for submission.
    #[serde(rename = "timelimit")]
    pub r#timelimit: Option<i64>,
    /// Whether blind marking is enabled.
    #[serde(rename = "blindmarking")]
    pub r#blindmarking: Option<bool>,
    /// Grading status.
    #[serde(rename = "gradingstatus")]
    pub r#gradingstatus: Option<String>,
    /// User groups in the course.
    #[serde(rename = "usergroups")]
    pub r#usergroups: Option<r#ReturnsLastattemptUsergroups>,
}

/// grade information
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedbackGrade {
    /// grade id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// assignment id
    #[serde(rename = "assignment")]
    pub r#assignment: Option<i64>,
    /// student id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// attempt number
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// grade creation time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// grade last modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// grader, -1 if grader is hidden
    #[serde(rename = "grader")]
    pub r#grader: Option<i64>,
    /// grade
    #[serde(rename = "grade")]
    pub r#grade: Option<String>,
    /// grade rendered into a format suitable for display
    #[serde(rename = "gradefordisplay")]
    pub r#gradefordisplay: Option<String>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedbackPluginsItemFileareasItemFilesItem {
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

/// files
pub type r#ReturnsFeedbackPluginsItemFileareasItemFiles =
    Vec<ReturnsFeedbackPluginsItemFileareasItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedbackPluginsItemFileareasItem {
    /// file area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsFeedbackPluginsItemFileareasItemFiles>,
}

/// fileareas
pub type r#ReturnsFeedbackPluginsItemFileareas = Vec<ReturnsFeedbackPluginsItemFileareasItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedbackPluginsItemEditorfieldsItem {
    /// field name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// field description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// field value
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
}

/// editorfields
pub type r#ReturnsFeedbackPluginsItemEditorfields = Vec<ReturnsFeedbackPluginsItemEditorfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedbackPluginsItem {
    /// submission plugin type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// submission plugin name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// fileareas
    #[serde(rename = "fileareas")]
    pub r#fileareas: Option<r#ReturnsFeedbackPluginsItemFileareas>,
    /// editorfields
    #[serde(rename = "editorfields")]
    pub r#editorfields: Option<r#ReturnsFeedbackPluginsItemEditorfields>,
}

/// Plugins info.
pub type r#ReturnsFeedbackPlugins = Vec<ReturnsFeedbackPluginsItem>;

/// Feedback for the last attempt.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeedback {
    /// grade information
    #[serde(rename = "grade")]
    pub r#grade: Option<ReturnsFeedbackGrade>,
    /// Grade rendered into a format suitable for display.
    #[serde(rename = "gradefordisplay")]
    pub r#gradefordisplay: Option<String>,
    /// The date the user was graded.
    #[serde(rename = "gradeddate")]
    pub r#gradeddate: Option<i64>,
    /// Plugins info.
    #[serde(rename = "plugins")]
    pub r#plugins: Option<r#ReturnsFeedbackPlugins>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemSubmissionPluginsItemFileareasItemFilesItem {
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

/// files
pub type r#ReturnsPreviousattemptsItemSubmissionPluginsItemFileareasItemFiles =
    Vec<ReturnsPreviousattemptsItemSubmissionPluginsItemFileareasItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemSubmissionPluginsItemFileareasItem {
    /// file area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsPreviousattemptsItemSubmissionPluginsItemFileareasItemFiles>,
}

/// fileareas
pub type r#ReturnsPreviousattemptsItemSubmissionPluginsItemFileareas =
    Vec<ReturnsPreviousattemptsItemSubmissionPluginsItemFileareasItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemSubmissionPluginsItemEditorfieldsItem {
    /// field name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// field description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// field value
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
}

/// editorfields
pub type r#ReturnsPreviousattemptsItemSubmissionPluginsItemEditorfields =
    Vec<ReturnsPreviousattemptsItemSubmissionPluginsItemEditorfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemSubmissionPluginsItem {
    /// submission plugin type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// submission plugin name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// fileareas
    #[serde(rename = "fileareas")]
    pub r#fileareas: Option<r#ReturnsPreviousattemptsItemSubmissionPluginsItemFileareas>,
    /// editorfields
    #[serde(rename = "editorfields")]
    pub r#editorfields: Option<r#ReturnsPreviousattemptsItemSubmissionPluginsItemEditorfields>,
}

/// plugins
pub type r#ReturnsPreviousattemptsItemSubmissionPlugins =
    Vec<ReturnsPreviousattemptsItemSubmissionPluginsItem>;

/// submission info
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemSubmission {
    /// submission id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// student id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// attempt number
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// submission creation time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// submission last modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// submission start time
    #[serde(rename = "timestarted")]
    pub r#timestarted: Option<i64>,
    /// submission status
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// assignment id
    #[serde(rename = "assignment")]
    pub r#assignment: Option<i64>,
    /// latest attempt
    #[serde(rename = "latest")]
    pub r#latest: Option<i64>,
    /// plugins
    #[serde(rename = "plugins")]
    pub r#plugins: Option<r#ReturnsPreviousattemptsItemSubmissionPlugins>,
    /// Grading status.
    #[serde(rename = "gradingstatus")]
    pub r#gradingstatus: Option<String>,
}

/// grade information
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemGrade {
    /// grade id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// assignment id
    #[serde(rename = "assignment")]
    pub r#assignment: Option<i64>,
    /// student id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// attempt number
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// grade creation time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// grade last modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// grader, -1 if grader is hidden
    #[serde(rename = "grader")]
    pub r#grader: Option<i64>,
    /// grade
    #[serde(rename = "grade")]
    pub r#grade: Option<String>,
    /// grade rendered into a format suitable for display
    #[serde(rename = "gradefordisplay")]
    pub r#gradefordisplay: Option<String>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemFeedbackpluginsItemFileareasItemFilesItem {
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

/// files
pub type r#ReturnsPreviousattemptsItemFeedbackpluginsItemFileareasItemFiles =
    Vec<ReturnsPreviousattemptsItemFeedbackpluginsItemFileareasItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemFeedbackpluginsItemFileareasItem {
    /// file area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsPreviousattemptsItemFeedbackpluginsItemFileareasItemFiles>,
}

/// fileareas
pub type r#ReturnsPreviousattemptsItemFeedbackpluginsItemFileareas =
    Vec<ReturnsPreviousattemptsItemFeedbackpluginsItemFileareasItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemFeedbackpluginsItemEditorfieldsItem {
    /// field name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// field description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// field value
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
}

/// editorfields
pub type r#ReturnsPreviousattemptsItemFeedbackpluginsItemEditorfields =
    Vec<ReturnsPreviousattemptsItemFeedbackpluginsItemEditorfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItemFeedbackpluginsItem {
    /// submission plugin type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// submission plugin name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// fileareas
    #[serde(rename = "fileareas")]
    pub r#fileareas: Option<r#ReturnsPreviousattemptsItemFeedbackpluginsItemFileareas>,
    /// editorfields
    #[serde(rename = "editorfields")]
    pub r#editorfields: Option<r#ReturnsPreviousattemptsItemFeedbackpluginsItemEditorfields>,
}

/// Feedback info.
pub type r#ReturnsPreviousattemptsItemFeedbackplugins =
    Vec<ReturnsPreviousattemptsItemFeedbackpluginsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreviousattemptsItem {
    /// Attempt number.
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// submission info
    #[serde(rename = "submission")]
    pub r#submission: Option<ReturnsPreviousattemptsItemSubmission>,
    /// grade information
    #[serde(rename = "grade")]
    pub r#grade: Option<ReturnsPreviousattemptsItemGrade>,
    /// Feedback info.
    #[serde(rename = "feedbackplugins")]
    pub r#feedbackplugins: Option<r#ReturnsPreviousattemptsItemFeedbackplugins>,
}

/// List all the previous attempts did by the user.
pub type r#ReturnsPreviousattempts = Vec<ReturnsPreviousattemptsItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentdataAttachmentsIntroItem {
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

/// Intro attachments files
pub type r#ReturnsAssignmentdataAttachmentsIntro = Vec<ReturnsAssignmentdataAttachmentsIntroItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentdataAttachmentsActivityItem {
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

/// Activity attachments files
pub type r#ReturnsAssignmentdataAttachmentsActivity =
    Vec<ReturnsAssignmentdataAttachmentsActivityItem>;

/// Intro and activity attachments
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentdataAttachments {
    /// Intro attachments files
    #[serde(rename = "intro")]
    pub r#intro: Option<r#ReturnsAssignmentdataAttachmentsIntro>,
    /// Activity attachments files
    #[serde(rename = "activity")]
    pub r#activity: Option<r#ReturnsAssignmentdataAttachmentsActivity>,
}

/// Extra information about assignment
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentdata {
    /// Intro and activity attachments
    #[serde(rename = "attachments")]
    pub r#attachments: Option<ReturnsAssignmentdataAttachments>,
    /// Text of activity
    #[serde(rename = "activity")]
    pub r#activity: Option<String>,
    /// activity format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "activityformat")]
    pub r#activityformat: Option<i64>,
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
    /// Grading information.
    #[serde(rename = "gradingsummary")]
    pub r#gradingsummary: Option<ReturnsGradingsummary>,
    /// Last attempt information.
    #[serde(rename = "lastattempt")]
    pub r#lastattempt: Option<ReturnsLastattempt>,
    /// Feedback for the last attempt.
    #[serde(rename = "feedback")]
    pub r#feedback: Option<ReturnsFeedback>,
    /// List all the previous attempts did by the user.
    #[serde(rename = "previousattempts")]
    pub r#previousattempts: Option<r#ReturnsPreviousattempts>,
    /// Extra information about assignment
    #[serde(rename = "assignmentdata")]
    pub r#assignmentdata: Option<ReturnsAssignmentdata>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_assign_get_submission_status", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_assign_get_submission_status", params)
        .await
}
