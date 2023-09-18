use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tool proxy id
    #[serde(rename = "toolproxyid")]
    pub r#toolproxyid: Option<i64>,
    /// Orphaned tool types only
    #[serde(rename = "orphanedonly")]
    pub r#orphanedonly: Option<bool>,
    /// How many tool types displayed per page
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
    /// Current offset of tool elements
    #[serde(rename = "offset")]
    pub r#offset: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTypesItemUrls {
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
pub struct ReturnsTypesItemState {
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
pub type r#ReturnsTypesItemCapabilitygroups = Vec<String>;

/// IDs for the LTI instances using this type
pub type r#ReturnsTypesItemInstanceids = Vec<i64>;

/// Tool
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTypesItem {
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
    pub r#urls: Option<ReturnsTypesItemUrls>,
    #[serde(rename = "state")]
    pub r#state: Option<ReturnsTypesItemState>,
    /// Indicate if capabilitygroups is populated
    #[serde(rename = "hascapabilitygroups")]
    pub r#hascapabilitygroups: Option<bool>,
    /// Array of capability groups
    #[serde(rename = "capabilitygroups")]
    pub r#capabilitygroups: Option<r#ReturnsTypesItemCapabilitygroups>,
    /// Tool type course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// IDs for the LTI instances using this type
    #[serde(rename = "instanceids")]
    pub r#instanceids: Option<r#ReturnsTypesItemInstanceids>,
    /// The number of times this tool is being used
    #[serde(rename = "instancecount")]
    pub r#instancecount: Option<i64>,
}

pub type r#ReturnsTypes = Vec<ReturnsTypesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsProxiesItem {
    /// Tool proxy id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Tool proxy name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Tool proxy registration URL
    #[serde(rename = "regurl")]
    pub r#regurl: Option<String>,
    /// Tool proxy state
    #[serde(rename = "state")]
    pub r#state: Option<i64>,
    /// Tool proxy globally unique identifier
    #[serde(rename = "guid")]
    pub r#guid: Option<String>,
    /// Tool proxy shared secret
    #[serde(rename = "secret")]
    pub r#secret: Option<String>,
    /// Tool proxy consumer code
    #[serde(rename = "vendorcode")]
    pub r#vendorcode: Option<String>,
    /// Tool proxy capabilities offered
    #[serde(rename = "capabilityoffered")]
    pub r#capabilityoffered: Option<String>,
    /// Tool proxy services offered
    #[serde(rename = "serviceoffered")]
    pub r#serviceoffered: Option<String>,
    /// Tool proxy
    #[serde(rename = "toolproxy")]
    pub r#toolproxy: Option<String>,
    /// Tool proxy time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Tool proxy modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

pub type r#ReturnsProxies = Vec<ReturnsProxiesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "types")]
    pub r#types: Option<r#ReturnsTypes>,
    #[serde(rename = "proxies")]
    pub r#proxies: Option<r#ReturnsProxies>,
    /// Limit of how many tool types to show
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
    /// Offset of tool types
    #[serde(rename = "offset")]
    pub r#offset: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lti_get_tool_types_and_proxies", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lti_get_tool_types_and_proxies", params)
        .await
}
