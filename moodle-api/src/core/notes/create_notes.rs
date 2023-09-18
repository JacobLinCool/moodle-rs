use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsNotesItem {
    /// id of the user the note is about
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// 'personal', 'course' or 'site'
    #[serde(rename = "publishstate")]
    pub r#publishstate: Option<String>,
    /// course id of the note (in Moodle a note can only be created into a course, even for site and personal notes)
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// the text of the message - text or HTML
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// text format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// your own client id for the note. If this id is provided, the fail message id will be returned to you
    #[serde(rename = "clientnoteid")]
    pub r#clientnoteid: Option<String>,
}

pub type r#ParamsNotes = Vec<ParamsNotesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "notes")]
    pub r#notes: Option<r#ParamsNotes>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// your own id for the note
    #[serde(rename = "clientnoteid")]
    pub r#clientnoteid: Option<String>,
    /// ID of the created note when successful, -1 when failed
    #[serde(rename = "noteid")]
    pub r#noteid: Option<i64>,
    /// error message - if failed
    #[serde(rename = "errormessage")]
    pub r#errormessage: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_notes_create_notes", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_notes_create_notes", params).await
}
