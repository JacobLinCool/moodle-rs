use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// New page title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Page contents.
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// Page contents format. If an invalid format is provided, default wiki format is used.
    #[serde(rename = "contentformat")]
    pub r#contentformat: Option<String>,
    /// Page's subwiki ID.
    #[serde(rename = "subwikiid")]
    pub r#subwikiid: Option<i64>,
    /// Page's wiki ID. Used if subwiki does not exists.
    #[serde(rename = "wikiid")]
    pub r#wikiid: Option<i64>,
    /// Subwiki's user ID. Used if subwiki does not exists.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Subwiki's group ID. Used if subwiki does not exists.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
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
    /// New page id.
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_wiki_new_page", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_wiki_new_page", params).await
}
