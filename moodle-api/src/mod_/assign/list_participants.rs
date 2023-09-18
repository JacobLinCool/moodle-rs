use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// assign instance id
    #[serde(rename = "assignid")]
    pub r#assignid: Option<i64>,
    /// group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// search string to filter the results
    #[serde(rename = "filter")]
    pub r#filter: Option<String>,
    /// number of records to skip
    #[serde(rename = "skip")]
    pub r#skip: Option<i64>,
    /// maximum number of records to return
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
    /// Do not return all user fields
    #[serde(rename = "onlyids")]
    pub r#onlyids: Option<bool>,
    /// Do return courses where the user is enrolled
    #[serde(rename = "includeenrolments")]
    pub r#includeenrolments: Option<bool>,
    /// Apply current user table sorting preferences.
    #[serde(rename = "tablesort")]
    pub r#tablesort: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemCustomfieldsItem {
    /// The type of the custom field - text field, checkbox...
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the custom field (as stored in the database)
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// The value of the custom field for display
    #[serde(rename = "displayvalue")]
    pub r#displayvalue: Option<String>,
    /// The name of the custom field
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The shortname of the custom field - to be able to build the field class in the code
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
}

/// User custom fields (also known as user profile fields)
pub type r#ReturnsItemCustomfields = Vec<ReturnsItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemPreferencesItem {
    /// The name of the preferences
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the preference
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Users preferences
pub type r#ReturnsItemPreferences = Vec<ReturnsItemPreferencesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemGroupsItem {
    /// group id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// group name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// group description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}

/// user groups
pub type r#ReturnsItemGroups = Vec<ReturnsItemGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemRolesItem {
    /// role id
    #[serde(rename = "roleid")]
    pub r#roleid: Option<i64>,
    /// role name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// role shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// role sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
}

/// user roles
pub type r#ReturnsItemRoles = Vec<ReturnsItemRolesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemEnrolledcoursesItem {
    /// Id of the course
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Fullname of the course
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// Shortname of the course
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
}

/// Courses where the user is enrolled - limited by which courses the user is able to see
pub type r#ReturnsItemEnrolledcourses = Vec<ReturnsItemEnrolledcoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// ID of the user
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The username
    #[serde(rename = "username")]
    pub r#username: Option<String>,
    /// The first name(s) of the user
    #[serde(rename = "firstname")]
    pub r#firstname: Option<String>,
    /// The family name of the user
    #[serde(rename = "lastname")]
    pub r#lastname: Option<String>,
    /// The fullname of the user
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// Email address
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// Postal address
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// Phone 1
    #[serde(rename = "phone1")]
    pub r#phone1: Option<String>,
    /// Phone 2
    #[serde(rename = "phone2")]
    pub r#phone2: Option<String>,
    /// department
    #[serde(rename = "department")]
    pub r#department: Option<String>,
    /// institution
    #[serde(rename = "institution")]
    pub r#institution: Option<String>,
    /// The idnumber of the user
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// user interests (separated by commas)
    #[serde(rename = "interests")]
    pub r#interests: Option<String>,
    /// first access to the site (0 if never)
    #[serde(rename = "firstaccess")]
    pub r#firstaccess: Option<i64>,
    /// last access to the site (0 if never)
    #[serde(rename = "lastaccess")]
    pub r#lastaccess: Option<i64>,
    /// Suspend user account, either false to enable user login or true to disable it
    #[serde(rename = "suspended")]
    pub r#suspended: Option<bool>,
    /// User profile description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// int format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// Home city of the user
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// Home country code of the user, such as AU or CZ
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// User image profile URL - small version
    #[serde(rename = "profileimageurlsmall")]
    pub r#profileimageurlsmall: Option<String>,
    /// User image profile URL - big version
    #[serde(rename = "profileimageurl")]
    pub r#profileimageurl: Option<String>,
    /// User custom fields (also known as user profile fields)
    #[serde(rename = "customfields")]
    pub r#customfields: Option<r#ReturnsItemCustomfields>,
    /// Users preferences
    #[serde(rename = "preferences")]
    pub r#preferences: Option<r#ReturnsItemPreferences>,
    /// record id
    #[serde(rename = "recordid")]
    pub r#recordid: Option<i64>,
    /// user groups
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ReturnsItemGroups>,
    /// user roles
    #[serde(rename = "roles")]
    pub r#roles: Option<r#ReturnsItemRoles>,
    /// Courses where the user is enrolled - limited by which courses the user is able to see
    #[serde(rename = "enrolledcourses")]
    pub r#enrolledcourses: Option<r#ReturnsItemEnrolledcourses>,
    /// have they submitted their assignment
    #[serde(rename = "submitted")]
    pub r#submitted: Option<bool>,
    /// is their submission waiting for grading
    #[serde(rename = "requiregrading")]
    pub r#requiregrading: Option<bool>,
    /// have they been granted an extension
    #[serde(rename = "grantedextension")]
    pub r#grantedextension: Option<bool>,
    /// The submission status (new, draft, reopened or submitted). Empty when not submitted.
    #[serde(rename = "submissionstatus")]
    pub r#submissionstatus: Option<String>,
    /// for group assignments this is the group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// for group assignments this is the group name
    #[serde(rename = "groupname")]
    pub r#groupname: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_assign_list_participants", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_list_participants", params).await
}
