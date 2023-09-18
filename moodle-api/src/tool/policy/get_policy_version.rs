use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The policy version ID
    #[serde(rename = "versionid")]
    pub r#versionid: Option<i64>,
    /// The id of user on whose behalf the user is viewing the policy
    #[serde(rename = "behalfid")]
    pub r#behalfid: Option<i64>,
}

/// Policy information
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsResultPolicy {
    /// The policy version name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The policy version id
    #[serde(rename = "versionid")]
    pub r#versionid: Option<i64>,
    /// The policy version content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsResult {
    /// Policy information
    #[serde(rename = "policy")]
    pub r#policy: Option<ReturnsResultPolicy>,
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
    #[serde(rename = "result")]
    pub r#result: Option<ReturnsResult>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_policy_get_policy_version", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_policy_get_policy_version", params).await
}
