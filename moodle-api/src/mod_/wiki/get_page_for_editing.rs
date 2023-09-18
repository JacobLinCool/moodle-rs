use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Page ID to edit.
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// Section page title.
    #[serde(rename = "section")]
    pub r#section: Option<String>,
    /// Just renew lock and not return content.
    #[serde(rename = "lockonly")]
    pub r#lockonly: Option<bool>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesectionWarningsItem {
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
pub type r#ReturnsPagesectionWarnings = Vec<ReturnsPagesectionWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesection {
    /// The contents of the page-section to be edited.
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// Format of the original content of the page.
    #[serde(rename = "contentformat")]
    pub r#contentformat: Option<String>,
    /// Latest version of the page.
    #[serde(rename = "version")]
    pub r#version: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsPagesectionWarnings>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "pagesection")]
    pub r#pagesection: Option<ReturnsPagesection>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_wiki_get_page_for_editing", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_wiki_get_page_for_editing", params).await
}
