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
    /// ID of the user
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Username policy is defined in Moodle security config.
    #[serde(rename = "username")]
    pub r#username: Option<String>,
    /// Auth plugins include manual, ldap, etc
    #[serde(rename = "auth")]
    pub r#auth: Option<String>,
    /// Suspend user account, either false to enable user login or true to disable it
    #[serde(rename = "suspended")]
    pub r#suspended: Option<bool>,
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
    /// The itemid where the new user picture has been uploaded to, 0 to delete
    #[serde(rename = "userpicture")]
    pub r#userpicture: Option<i64>,
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
    /// Institution
    #[serde(rename = "institution")]
    pub r#institution: Option<String>,
    /// Department
    #[serde(rename = "department")]
    pub r#department: Option<String>,
    /// Phone
    #[serde(rename = "phone1")]
    pub r#phone1: Option<String>,
    /// Mobile phone
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
    let json = client.post("core_user_update_users", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_user_update_users", params).await
}
