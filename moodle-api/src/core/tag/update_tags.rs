use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsTagsItem {
    /// tag id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// tag raw name (may contain capital letters)
    #[serde(rename = "rawname")]
    pub r#rawname: Option<String>,
    /// tag description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// tag description format
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// flag
    #[serde(rename = "flag")]
    pub r#flag: Option<i64>,
    /// (deprecated, use isstandard) whether this flag is standard
    #[serde(rename = "official")]
    pub r#official: Option<i64>,
    /// whether this flag is standard
    #[serde(rename = "isstandard")]
    pub r#isstandard: Option<i64>,
}

pub type r#ParamsTags = Vec<ParamsTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ParamsTags>,
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
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_tag_update_tags", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_tag_update_tags", params).await
}
