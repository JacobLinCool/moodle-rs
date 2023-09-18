use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// h5p activity instance id
    #[serde(rename = "h5pactivityid")]
    pub r#h5pactivityid: Option<i64>,
    /// sort by either user id, firstname or lastname (with optional asc/desc)
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<String>,
    /// current page
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// items per page
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
    /// Users whose first name starts with $firstinitial
    #[serde(rename = "firstinitial")]
    pub r#firstinitial: Option<String>,
    /// Users whose last name starts with $lastinitial
    #[serde(rename = "lastinitial")]
    pub r#lastinitial: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsersattemptsItemAttemptsItem {
    /// ID of the context
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// ID of the H5P activity
    #[serde(rename = "h5pactivityid")]
    pub r#h5pactivityid: Option<i64>,
    /// ID of the user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Attempt creation
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Attempt modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Attempt number
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
    /// Attempt score value
    #[serde(rename = "rawscore")]
    pub r#rawscore: Option<i64>,
    /// Attempt max score
    #[serde(rename = "maxscore")]
    pub r#maxscore: Option<i64>,
    /// Attempt duration in seconds
    #[serde(rename = "duration")]
    pub r#duration: Option<i64>,
    /// Attempt completion
    #[serde(rename = "completion")]
    pub r#completion: Option<i64>,
    /// Attempt success
    #[serde(rename = "success")]
    pub r#success: Option<i64>,
    /// Attempt scaled
    #[serde(rename = "scaled")]
    pub r#scaled: Option<f64>,
}

/// The complete attempts list
pub type r#ReturnsUsersattemptsItemAttempts = Vec<ReturnsUsersattemptsItemAttemptsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsersattemptsItemScoredAttemptsItem {
    /// ID of the context
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// ID of the H5P activity
    #[serde(rename = "h5pactivityid")]
    pub r#h5pactivityid: Option<i64>,
    /// ID of the user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Attempt creation
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Attempt modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Attempt number
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
    /// Attempt score value
    #[serde(rename = "rawscore")]
    pub r#rawscore: Option<i64>,
    /// Attempt max score
    #[serde(rename = "maxscore")]
    pub r#maxscore: Option<i64>,
    /// Attempt duration in seconds
    #[serde(rename = "duration")]
    pub r#duration: Option<i64>,
    /// Attempt completion
    #[serde(rename = "completion")]
    pub r#completion: Option<i64>,
    /// Attempt success
    #[serde(rename = "success")]
    pub r#success: Option<i64>,
    /// Attempt scaled
    #[serde(rename = "scaled")]
    pub r#scaled: Option<f64>,
}

/// List of the grading attempts
pub type r#ReturnsUsersattemptsItemScoredAttempts = Vec<ReturnsUsersattemptsItemScoredAttemptsItem>;

/// Attempts used to grade the activity
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsersattemptsItemScored {
    /// Scored attempts title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Grading method
    #[serde(rename = "grademethod")]
    pub r#grademethod: Option<String>,
    /// List of the grading attempts
    #[serde(rename = "attempts")]
    pub r#attempts: Option<r#ReturnsUsersattemptsItemScoredAttempts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsersattemptsItem {
    /// The user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The complete attempts list
    #[serde(rename = "attempts")]
    pub r#attempts: Option<r#ReturnsUsersattemptsItemAttempts>,
    /// Attempts used to grade the activity
    #[serde(rename = "scored")]
    pub r#scored: Option<ReturnsUsersattemptsItemScored>,
}

/// The complete users attempts list
pub type r#ReturnsUsersattempts = Vec<ReturnsUsersattemptsItem>;

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

/// Activity attempts data
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Activity course module ID
    #[serde(rename = "activityid")]
    pub r#activityid: Option<i64>,
    /// The complete users attempts list
    #[serde(rename = "usersattempts")]
    pub r#usersattempts: Option<r#ReturnsUsersattempts>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_h5pactivity_get_user_attempts", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_h5pactivity_get_user_attempts", params)
        .await
}
