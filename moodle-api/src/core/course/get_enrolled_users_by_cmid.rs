use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of the course module
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// id of the group
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// whether to return only active users or all.
    #[serde(rename = "onlyactive")]
    pub r#onlyactive: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsersItem {
    /// ID of the user
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The location of the users larger image
    #[serde(rename = "profileimage")]
    pub r#profileimage: Option<String>,
    /// The full name of the user
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// The first name(s) of the user
    #[serde(rename = "firstname")]
    pub r#firstname: Option<String>,
    /// The family name of the user
    #[serde(rename = "lastname")]
    pub r#lastname: Option<String>,
}

pub type r#ReturnsUsers = Vec<ReturnsUsersItem>;

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
    #[serde(rename = "users")]
    pub r#users: Option<r#ReturnsUsers>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_course_get_enrolled_users_by_cmid", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_course_get_enrolled_users_by_cmid", params)
        .await
}
