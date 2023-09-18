use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tag collection id.
    #[serde(rename = "tagcollid")]
    pub r#tagcollid: Option<i64>,
    /// Whether to return only standard tags.
    #[serde(rename = "isstandard")]
    pub r#isstandard: Option<bool>,
    /// Maximum number of tags to retrieve.
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
    /// Sort order for display (id, name, rawname, count, flag, isstandard, tagcollid).
    #[serde(rename = "sort")]
    pub r#sort: Option<String>,
    /// Search string.
    #[serde(rename = "search")]
    pub r#search: Option<String>,
    /// Context id where this tag cloud is displayed.
    #[serde(rename = "fromctx")]
    pub r#fromctx: Option<i64>,
    /// Only retrieve tag instances in this context.
    #[serde(rename = "ctx")]
    pub r#ctx: Option<i64>,
    /// Retrieve tag instances in the $ctx context and it's children.
    #[serde(rename = "rec")]
    pub r#rec: Option<i64>,
}

/// Tags.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTagsItem {
    /// Tag name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// URL to view the tag index.
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// Whether the tag is flagged as inappropriate.
    #[serde(rename = "flag")]
    pub r#flag: Option<bool>,
    /// Whether is a standard tag or not.
    #[serde(rename = "isstandard")]
    pub r#isstandard: Option<bool>,
    /// Number of tag instances.
    #[serde(rename = "count")]
    pub r#count: Option<i64>,
    /// Proportional size to display the tag.
    #[serde(rename = "size")]
    pub r#size: Option<i64>,
}

pub type r#ReturnsTags = Vec<ReturnsTagsItem>;

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
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsTags>,
    /// Number of tags returned.
    #[serde(rename = "tagscount")]
    pub r#tagscount: Option<i64>,
    /// Total count of tags.
    #[serde(rename = "totalcount")]
    pub r#totalcount: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_tag_get_tag_cloud", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_tag_get_tag_cloud", params).await
}
