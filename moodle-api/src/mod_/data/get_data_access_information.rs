use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Database instance id.
    #[serde(rename = "databaseid")]
    pub r#databaseid: Option<i64>,
    /// Group id, 0 means that the function will determine the user group.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
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
    /// User current group id (calculated)
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Whether the user can add entries or not.
    #[serde(rename = "canaddentry")]
    pub r#canaddentry: Option<bool>,
    /// Whether the user can manage entries or not.
    #[serde(rename = "canmanageentries")]
    pub r#canmanageentries: Option<bool>,
    /// Whether the user can approve entries or not.
    #[serde(rename = "canapprove")]
    pub r#canapprove: Option<bool>,
    /// Whether the database is available or not by time restrictions.
    #[serde(rename = "timeavailable")]
    pub r#timeavailable: Option<bool>,
    /// Whether the database is in read mode only.
    #[serde(rename = "inreadonlyperiod")]
    pub r#inreadonlyperiod: Option<bool>,
    /// The number of entries the current user added.
    #[serde(rename = "numentries")]
    pub r#numentries: Option<i64>,
    /// The number of entries left to complete the activity.
    #[serde(rename = "entrieslefttoadd")]
    pub r#entrieslefttoadd: Option<i64>,
    /// The number of entries left to view other users entries.
    #[serde(rename = "entrieslefttoview")]
    pub r#entrieslefttoview: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_data_get_data_access_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_data_get_data_access_information", params)
        .await
}
