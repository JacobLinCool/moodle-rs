use serde::{self, Deserialize, Serialize};

/// Array of Note Ids to be retrieved.
pub type r#ParamsNotes = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of Note Ids to be retrieved.
    #[serde(rename = "notes")]
    pub r#notes: Option<r#ParamsNotes>,
}

/// note
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsNotesItem {
    /// id of the note
    #[serde(rename = "noteid")]
    pub r#noteid: Option<i64>,
    /// id of the user the note is about
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// 'personal', 'course' or 'site'
    #[serde(rename = "publishstate")]
    pub r#publishstate: Option<String>,
    /// course id of the note
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// the text of the message - text or HTML
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
}

pub type r#ReturnsNotes = Vec<ReturnsNotesItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
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
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "notes")]
    pub r#notes: Option<r#ReturnsNotes>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_notes_get_notes", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_notes_get_notes", params).await
}
