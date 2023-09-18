use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCollectionsItem {
    /// Collection id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Collection name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether is the default collection.
    #[serde(rename = "isdefault")]
    pub r#isdefault: Option<bool>,
    /// Component the collection is related to.
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Collection ordering in the list.
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// Whether the tag collection is searchable.
    #[serde(rename = "searchable")]
    pub r#searchable: Option<bool>,
    /// Custom URL for the tag page instead of /tag/index.php.
    #[serde(rename = "customurl")]
    pub r#customurl: Option<String>,
}

pub type r#ReturnsCollections = Vec<ReturnsCollectionsItem>;

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
    #[serde(rename = "collections")]
    pub r#collections: Option<r#ReturnsCollections>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_tag_get_tag_collections", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_tag_get_tag_collections", params).await
}
