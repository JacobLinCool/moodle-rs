use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// h5p activity instance id
    #[serde(rename = "h5pactivityid")]
    pub r#h5pactivityid: Option<i64>,
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
    /// Whether the user has the capability mod/h5pactivity:view allowed.
    #[serde(rename = "canview")]
    pub r#canview: Option<bool>,
    /// Whether the user has the capability mod/h5pactivity:addinstance allowed.
    #[serde(rename = "canaddinstance")]
    pub r#canaddinstance: Option<bool>,
    /// Whether the user has the capability mod/h5pactivity:submit allowed.
    #[serde(rename = "cansubmit")]
    pub r#cansubmit: Option<bool>,
    /// Whether the user has the capability mod/h5pactivity:reviewattempts allowed.
    #[serde(rename = "canreviewattempts")]
    pub r#canreviewattempts: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_h5pactivity_get_h5pactivity_access_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_h5pactivity_get_h5pactivity_access_information", params)
        .await
}
