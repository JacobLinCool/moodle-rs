use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The glossary ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Start returning records from here
    #[serde(rename = "from")]
    pub r#from: Option<i64>,
    /// Number of records to return
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCategoriesItem {
    /// The category ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The glossary ID
    #[serde(rename = "glossaryid")]
    pub r#glossaryid: Option<i64>,
    /// The name of the category
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether the category is automatically linked
    #[serde(rename = "usedynalink")]
    pub r#usedynalink: Option<bool>,
}

pub type r#ReturnsCategories = Vec<ReturnsCategoriesItem>;

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
    /// The total number of records.
    #[serde(rename = "count")]
    pub r#count: Option<i64>,
    #[serde(rename = "categories")]
    pub r#categories: Option<r#ReturnsCategories>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_glossary_get_categories", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_glossary_get_categories", params).await
}
