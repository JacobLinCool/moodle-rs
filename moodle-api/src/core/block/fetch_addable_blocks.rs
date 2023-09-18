use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The context ID of the page.
    #[serde(rename = "pagecontextid")]
    pub r#pagecontextid: Option<i64>,
    /// The type of the page.
    #[serde(rename = "pagetype")]
    pub r#pagetype: Option<String>,
    /// The layout of the page.
    #[serde(rename = "pagelayout")]
    pub r#pagelayout: Option<String>,
    /// The subpage identifier
    #[serde(rename = "subpage")]
    pub r#subpage: Option<String>,
    /// Page hash
    #[serde(rename = "pagehash")]
    pub r#pagehash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// The name of the block.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The title of the block.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// If this block type has a form when it is being added then the classname of the form
    #[serde(rename = "blockform")]
    pub r#blockform: Option<String>,
}

/// List of addable blocks in a given page.
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_block_fetch_addable_blocks", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_block_fetch_addable_blocks", params).await
}
