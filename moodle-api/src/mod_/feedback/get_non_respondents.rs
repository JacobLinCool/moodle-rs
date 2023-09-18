use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Feedback instance id
    #[serde(rename = "feedbackid")]
    pub r#feedbackid: Option<i64>,
    /// Group id, 0 means that the function will determine the user group.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Sort param, must be firstname, lastname or lastaccess (default).
    #[serde(rename = "sort")]
    pub r#sort: Option<String>,
    /// The page of records to return.
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The number of records to return per page.
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
    /// Course where user completes the feedback (for site feedbacks only).
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsersItem {
    /// Course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// The user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// User full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// If the user has started the attempt
    #[serde(rename = "started")]
    pub r#started: Option<bool>,
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
    /// Total number of non respondents
    #[serde(rename = "total")]
    pub r#total: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_feedback_get_non_respondents", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_feedback_get_non_respondents", params)
        .await
}
