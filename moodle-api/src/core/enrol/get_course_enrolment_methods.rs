use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// id of course enrolment instance
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// id of course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// type of enrolment plugin
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// name of enrolment plugin
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// status of enrolment plugin
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// webservice function to get more information
    #[serde(rename = "wsfunction")]
    pub r#wsfunction: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_enrol_get_course_enrolment_methods", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_enrol_get_course_enrolment_methods", params)
        .await
}
