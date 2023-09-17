use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The search query
    #[serde(rename = "query")]
    pub r#query: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsListItemExtrafieldsItem {
    /// Name of the extrafield.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Value of the extrafield.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// List of extra fields
pub type r#ReturnsListItemExtrafields = Vec<ReturnsListItemExtrafieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsListItem {
    /// ID of the user
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The fullname of the user
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// List of extra fields
    #[serde(rename = "extrafields")]
    pub r#extrafields: ReturnsListItemExtrafields,
}

pub type r#ReturnsList = Vec<ReturnsListItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "list")]
    pub r#list: ReturnsList,
    /// Configured maximum users per page.
    #[serde(rename = "maxusersperpage")]
    pub r#maxusersperpage: Option<i64>,
    /// Were there more records than maxusersperpage found?
    #[serde(rename = "overflow")]
    pub r#overflow: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_user_search_identity", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
