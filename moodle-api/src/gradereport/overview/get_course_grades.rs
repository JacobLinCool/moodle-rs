use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Get grades for this user (optional, default current)
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGradesItem {
    /// Course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Grade formatted
    #[serde(rename = "grade")]
    pub r#grade: Option<String>,
    /// Raw grade, not formatted
    #[serde(rename = "rawgrade")]
    pub r#rawgrade: Option<String>,
    /// Your rank in the course
    #[serde(rename = "rank")]
    pub r#rank: Option<i64>,
}

pub type r#ReturnsGrades = Vec<ReturnsGradesItem>;

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
    #[serde(rename = "grades")]
    pub r#grades: Option<r#ReturnsGrades>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("gradereport_overview_get_course_grades", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("gradereport_overview_get_course_grades", params)
        .await
}
