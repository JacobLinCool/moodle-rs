use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The context id to expand
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The element we are interested on
    #[serde(rename = "element")]
    pub r#element: Option<String>,
}

pub type r#ReturnsBranchesItemBranchesItemBranches = Vec<String>;

/// Node structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBranchesItemBranchesItem {
    /// The node text
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// The contextid this node expands
    #[serde(rename = "expandcontextid")]
    pub r#expandcontextid: Option<i64>,
    /// What element is this node expanded to
    #[serde(rename = "expandelement")]
    pub r#expandelement: Option<String>,
    /// The node contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The node contextlevel
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<i64>,
    /// Is it expanded
    #[serde(rename = "expanded")]
    pub r#expanded: Option<i64>,
    #[serde(rename = "branches")]
    pub r#branches: Option<r#ReturnsBranchesItemBranchesItemBranches>,
}

/// Children node structure
pub type r#ReturnsBranchesItemBranches = Vec<ReturnsBranchesItemBranchesItem>;

/// Node structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsBranchesItem {
    /// The node text
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// The contextid this node expands
    #[serde(rename = "expandcontextid")]
    pub r#expandcontextid: Option<i64>,
    /// What element is this node expanded to
    #[serde(rename = "expandelement")]
    pub r#expandelement: Option<String>,
    /// The node contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The node contextlevel
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<i64>,
    /// Is it expanded
    #[serde(rename = "expanded")]
    pub r#expanded: Option<i64>,
    /// Children node structure
    #[serde(rename = "branches")]
    pub r#branches: Option<r#ReturnsBranchesItemBranches>,
}

pub type r#ReturnsBranches = Vec<ReturnsBranchesItem>;

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
    #[serde(rename = "branches")]
    pub r#branches: Option<r#ReturnsBranches>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_dataprivacy_tree_extra_branches", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_dataprivacy_tree_extra_branches", params)
        .await
}
