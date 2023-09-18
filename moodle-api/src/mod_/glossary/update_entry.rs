use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptionsItem {
    /// The allowed keys (value format) are: inlineattachmentsid (int); the draft file area id for inline attachments attachmentsid (int); the draft file area id for attachments categories (comma separated int); comma separated category ids aliases (comma separated str); comma separated aliases usedynalink (bool); whether the entry should be automatically linked. casesensitive (bool); whether the entry is case sensitive. fullmatch (bool); whether to match whole words only.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// the value of the option (validated inside the function)
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Optional settings
pub type r#ParamsOptions = Vec<ParamsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Glossary entry id to update
    #[serde(rename = "entryid")]
    pub r#entryid: Option<i64>,
    /// Glossary concept
    #[serde(rename = "concept")]
    pub r#concept: Option<String>,
    /// Glossary concept definition
    #[serde(rename = "definition")]
    pub r#definition: Option<String>,
    /// definition format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "definitionformat")]
    pub r#definitionformat: Option<i64>,
    /// Optional settings
    #[serde(rename = "options")]
    pub r#options: Option<r#ParamsOptions>,
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
    /// The update result
    #[serde(rename = "result")]
    pub r#result: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_glossary_update_entry", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_glossary_update_entry", params).await
}
