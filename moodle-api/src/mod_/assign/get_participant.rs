use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// assign instance id
    #[serde(rename = "assignid")]
    pub r#assignid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// user id
    #[serde(rename = "embeduser")]
    pub r#embeduser: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserCustomfieldsItem {
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
pub type r#ReturnsUserCustomfields = Vec<ReturnsUserCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserPreferencesItem {
    /// The name of the preferences
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the preference
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Users preferences
pub type r#ReturnsUserPreferences = Vec<ReturnsUserPreferencesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUser {
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
    /// Auth plugins include manual, ldap, etc
    #[serde(rename = "auth")]
    pub r#auth: Option<String>,
    /// Suspend user account, either false to enable user login or true to disable it
    #[serde(rename = "suspended")]
    pub r#suspended: Option<bool>,
    /// Active user: 1 if confirmed, 0 otherwise
    #[serde(rename = "confirmed")]
    pub r#confirmed: Option<bool>,
    /// Language code such as "en", must exist on server
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// Calendar type such as "gregorian", must exist on server
    #[serde(rename = "calendartype")]
    pub r#calendartype: Option<String>,
    /// Theme name such as "standard", must exist on server
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
    /// Timezone code such as Australia/Perth, or 99 for default
    #[serde(rename = "timezone")]
    pub r#timezone: Option<String>,
    /// Mail format code is 0 for plain text, 1 for HTML etc
    #[serde(rename = "mailformat")]
    pub r#mailformat: Option<i64>,
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
    pub r#customfields: Option<r#ReturnsUserCustomfields>,
    /// Users preferences
    #[serde(rename = "preferences")]
    pub r#preferences: Option<r#ReturnsUserPreferences>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// ID of the user
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The fullname of the user
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// have they submitted their assignment
    #[serde(rename = "submitted")]
    pub r#submitted: Option<bool>,
    /// is their submission waiting for grading
    #[serde(rename = "requiregrading")]
    pub r#requiregrading: Option<bool>,
    /// have they been granted an extension
    #[serde(rename = "grantedextension")]
    pub r#grantedextension: Option<bool>,
    /// is blind marking enabled for this assignment
    #[serde(rename = "blindmarking")]
    pub r#blindmarking: Option<bool>,
    /// allowsubmissionsfromdate for the user
    #[serde(rename = "allowsubmissionsfromdate")]
    pub r#allowsubmissionsfromdate: Option<i64>,
    /// duedate for the user
    #[serde(rename = "duedate")]
    pub r#duedate: Option<i64>,
    /// cutoffdate for the user
    #[serde(rename = "cutoffdate")]
    pub r#cutoffdate: Option<i64>,
    /// duedate for the user
    #[serde(rename = "duedatestr")]
    pub r#duedatestr: Option<String>,
    /// for group assignments this is the group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// for group assignments this is the group name
    #[serde(rename = "groupname")]
    pub r#groupname: Option<String>,
    /// The submission status (new, draft, reopened or submitted). Empty when not submitted.
    #[serde(rename = "submissionstatus")]
    pub r#submissionstatus: Option<String>,
    #[serde(rename = "user")]
    pub r#user: Option<ReturnsUser>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_assign_get_participant", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_get_participant", params).await
}
