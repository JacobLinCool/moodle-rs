use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsDataItem {
    /// The field id.
    #[serde(rename = "fieldid")]
    pub r#fieldid: Option<i64>,
    /// The subfield name (if required).
    #[serde(rename = "subfield")]
    pub r#subfield: Option<String>,
    /// The new contents for the field always JSON encoded.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// The fields data to be updated
pub type r#ParamsData = Vec<ParamsDataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The entry record id.
    #[serde(rename = "entryid")]
    pub r#entryid: Option<i64>,
    /// The fields data to be updated
    #[serde(rename = "data")]
    pub r#data: Option<r#ParamsData>,
}

pub type r#ReturnsGeneralnotifications = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFieldnotificationsItem {
    /// The field name.
    #[serde(rename = "fieldname")]
    pub r#fieldname: Option<String>,
    /// The notification for the field.
    #[serde(rename = "notification")]
    pub r#notification: Option<String>,
}

pub type r#ReturnsFieldnotifications = Vec<ReturnsFieldnotificationsItem>;

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
    /// True if the entry was successfully updated, false other wise.
    #[serde(rename = "updated")]
    pub r#updated: Option<bool>,
    #[serde(rename = "generalnotifications")]
    pub r#generalnotifications: Option<r#ReturnsGeneralnotifications>,
    #[serde(rename = "fieldnotifications")]
    pub r#fieldnotifications: Option<r#ReturnsFieldnotifications>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_data_update_entry", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_data_update_entry", params).await
}
