use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The field to search can be left empty for all courses or: id: course id ids: comma separated course ids shortname: course short name idnumber: course id number category: category id the course belongs to
    #[serde(rename = "field")]
    pub r#field: Option<String>,
    /// The value to match
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemSummaryfilesItem {
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

/// summary files in the summary field
pub type r#ReturnsCoursesItemSummaryfiles = Vec<ReturnsCoursesItemSummaryfilesItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemOverviewfilesItem {
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

/// additional overview files attached to this course
pub type r#ReturnsCoursesItemOverviewfiles = Vec<ReturnsCoursesItemOverviewfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemContactsItem {
    /// contact user id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// contact user fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
}

/// contact users
pub type r#ReturnsCoursesItemContacts = Vec<ReturnsCoursesItemContactsItem>;

/// enrollment methods list
pub type r#ReturnsCoursesItemEnrollmentmethods = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemCustomfieldsItem {
    /// The name of the custom field
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The shortname of the custom field - to be able to build the field class in the code
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// The type of the custom field - text field, checkbox...
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The raw value of the custom field
    #[serde(rename = "valueraw")]
    pub r#valueraw: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Custom fields
pub type r#ReturnsCoursesItemCustomfields = Vec<ReturnsCoursesItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemFiltersItem {
    /// Filter plugin name
    #[serde(rename = "filter")]
    pub r#filter: Option<String>,
    /// Filter state: 1 for on, -1 for off, 0 if inherit
    #[serde(rename = "localstate")]
    pub r#localstate: Option<i64>,
    /// 1 or 0 to use when localstate is set to inherit
    #[serde(rename = "inheritedstate")]
    pub r#inheritedstate: Option<i64>,
}

/// Course filters
pub type r#ReturnsCoursesItemFilters = Vec<ReturnsCoursesItemFiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemCourseformatoptionsItem {
    /// Course format option name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Course format option value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Additional options for particular course format.
pub type r#ReturnsCoursesItemCourseformatoptions = Vec<ReturnsCoursesItemCourseformatoptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItem {
    /// course id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// course full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// course display name
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// course short name
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// Course image
    #[serde(rename = "courseimage")]
    pub r#courseimage: Option<String>,
    /// category id
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// category name
    #[serde(rename = "categoryname")]
    pub r#categoryname: Option<String>,
    /// Sort order in the category
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// summary
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// summary files in the summary field
    #[serde(rename = "summaryfiles")]
    pub r#summaryfiles: Option<r#ReturnsCoursesItemSummaryfiles>,
    /// additional overview files attached to this course
    #[serde(rename = "overviewfiles")]
    pub r#overviewfiles: Option<r#ReturnsCoursesItemOverviewfiles>,
    /// Whether the activity dates are shown or not
    #[serde(rename = "showactivitydates")]
    pub r#showactivitydates: Option<bool>,
    /// Whether the activity completion conditions are shown or not
    #[serde(rename = "showcompletionconditions")]
    pub r#showcompletionconditions: Option<bool>,
    /// contact users
    #[serde(rename = "contacts")]
    pub r#contacts: Option<r#ReturnsCoursesItemContacts>,
    /// enrollment methods list
    #[serde(rename = "enrollmentmethods")]
    pub r#enrollmentmethods: Option<r#ReturnsCoursesItemEnrollmentmethods>,
    /// Custom fields
    #[serde(rename = "customfields")]
    pub r#customfields: Option<r#ReturnsCoursesItemCustomfields>,
    /// Id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// Course format: weeks, topics, social, site,..
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// 1 if grades are shown, otherwise 0
    #[serde(rename = "showgrades")]
    pub r#showgrades: Option<i64>,
    /// Number of recent items appearing on the course page
    #[serde(rename = "newsitems")]
    pub r#newsitems: Option<i64>,
    /// Timestamp when the course start
    #[serde(rename = "startdate")]
    pub r#startdate: Option<i64>,
    /// Timestamp when the course end
    #[serde(rename = "enddate")]
    pub r#enddate: Option<i64>,
    /// Largest size of file that can be uploaded into
    #[serde(rename = "maxbytes")]
    pub r#maxbytes: Option<i64>,
    /// Are activity report shown (yes = 1, no =0)
    #[serde(rename = "showreports")]
    pub r#showreports: Option<i64>,
    /// 1: available to student, 0:not available
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// no group, separate, visible
    #[serde(rename = "groupmode")]
    pub r#groupmode: Option<i64>,
    /// 1: yes, 0: no
    #[serde(rename = "groupmodeforce")]
    pub r#groupmodeforce: Option<i64>,
    /// default grouping id
    #[serde(rename = "defaultgroupingid")]
    pub r#defaultgroupingid: Option<i64>,
    /// Completion enabled? 1: yes 0: no
    #[serde(rename = "enablecompletion")]
    pub r#enablecompletion: Option<i64>,
    /// 1: yes 0: no
    #[serde(rename = "completionnotify")]
    pub r#completionnotify: Option<i64>,
    /// Forced course language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// Fame of the forced theme
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
    /// Current course marker
    #[serde(rename = "marker")]
    pub r#marker: Option<i64>,
    /// If legacy files are enabled
    #[serde(rename = "legacyfiles")]
    pub r#legacyfiles: Option<i64>,
    /// Calendar type
    #[serde(rename = "calendartype")]
    pub r#calendartype: Option<String>,
    /// Time when the course was created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Last time  the course was updated
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// If is a requested course
    #[serde(rename = "requested")]
    pub r#requested: Option<i64>,
    /// Cache revision number
    #[serde(rename = "cacherev")]
    pub r#cacherev: Option<i64>,
    /// Course filters
    #[serde(rename = "filters")]
    pub r#filters: Option<r#ReturnsCoursesItemFilters>,
    /// Additional options for particular course format.
    #[serde(rename = "courseformatoptions")]
    pub r#courseformatoptions: Option<r#ReturnsCoursesItemCourseformatoptions>,
}

/// Course
pub type r#ReturnsCourses = Vec<ReturnsCoursesItem>;

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
    /// Course
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
    let json = client
        .post("core_course_get_courses_by_field", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_course_get_courses_by_field", params)
        .await
}
