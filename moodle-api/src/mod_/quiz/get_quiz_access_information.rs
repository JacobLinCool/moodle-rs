use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// quiz instance id
    #[serde(rename = "quizid")]
    pub r#quizid: Option<i64>,
}

/// list of rules
pub type r#ReturnsAccessrules = Vec<String>;

/// list of active rules
pub type r#ReturnsActiverulenames = Vec<String>;

/// list of reasons
pub type r#ReturnsPreventaccessreasons = Vec<String>;

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
    /// Whether the user can do the quiz or not.
    #[serde(rename = "canattempt")]
    pub r#canattempt: Option<bool>,
    /// Whether the user can edit the quiz settings or not.
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// Whether the user can preview the quiz or not.
    #[serde(rename = "canpreview")]
    pub r#canpreview: Option<bool>,
    /// Whether the users can review their previous attempts or not.
    #[serde(rename = "canreviewmyattempts")]
    pub r#canreviewmyattempts: Option<bool>,
    /// Whether the user can view the quiz reports or not.
    #[serde(rename = "canviewreports")]
    pub r#canviewreports: Option<bool>,
    /// list of rules
    #[serde(rename = "accessrules")]
    pub r#accessrules: Option<r#ReturnsAccessrules>,
    /// list of active rules
    #[serde(rename = "activerulenames")]
    pub r#activerulenames: Option<r#ReturnsActiverulenames>,
    /// list of reasons
    #[serde(rename = "preventaccessreasons")]
    pub r#preventaccessreasons: Option<r#ReturnsPreventaccessreasons>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_quiz_get_quiz_access_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_quiz_get_quiz_access_information", params)
        .await
}
