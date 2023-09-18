use serde::{self, Deserialize, Serialize};

/// Optional list of required capabilities (used to filter the list)
pub type r#ParamsRequiredcapabilities = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// criteria name (search, modulelist (only admins), blocklist (only admins), tagid)
    #[serde(rename = "criterianame")]
    pub r#criterianame: Option<String>,
    /// criteria value
    #[serde(rename = "criteriavalue")]
    pub r#criteriavalue: Option<String>,
    /// page number (0 based)
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// items per page
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
    /// Optional list of required capabilities (used to filter the list)
    #[serde(rename = "requiredcapabilities")]
    pub r#requiredcapabilities: Option<r#ParamsRequiredcapabilities>,
    /// limit to enrolled courses
    #[serde(rename = "limittoenrolled")]
    pub r#limittoenrolled: Option<bool>,
    /// limit to courses where completion is enabled
    #[serde(rename = "onlywithcompletion")]
    pub r#onlywithcompletion: Option<bool>,
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
}

/// course
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
    /// total course count
    #[serde(rename = "total")]
    pub r#total: Option<i64>,
    /// course
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
    let json = client.post("core_course_search_courses", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_course_search_courses", params).await
}
