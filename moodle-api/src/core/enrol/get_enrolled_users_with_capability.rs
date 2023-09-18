use serde::{self, Deserialize, Serialize};

pub type r#ParamsCoursecapabilitiesItemCapabilities = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCoursecapabilitiesItem {
    /// Course ID number in the Moodle course table
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    #[serde(rename = "capabilities")]
    pub r#capabilities: Option<r#ParamsCoursecapabilitiesItemCapabilities>,
}

/// course id and associated capability name
pub type r#ParamsCoursecapabilities = Vec<ParamsCoursecapabilitiesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptionsItem {
    /// option name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// option value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Option names:

/// * groupid (integer) return only users in this group id. Requires 'moodle/site:accessallgroups' .

/// * onlyactive (integer) only users with active enrolments. Requires 'moodle/course:enrolreview' .

/// * userfields ('string, string, ...') return only the values of these user fields.

/// * limitfrom (integer) sql limit from.

/// * limitnumber (integer) max number of users per course and capability.
pub type r#ParamsOptions = Vec<ParamsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// course id and associated capability name
    #[serde(rename = "coursecapabilities")]
    pub r#coursecapabilities: Option<r#ParamsCoursecapabilities>,
    /// Option names: * groupid (integer) return only users in this group id. Requires 'moodle/site:accessallgroups' . * onlyactive (integer) only users with active enrolments. Requires 'moodle/course:enrolreview' . * userfields ('string, string, ...') return only the values of these user fields. * limitfrom (integer) sql limit from. * limitnumber (integer) max number of users per course and capability.
    #[serde(rename = "options")]
    pub r#options: Option<r#ParamsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemUsersItemCustomfieldsItem {
    /// The type of the custom field
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// The name of the custom field
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The shortname of the custom field
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
}

/// User custom fields (also known as user profil fields)
pub type r#ReturnsItemUsersItemCustomfields = Vec<ReturnsItemUsersItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemUsersItemGroupsItem {
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
pub type r#ReturnsItemUsersItemGroups = Vec<ReturnsItemUsersItemGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemUsersItemRolesItem {
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
pub type r#ReturnsItemUsersItemRoles = Vec<ReturnsItemUsersItemRolesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemUsersItemPreferencesItem {
    /// The name of the preferences
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// User preferences
pub type r#ReturnsItemUsersItemPreferences = Vec<ReturnsItemUsersItemPreferencesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemUsersItemEnrolledcoursesItem {
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
pub type r#ReturnsItemUsersItemEnrolledcourses = Vec<ReturnsItemUsersItemEnrolledcoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemUsersItem {
    /// ID of the user
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Username
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
    /// user interests (separated by commas)
    #[serde(rename = "interests")]
    pub r#interests: Option<String>,
    /// first access to the site (0 if never)
    #[serde(rename = "firstaccess")]
    pub r#firstaccess: Option<i64>,
    /// last access to the site (0 if never)
    #[serde(rename = "lastaccess")]
    pub r#lastaccess: Option<i64>,
    /// last access to the course (0 if never)
    #[serde(rename = "lastcourseaccess")]
    pub r#lastcourseaccess: Option<i64>,
    /// User profile description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// User profile description format
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// Home city of the user
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// Country code of the user, such as AU or CZ
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// User image profile URL - small
    #[serde(rename = "profileimageurlsmall")]
    pub r#profileimageurlsmall: Option<String>,
    /// User image profile URL - big
    #[serde(rename = "profileimageurl")]
    pub r#profileimageurl: Option<String>,
    /// User custom fields (also known as user profil fields)
    #[serde(rename = "customfields")]
    pub r#customfields: Option<r#ReturnsItemUsersItemCustomfields>,
    /// user groups
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ReturnsItemUsersItemGroups>,
    /// user roles
    #[serde(rename = "roles")]
    pub r#roles: Option<r#ReturnsItemUsersItemRoles>,
    /// User preferences
    #[serde(rename = "preferences")]
    pub r#preferences: Option<r#ReturnsItemUsersItemPreferences>,
    /// Courses where the user is enrolled - limited by which courses the user is able to see
    #[serde(rename = "enrolledcourses")]
    pub r#enrolledcourses: Option<r#ReturnsItemUsersItemEnrolledcourses>,
}

/// List of users that are enrolled in the course and have the specified capability
pub type r#ReturnsItemUsers = Vec<ReturnsItemUsersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Course ID number in the Moodle course table
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Capability name
    #[serde(rename = "capability")]
    pub r#capability: Option<String>,
    /// List of users that are enrolled in the course and have the specified capability
    #[serde(rename = "users")]
    pub r#users: Option<r#ReturnsItemUsers>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_enrol_get_enrolled_users_with_capability", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_enrol_get_enrolled_users_with_capability", params)
        .await
}
