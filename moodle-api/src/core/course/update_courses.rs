use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCoursesItemCourseformatoptionsItem {
    /// course format option name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// course format option value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// additional options for particular course format
pub type r#ParamsCoursesItemCourseformatoptions = Vec<ParamsCoursesItemCourseformatoptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCoursesItemCustomfieldsItem {
    /// The shortname of the custom field
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Custom fields
pub type r#ParamsCoursesItemCustomfields = Vec<ParamsCoursesItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCoursesItem {
    /// ID of the course
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// course short name
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// category id
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// summary
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// course format: weeks, topics, social, site,..
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// 1 if grades are shown, otherwise 0
    #[serde(rename = "showgrades")]
    pub r#showgrades: Option<i64>,
    /// number of recent items appearing on the course page
    #[serde(rename = "newsitems")]
    pub r#newsitems: Option<i64>,
    /// timestamp when the course start
    #[serde(rename = "startdate")]
    pub r#startdate: Option<i64>,
    /// timestamp when the course end
    #[serde(rename = "enddate")]
    pub r#enddate: Option<i64>,
    /// (deprecated, use courseformatoptions) number of weeks/topics
    #[serde(rename = "numsections")]
    pub r#numsections: Option<i64>,
    /// largest size of file that can be uploaded into the course
    #[serde(rename = "maxbytes")]
    pub r#maxbytes: Option<i64>,
    /// are activity report shown (yes = 1, no =0)
    #[serde(rename = "showreports")]
    pub r#showreports: Option<i64>,
    /// 1: available to student, 0:not available
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// (deprecated, use courseformatoptions) How the hidden sections in the course are displayed to students
    #[serde(rename = "hiddensections")]
    pub r#hiddensections: Option<i64>,
    /// no group, separate, visible
    #[serde(rename = "groupmode")]
    pub r#groupmode: Option<i64>,
    /// 1: yes, 0: no
    #[serde(rename = "groupmodeforce")]
    pub r#groupmodeforce: Option<i64>,
    /// default grouping id
    #[serde(rename = "defaultgroupingid")]
    pub r#defaultgroupingid: Option<i64>,
    /// Enabled, control via completion and activity settings. Disabled, not shown in activity settings.
    #[serde(rename = "enablecompletion")]
    pub r#enablecompletion: Option<i64>,
    /// 1: yes 0: no
    #[serde(rename = "completionnotify")]
    pub r#completionnotify: Option<i64>,
    /// forced course language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// name of the force theme
    #[serde(rename = "forcetheme")]
    pub r#forcetheme: Option<String>,
    /// additional options for particular course format
    #[serde(rename = "courseformatoptions")]
    pub r#courseformatoptions: Option<r#ParamsCoursesItemCourseformatoptions>,
    /// Custom fields
    #[serde(rename = "customfields")]
    pub r#customfields: Option<r#ParamsCoursesItemCustomfields>,
}

/// courses to update
pub type r#ParamsCourses = Vec<ParamsCoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// courses to update
    #[serde(rename = "courses")]
    pub r#courses: Option<r#ParamsCourses>,
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
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_course_update_courses", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_course_update_courses", params).await
}
