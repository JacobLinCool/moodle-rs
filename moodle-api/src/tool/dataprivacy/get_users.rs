use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The search query
    #[serde(rename = "query")]
    pub r#query: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemExtrafieldsItem {
    /// Name of the extrafield.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Value of the extrafield.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// List of extra fields
pub type r#ReturnsItemExtrafields = Vec<ReturnsItemExtrafieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// ID of the user
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The fullname of the user
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// List of extra fields
    #[serde(rename = "extrafields")]
    pub r#extrafields: Option<r#ReturnsItemExtrafields>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tool_dataprivacy_get_users", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_dataprivacy_get_users", params).await
}
