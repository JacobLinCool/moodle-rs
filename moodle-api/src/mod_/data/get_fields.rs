use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Database instance id.
    #[serde(rename = "databaseid")]
    pub r#databaseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFieldsItem {
    /// Field id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The field type of the content.
    #[serde(rename = "dataid")]
    pub r#dataid: Option<i64>,
    /// The field type.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The field name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The field description.
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Whether is a field required or not.
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// Field parameters
    #[serde(rename = "param1")]
    pub r#param1: Option<String>,
    /// Field parameters
    #[serde(rename = "param2")]
    pub r#param2: Option<String>,
    /// Field parameters
    #[serde(rename = "param3")]
    pub r#param3: Option<String>,
    /// Field parameters
    #[serde(rename = "param4")]
    pub r#param4: Option<String>,
    /// Field parameters
    #[serde(rename = "param5")]
    pub r#param5: Option<String>,
    /// Field parameters
    #[serde(rename = "param6")]
    pub r#param6: Option<String>,
    /// Field parameters
    #[serde(rename = "param7")]
    pub r#param7: Option<String>,
    /// Field parameters
    #[serde(rename = "param8")]
    pub r#param8: Option<String>,
    /// Field parameters
    #[serde(rename = "param9")]
    pub r#param9: Option<String>,
    /// Field parameters
    #[serde(rename = "param10")]
    pub r#param10: Option<String>,
}

pub type r#ReturnsFields = Vec<ReturnsFieldsItem>;

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
    #[serde(rename = "fields")]
    pub r#fields: Option<r#ReturnsFields>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_data_get_fields", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_data_get_fields", params).await
}
