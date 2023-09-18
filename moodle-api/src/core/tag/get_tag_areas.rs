use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItem {
    /// Area id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Component the area is related to.
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Type of item in the component.
    #[serde(rename = "itemtype")]
    pub r#itemtype: Option<String>,
    /// Whether this area is enabled.
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The tag collection this are belongs to.
    #[serde(rename = "tagcollid")]
    pub r#tagcollid: Option<i64>,
    /// Component callback for processing tags.
    #[serde(rename = "callback")]
    pub r#callback: Option<String>,
    /// Component callback file.
    #[serde(rename = "callbackfile")]
    pub r#callbackfile: Option<String>,
    /// Return whether to display only standard, only non-standard or both tags.
    #[serde(rename = "showstandard")]
    pub r#showstandard: Option<i64>,
    /// Whether the tag area allows tag instances to be created in multiple contexts.
    #[serde(rename = "multiplecontexts")]
    pub r#multiplecontexts: Option<bool>,
    /// Whether the area is locked.
    #[serde(rename = "locked")]
    pub r#locked: Option<bool>,
}

pub type r#ReturnsAreas = Vec<ReturnsAreasItem>;

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
    #[serde(rename = "areas")]
    pub r#areas: Option<r#ReturnsAreas>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_tag_get_tag_areas", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_tag_get_tag_areas", params).await
}
