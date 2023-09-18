use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Id of the data activity
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// Preset to be imported
    #[serde(rename = "importedpreset")]
    pub r#importedpreset: Option<String>,
}

/// Information to import if everything went fine
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsData {
    /// Whether the importing needs mapping or not
    #[serde(rename = "needsmapping")]
    pub r#needsmapping: Option<bool>,
    /// Name of the applied preset
    #[serde(rename = "presetname")]
    pub r#presetname: Option<String>,
    /// List of field names to create
    #[serde(rename = "fieldstocreate")]
    pub r#fieldstocreate: Option<String>,
    /// List of field names to remove
    #[serde(rename = "fieldstoremove")]
    pub r#fieldstoremove: Option<String>,
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
    /// Information to import if everything went fine
    #[serde(rename = "data")]
    pub r#data: Option<ReturnsData>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_data_get_mapping_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_data_get_mapping_information", params)
        .await
}
