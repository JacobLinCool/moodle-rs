use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPreferencesItem {
    /// The name of the preference
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the preference
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// Id of the user to set the preference
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

pub type r#ParamsPreferences = Vec<ParamsPreferencesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "preferences")]
    pub r#preferences: Option<r#ParamsPreferences>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSavedItem {
    /// The name of the preference
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The user the preference was set for
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

/// Preferences saved
pub type r#ReturnsSaved = Vec<ReturnsSavedItem>;

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
    /// Preferences saved
    #[serde(rename = "saved")]
    pub r#saved: Option<r#ReturnsSaved>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_user_set_user_preferences", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_user_set_user_preferences", params).await
}
