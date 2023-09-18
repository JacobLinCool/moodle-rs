use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tool type id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Tool type name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Tool type description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Tool type state
    #[serde(rename = "state")]
    pub r#state: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUrls {
    /// Tool type icon URL
    #[serde(rename = "icon")]
    pub r#icon: Option<String>,
    /// Tool type edit URL
    #[serde(rename = "edit")]
    pub r#edit: Option<String>,
    /// Tool type edit URL
    #[serde(rename = "course")]
    pub r#course: Option<String>,
    /// Public Keyset URL
    #[serde(rename = "publickeyset")]
    pub r#publickeyset: Option<String>,
    /// Access Token URL
    #[serde(rename = "accesstoken")]
    pub r#accesstoken: Option<String>,
    /// Authorisation Request URL
    #[serde(rename = "authrequest")]
    pub r#authrequest: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsState {
    /// Tool type state name string
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// Is the state pending
    #[serde(rename = "pending")]
    pub r#pending: Option<bool>,
    /// Is the state configured
    #[serde(rename = "configured")]
    pub r#configured: Option<bool>,
    /// Is the state rejected
    #[serde(rename = "rejected")]
    pub r#rejected: Option<bool>,
    /// Is the state unknown
    #[serde(rename = "unknown")]
    pub r#unknown: Option<bool>,
}

/// Array of capability groups
pub type r#ReturnsCapabilitygroups = Vec<String>;

/// IDs for the LTI instances using this type
pub type r#ReturnsInstanceids = Vec<i64>;

/// Tool
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Tool type id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Tool type name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Tool type description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Platform ID
    #[serde(rename = "platformid")]
    pub r#platformid: Option<String>,
    /// Client ID
    #[serde(rename = "clientid")]
    pub r#clientid: Option<String>,
    /// Deployment ID
    #[serde(rename = "deploymentid")]
    pub r#deploymentid: Option<i64>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsUrls>,
    #[serde(rename = "state")]
    pub r#state: Option<ReturnsState>,
    /// Indicate if capabilitygroups is populated
    #[serde(rename = "hascapabilitygroups")]
    pub r#hascapabilitygroups: Option<bool>,
    /// Array of capability groups
    #[serde(rename = "capabilitygroups")]
    pub r#capabilitygroups: Option<r#ReturnsCapabilitygroups>,
    /// Tool type course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// IDs for the LTI instances using this type
    #[serde(rename = "instanceids")]
    pub r#instanceids: Option<r#ReturnsInstanceids>,
    /// The number of times this tool is being used
    #[serde(rename = "instancecount")]
    pub r#instancecount: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lti_update_tool_type", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lti_update_tool_type", params).await
}
