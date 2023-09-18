use serde::{self, Deserialize, Serialize};

/// Array of Note Ids to be deleted.
pub type r#ParamsNotes = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of Note Ids to be deleted.
    #[serde(rename = "notes")]
    pub r#notes: Option<r#ParamsNotes>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// item is always 'note'
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// When errorcode is savedfailed the note could not be modified.When errorcode is badparam, an incorrect parameter was provided.When errorcode is badid, the note does not exist
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// errorcode can be badparam (incorrect parameter), savedfailed (could not be modified), or badid (note does not exist)
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_notes_delete_notes", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_notes_delete_notes", params).await
}
