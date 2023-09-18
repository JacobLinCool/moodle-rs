use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Wiki instance ID.
    #[serde(rename = "wikiid")]
    pub r#wikiid: Option<i64>,
}

/// Subwikis
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSubwikisItem {
    /// Subwiki ID.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Wiki ID.
    #[serde(rename = "wikiid")]
    pub r#wikiid: Option<i64>,
    /// Group ID.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<String>,
    /// User ID.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// True if user can edit the subwiki.
    #[serde(rename = "canedit")]
    pub r#canedit: Option<bool>,
}

pub type r#ReturnsSubwikis = Vec<ReturnsSubwikisItem>;

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
    #[serde(rename = "subwikis")]
    pub r#subwikis: Option<r#ReturnsSubwikis>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_wiki_get_subwikis", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_wiki_get_subwikis", params).await
}
