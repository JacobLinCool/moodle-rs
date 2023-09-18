use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// course id, 0 for SITE
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSitenotesItem {
    /// id of this note
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// id of the course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// the content text formated
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// content format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// time created (timestamp)
    #[serde(rename = "created")]
    pub r#created: Option<i64>,
    /// time of last modification (timestamp)
    #[serde(rename = "lastmodified")]
    pub r#lastmodified: Option<i64>,
    /// user id of the creator of this note
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// state of the note (i.e. draft, public, site)
    #[serde(rename = "publishstate")]
    pub r#publishstate: Option<String>,
}

/// site notes
pub type r#ReturnsSitenotes = Vec<ReturnsSitenotesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursenotesItem {
    /// id of this note
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// id of the course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// the content text formated
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// content format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// time created (timestamp)
    #[serde(rename = "created")]
    pub r#created: Option<i64>,
    /// time of last modification (timestamp)
    #[serde(rename = "lastmodified")]
    pub r#lastmodified: Option<i64>,
    /// user id of the creator of this note
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// state of the note (i.e. draft, public, site)
    #[serde(rename = "publishstate")]
    pub r#publishstate: Option<String>,
}

/// couse notes
pub type r#ReturnsCoursenotes = Vec<ReturnsCoursenotesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPersonalnotesItem {
    /// id of this note
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// id of the course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// the content text formated
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// content format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// time created (timestamp)
    #[serde(rename = "created")]
    pub r#created: Option<i64>,
    /// time of last modification (timestamp)
    #[serde(rename = "lastmodified")]
    pub r#lastmodified: Option<i64>,
    /// user id of the creator of this note
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// state of the note (i.e. draft, public, site)
    #[serde(rename = "publishstate")]
    pub r#publishstate: Option<String>,
}

/// personal notes
pub type r#ReturnsPersonalnotes = Vec<ReturnsPersonalnotesItem>;

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

/// notes
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// site notes
    #[serde(rename = "sitenotes")]
    pub r#sitenotes: Option<r#ReturnsSitenotes>,
    /// couse notes
    #[serde(rename = "coursenotes")]
    pub r#coursenotes: Option<r#ReturnsCoursenotes>,
    /// personal notes
    #[serde(rename = "personalnotes")]
    pub r#personalnotes: Option<r#ReturnsPersonalnotes>,
    /// Whether the user can manage notes at system level.
    #[serde(rename = "canmanagesystemnotes")]
    pub r#canmanagesystemnotes: Option<bool>,
    /// Whether the user can manage notes at the given course.
    #[serde(rename = "canmanagecoursenotes")]
    pub r#canmanagecoursenotes: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_notes_get_course_notes", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_notes_get_course_notes", params).await
}
