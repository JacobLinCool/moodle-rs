use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Include count of enrolled users for each course? This can add several seconds to the response time if a user is on several large courses, so set this to false if the value will not be used to improve performance.
    #[serde(rename = "returnusercount")]
    pub r#returnusercount: Option<bool>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemOverviewfilesItem {
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

/// Overview files attached to this course.
pub type r#ReturnsItemOverviewfiles = Vec<ReturnsItemOverviewfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// id of course
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// short name of course
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// long name of course
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// course display name for lists.
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// Number of enrolled users in this course
    #[serde(rename = "enrolledusercount")]
    pub r#enrolledusercount: Option<i64>,
    /// id number of course
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// 1 means visible, 0 means not yet visible course
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// summary
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// course format: weeks, topics, social, site
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// The course image URL
    #[serde(rename = "courseimage")]
    pub r#courseimage: Option<String>,
    /// true if grades are shown, otherwise false
    #[serde(rename = "showgrades")]
    pub r#showgrades: Option<bool>,
    /// forced course language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// true if completion is enabled, otherwise false
    #[serde(rename = "enablecompletion")]
    pub r#enablecompletion: Option<bool>,
    /// If completion criteria is set.
    #[serde(rename = "completionhascriteria")]
    pub r#completionhascriteria: Option<bool>,
    /// If the user is completion tracked.
    #[serde(rename = "completionusertracked")]
    pub r#completionusertracked: Option<bool>,
    /// course category id
    #[serde(rename = "category")]
    pub r#category: Option<i64>,
    /// Progress percentage
    #[serde(rename = "progress")]
    pub r#progress: Option<f64>,
    /// Whether the course is completed.
    #[serde(rename = "completed")]
    pub r#completed: Option<bool>,
    /// Timestamp when the course start
    #[serde(rename = "startdate")]
    pub r#startdate: Option<i64>,
    /// Timestamp when the course end
    #[serde(rename = "enddate")]
    pub r#enddate: Option<i64>,
    /// Course section marker.
    #[serde(rename = "marker")]
    pub r#marker: Option<i64>,
    /// Last access to the course (timestamp).
    #[serde(rename = "lastaccess")]
    pub r#lastaccess: Option<i64>,
    /// If the user marked this course a favourite.
    #[serde(rename = "isfavourite")]
    pub r#isfavourite: Option<bool>,
    /// If the user hide the course from the dashboard.
    #[serde(rename = "hidden")]
    pub r#hidden: Option<bool>,
    /// Overview files attached to this course.
    #[serde(rename = "overviewfiles")]
    pub r#overviewfiles: Option<r#ReturnsItemOverviewfiles>,
    /// Whether the activity dates are shown or not
    #[serde(rename = "showactivitydates")]
    pub r#showactivitydates: Option<bool>,
    /// Whether the activity completion conditions are shown or not
    #[serde(rename = "showcompletionconditions")]
    pub r#showcompletionconditions: Option<bool>,
    /// Last time course settings were updated (timestamp).
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_enrol_get_users_courses", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_enrol_get_users_courses", params).await
}
