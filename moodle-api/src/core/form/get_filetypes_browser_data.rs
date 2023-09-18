use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Limit the browser to the given groups and extensions
    #[serde(rename = "onlytypes")]
    pub r#onlytypes: Option<String>,
    /// Allows to select All file types, does not apply with onlytypes are set.
    #[serde(rename = "allowall")]
    pub r#allowall: Option<bool>,
    /// Current types that should be selected.
    #[serde(rename = "current")]
    pub r#current: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGroupsItemTypesItem {
    /// The file type identifier
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The file type name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Should it be marked as selected
    #[serde(rename = "selected")]
    pub r#selected: Option<bool>,
    /// The file extension associated with the file type
    #[serde(rename = "ext")]
    pub r#ext: Option<String>,
}

/// List of file types in the group
pub type r#ReturnsGroupsItemTypes = Vec<ReturnsGroupsItemTypesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGroupsItem {
    /// The file type group identifier
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The file type group name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Can it be marked as selected
    #[serde(rename = "selectable")]
    pub r#selectable: Option<bool>,
    /// Should it be marked as selected
    #[serde(rename = "selected")]
    pub r#selected: Option<bool>,
    /// The list of file extensions associated with the group
    #[serde(rename = "ext")]
    pub r#ext: Option<String>,
    /// Should the group start as expanded or collapsed
    #[serde(rename = "expanded")]
    pub r#expanded: Option<bool>,
    /// List of file types in the group
    #[serde(rename = "types")]
    pub r#types: Option<r#ReturnsGroupsItemTypes>,
}

/// List of file type groups
pub type r#ReturnsGroups = Vec<ReturnsGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// List of file type groups
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ReturnsGroups>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_form_get_filetypes_browser_data", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_form_get_filetypes_browser_data", params)
        .await
}
