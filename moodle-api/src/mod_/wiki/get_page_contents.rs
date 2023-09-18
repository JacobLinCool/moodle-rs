use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Page ID.
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPageTagsItem {
    /// Tag id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Tag name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The raw, unnormalised name for the tag as entered by users.
    #[serde(rename = "rawname")]
    pub r#rawname: Option<String>,
    /// Whether this tag is standard.
    #[serde(rename = "isstandard")]
    pub r#isstandard: Option<bool>,
    /// Tag collection id.
    #[serde(rename = "tagcollid")]
    pub r#tagcollid: Option<i64>,
    /// Tag instance id.
    #[serde(rename = "taginstanceid")]
    pub r#taginstanceid: Option<i64>,
    /// Context the tag instance belongs to.
    #[serde(rename = "taginstancecontextid")]
    pub r#taginstancecontextid: Option<i64>,
    /// Id of the record tagged.
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// Tag ordering.
    #[serde(rename = "ordering")]
    pub r#ordering: Option<i64>,
    /// Whether the tag is flagged as inappropriate.
    #[serde(rename = "flag")]
    pub r#flag: Option<i64>,
}

/// Tags
pub type r#ReturnsPageTags = Vec<ReturnsPageTagsItem>;

/// Page
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPage {
    /// Page ID.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Page's wiki ID.
    #[serde(rename = "wikiid")]
    pub r#wikiid: Option<i64>,
    /// Page's subwiki ID.
    #[serde(rename = "subwikiid")]
    pub r#subwikiid: Option<i64>,
    /// Page's group ID.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Page's user ID.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Page title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Page contents.
    #[serde(rename = "cachedcontent")]
    pub r#cachedcontent: Option<String>,
    /// cachedcontent format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "contentformat")]
    pub r#contentformat: Option<i64>,
    /// True if user can edit the page.
    #[serde(rename = "caneditpage")]
    pub r#caneditpage: Option<bool>,
    /// Latest version of the page.
    #[serde(rename = "version")]
    pub r#version: Option<i64>,
    /// Tags
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsPageTags>,
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
    /// Page
    #[serde(rename = "page")]
    pub r#page: Option<ReturnsPage>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_wiki_get_page_contents", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_wiki_get_page_contents", params).await
}
