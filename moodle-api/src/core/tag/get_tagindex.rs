use serde::{self, Deserialize, Serialize};

/// parameters
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsTagindex {
    /// tag name
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
    /// tag collection id
    #[serde(rename = "tc")]
    pub r#tc: Option<i64>,
    /// tag area id
    #[serde(rename = "ta")]
    pub r#ta: Option<i64>,
    /// exlusive mode for this tag area
    #[serde(rename = "excl")]
    pub r#excl: Option<bool>,
    /// context id where the link was displayed
    #[serde(rename = "from")]
    pub r#from: Option<i64>,
    /// context id where to search for items
    #[serde(rename = "ctx")]
    pub r#ctx: Option<i64>,
    /// search in the context recursive
    #[serde(rename = "rec")]
    pub r#rec: Option<i64>,
    /// page number (0-based)
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// parameters
    #[serde(rename = "tagindex")]
    pub r#tagindex: Option<ParamsTagindex>,
}

/// tag index
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// tag id
    #[serde(rename = "tagid")]
    pub r#tagid: Option<i64>,
    /// tag area id
    #[serde(rename = "ta")]
    pub r#ta: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// itemtype
    #[serde(rename = "itemtype")]
    pub r#itemtype: Option<String>,
    /// URL for the next page
    #[serde(rename = "nextpageurl")]
    pub r#nextpageurl: Option<String>,
    /// URL for the next page
    #[serde(rename = "prevpageurl")]
    pub r#prevpageurl: Option<String>,
    /// URL for exclusive link
    #[serde(rename = "exclusiveurl")]
    pub r#exclusiveurl: Option<String>,
    /// text for exclusive link
    #[serde(rename = "exclusivetext")]
    pub r#exclusivetext: Option<String>,
    /// title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// title
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// whether the content is present
    #[serde(rename = "hascontent")]
    pub r#hascontent: Option<i64>,
    /// name of anchor
    #[serde(rename = "anchor")]
    pub r#anchor: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_tag_get_tagindex", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_tag_get_tagindex", params).await
}
