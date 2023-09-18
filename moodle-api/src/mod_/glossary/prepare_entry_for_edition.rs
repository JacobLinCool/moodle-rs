use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Glossary entry id to update
    #[serde(rename = "entryid")]
    pub r#entryid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItemOptionsItem {
    /// Name of option.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Value of option.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Draft file area options.
pub type r#ReturnsAreasItemOptions = Vec<ReturnsAreasItemOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItem {
    /// File area name.
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// Draft file area options.
    #[serde(rename = "options")]
    pub r#options: Option<r#ReturnsAreasItemOptions>,
}

/// File areas including options
pub type r#ReturnsAreas = Vec<ReturnsAreasItem>;

pub type r#ReturnsAliases = Vec<String>;

pub type r#ReturnsCategories = Vec<i64>;

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
    /// Draft item id for the text editor.
    #[serde(rename = "inlineattachmentsid")]
    pub r#inlineattachmentsid: Option<i64>,
    /// Draft item id for the file manager.
    #[serde(rename = "attachmentsid")]
    pub r#attachmentsid: Option<i64>,
    /// File areas including options
    #[serde(rename = "areas")]
    pub r#areas: Option<r#ReturnsAreas>,
    #[serde(rename = "aliases")]
    pub r#aliases: Option<r#ReturnsAliases>,
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
    let json = client
        .post("mod_glossary_prepare_entry_for_edition", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_glossary_prepare_entry_for_edition", params)
        .await
}
