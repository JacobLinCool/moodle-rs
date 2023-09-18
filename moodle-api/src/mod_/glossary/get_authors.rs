use serde::{self, Deserialize, Serialize};

/// An array of options
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptions {
    /// When false, includes self even if all of their entries require approval. When true, also includes authors only having entries pending approval.
    #[serde(rename = "includenotapproved")]
    pub r#includenotapproved: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Glossary entry ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Start returning records from here
    #[serde(rename = "from")]
    pub r#from: Option<i64>,
    /// Number of records to return
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
    /// An array of options
    #[serde(rename = "options")]
    pub r#options: Option<ParamsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAuthorsItem {
    /// The user ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// The picture URL
    #[serde(rename = "pictureurl")]
    pub r#pictureurl: Option<String>,
}

pub type r#ReturnsAuthors = Vec<ReturnsAuthorsItem>;

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
    #[serde(rename = "authors")]
    pub r#authors: Option<r#ReturnsAuthors>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_glossary_get_authors", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_glossary_get_authors", params).await
}
