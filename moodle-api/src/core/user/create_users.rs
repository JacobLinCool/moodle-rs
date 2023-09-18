use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsUsersItemCustomfieldsItem {
    /// The name of the custom field
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// User custom fields (also known as user profil fields)
pub type r#ParamsUsersItemCustomfields = Vec<ParamsUsersItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsUsersItemPreferencesItem {
    /// The name of the preference
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the preference
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// User preferences
pub type r#ParamsUsersItemPreferences = Vec<ParamsUsersItemPreferencesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsUsersItem {
    /// True if password should be created and mailed to user.
    #[serde(rename = "createpassword")]
    pub r#createpassword: Option<bool>,
    /// Username policy is defined in Moodle security config.
    #[serde(rename = "username")]
    pub r#username: Option<String>,
    /// Auth plugins include manual, ldap, etc
    #[serde(rename = "auth")]
    pub r#auth: Option<String>,
    /// Plain text password consisting of any characters
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The first name(s) of the user
    #[serde(rename = "firstname")]
    pub r#firstname: Option<String>,
    /// The family name of the user
    #[serde(rename = "lastname")]
    pub r#lastname: Option<String>,
    /// A valid and unique email address
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// Email visibility
    #[serde(rename = "maildisplay")]
    pub r#maildisplay: Option<i64>,
    /// Home city of the user
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// Home country code of the user, such as AU or CZ
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// Timezone code such as Australia/Perth, or 99 for default
    #[serde(rename = "timezone")]
    pub r#timezone: Option<String>,
    /// User profile description, no HTML
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The first name(s) phonetically of the user
    #[serde(rename = "firstnamephonetic")]
    pub r#firstnamephonetic: Option<String>,
    /// The family name phonetically of the user
    #[serde(rename = "lastnamephonetic")]
    pub r#lastnamephonetic: Option<String>,
    /// The middle name of the user
    #[serde(rename = "middlename")]
    pub r#middlename: Option<String>,
    /// The alternate name of the user
    #[serde(rename = "alternatename")]
    pub r#alternatename: Option<String>,
    /// User interests (separated by commas)
    #[serde(rename = "interests")]
    pub r#interests: Option<String>,
    /// An arbitrary ID code number perhaps from the institution
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// institution
    #[serde(rename = "institution")]
    pub r#institution: Option<String>,
    /// department
    #[serde(rename = "department")]
    pub r#department: Option<String>,
    /// Phone 1
    #[serde(rename = "phone1")]
    pub r#phone1: Option<String>,
    /// Phone 2
    #[serde(rename = "phone2")]
    pub r#phone2: Option<String>,
    /// Postal address
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// Language code such as "en", must exist on server
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// Calendar type such as "gregorian", must exist on server
    #[serde(rename = "calendartype")]
    pub r#calendartype: Option<String>,
    /// Theme name such as "standard", must exist on server
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
    /// Mail format code is 0 for plain text, 1 for HTML etc
    #[serde(rename = "mailformat")]
    pub r#mailformat: Option<i64>,
    /// User custom fields (also known as user profil fields)
    #[serde(rename = "customfields")]
    pub r#customfields: Option<r#ParamsUsersItemCustomfields>,
    /// User preferences
    #[serde(rename = "preferences")]
    pub r#preferences: Option<r#ParamsUsersItemPreferences>,
}

pub type r#ParamsUsers = Vec<ParamsUsersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "users")]
    pub r#users: Option<r#ParamsUsers>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// user id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// user name
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_user_create_users", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_user_create_users", params).await
}
