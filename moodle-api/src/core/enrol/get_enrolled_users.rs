use serde::{self, Deserialize, Serialize};

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

/// * withcapability (string) return only users with this capability. This option requires 'moodle/role:review' on the course context.

/// * groupid (integer) return only users in this group id. If the course has groups enabled and this param

/// isn't defined, returns all the viewable users.

/// This option requires 'moodle/site:accessallgroups' on the course context if the

/// user doesn't belong to the group.

/// * onlyactive (integer) return only users with active enrolments and matching time restrictions.

/// This option requires 'moodle/course:enrolreview' on the course context.

/// Please note that this option can't

/// be used together with onlysuspended (only one can be active).

/// * onlysuspended (integer) return only suspended users. This option requires

/// 'moodle/course:enrolreview' on the course context. Please note that this option can't

/// be used together with onlyactive (only one can be active).

/// * userfields ('string, string, ...') return only the values of these user fields.

/// * limitfrom (integer) sql limit from.

/// * limitnumber (integer) maximum number of returned users.

/// * sortby (string) sort by id, firstname or lastname. For ordering like the site does, use siteorder.

/// * sortdirection (string) ASC or DESC
pub type r#ParamsOptions = Vec<ParamsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Option names: * withcapability (string) return only users with this capability. This option requires 'moodle/role:review' on the course context. * groupid (integer) return only users in this group id. If the course has groups enabled and this param isn't defined, returns all the viewable users. This option requires 'moodle/site:accessallgroups' on the course context if the user doesn't belong to the group. * onlyactive (integer) return only users with active enrolments and matching time restrictions. This option requires 'moodle/course:enrolreview' on the course context. Please note that this option can't be used together with onlysuspended (only one can be active). * onlysuspended (integer) return only suspended users. This option requires 'moodle/course:enrolreview' on the course context. Please note that this option can't be used together with onlyactive (only one can be active). * userfields ('string, string, ...') return only the values of these user fields. * limitfrom (integer) sql limit from. * limitnumber (integer) maximum number of returned users. * sortby (string) sort by id, firstname or lastname. For ordering like the site does, use siteorder. * sortdirection (string) ASC or DESC
    #[serde(rename = "options")]
    pub r#options: Option<r#ParamsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemCustomfieldsItem {
    /// The type of the custom field - text field, checkbox...
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// The name of the custom field
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The shortname of the custom field - to be able to build the field class in the code
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
}

/// User custom fields (also known as user profil fields)
pub type r#ReturnsItemCustomfields = Vec<ReturnsItemCustomfieldsItem>;

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
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
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
pub struct ReturnsItemPreferencesItem {
    /// The name of the preferences
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// User preferences
pub type r#ReturnsItemPreferences = Vec<ReturnsItemPreferencesItem>;

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
    /// Username policy is defined in Moodle security config
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
    /// An email address - allow email as root@localhost
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
    /// An arbitrary ID code number perhaps from the institution
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
    /// last access to the course (0 if never)
    #[serde(rename = "lastcourseaccess")]
    pub r#lastcourseaccess: Option<i64>,
    /// User profile description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
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
    /// User custom fields (also known as user profil fields)
    #[serde(rename = "customfields")]
    pub r#customfields: Option<r#ReturnsItemCustomfields>,
    /// user groups
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ReturnsItemGroups>,
    /// user roles
    #[serde(rename = "roles")]
    pub r#roles: Option<r#ReturnsItemRoles>,
    /// User preferences
    #[serde(rename = "preferences")]
    pub r#preferences: Option<r#ReturnsItemPreferences>,
    /// Courses where the user is enrolled - limited by which courses the user is able to see
    #[serde(rename = "enrolledcourses")]
    pub r#enrolledcourses: Option<r#ReturnsItemEnrolledcourses>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_enrol_get_enrolled_users", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_enrol_get_enrolled_users", params).await
}
