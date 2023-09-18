use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course Id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Group Id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
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
    /// An email address - allow email as root@localhost
    #[serde(rename = "email")]
    pub r#email: Option<String>,
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
        .post("core_grades_get_enrolled_users_for_selector", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_grades_get_enrolled_users_for_selector", params)
        .await
}
