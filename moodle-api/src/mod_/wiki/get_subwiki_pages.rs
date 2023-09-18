use serde::{self, Deserialize, Serialize};

/// Options
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptions {
    /// Field to sort by (id, title, ...).
    #[serde(rename = "sortby")]
    pub r#sortby: Option<String>,
    /// Sort direction: ASC or DESC.
    #[serde(rename = "sortdirection")]
    pub r#sortdirection: Option<String>,
    /// Include each page contents or just the contents size.
    #[serde(rename = "includecontent")]
    pub r#includecontent: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Wiki instance ID.
    #[serde(rename = "wikiid")]
    pub r#wikiid: Option<i64>,
    /// Subwiki's group ID, -1 means current group. It will be ignored if the wiki doesn't use groups.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Subwiki's user ID, 0 means current user. It will be ignored in collaborative wikis.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Options
    #[serde(rename = "options")]
    pub r#options: Option<ParamsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesItemTagsItem {
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
pub type r#ReturnsPagesItemTags = Vec<ReturnsPagesItemTagsItem>;

/// Pages
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPagesItem {
    /// Page ID.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Page's subwiki ID.
    #[serde(rename = "subwikiid")]
    pub r#subwikiid: Option<i64>,
    /// Page title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Time of creation.
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time of last modification.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Time of last renderization.
    #[serde(rename = "timerendered")]
    pub r#timerendered: Option<i64>,
    /// ID of the user that last modified the page.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Number of times the page has been viewed.
    #[serde(rename = "pageviews")]
    pub r#pageviews: Option<i64>,
    /// 1 if readonly, 0 otherwise.
    #[serde(rename = "readonly")]
    pub r#readonly: Option<i64>,
    /// True if user can edit the page.
    #[serde(rename = "caneditpage")]
    pub r#caneditpage: Option<bool>,
    /// True if it's the first page.
    #[serde(rename = "firstpage")]
    pub r#firstpage: Option<bool>,
    /// Page contents.
    #[serde(rename = "cachedcontent")]
    pub r#cachedcontent: Option<String>,
    /// cachedcontent format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "contentformat")]
    pub r#contentformat: Option<i64>,
    /// Size of page contents in bytes (doesn't include size of attached files).
    #[serde(rename = "contentsize")]
    pub r#contentsize: Option<i64>,
    /// Tags
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsPagesItemTags>,
}

pub type r#ReturnsPages = Vec<ReturnsPagesItem>;

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
    #[serde(rename = "pages")]
    pub r#pages: Option<r#ReturnsPages>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_wiki_get_subwiki_pages", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_wiki_get_subwiki_pages", params).await
}
