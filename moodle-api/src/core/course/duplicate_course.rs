use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptionsItem {
    /// The backup option name: "activities" (int) Include course activites (default to 1 that is equal to yes), "blocks" (int) Include course blocks (default to 1 that is equal to yes), "filters" (int) Include course filters  (default to 1 that is equal to yes), "users" (int) Include users (default to 0 that is equal to no), "enrolments" (int) Include enrolment methods (default to 1 - restore only with users), "role_assignments" (int) Include role assignments  (default to 0 that is equal to no), "comments" (int) Include user comments  (default to 0 that is equal to no), "userscompletion" (int) Include user course completion information  (default to 0 that is equal to no), "logs" (int) Include course logs  (default to 0 that is equal to no), "grade_histories" (int) Include histories  (default to 0 that is equal to no)
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// the value for the option 1 (yes) or 0 (no)
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Course duplication options
pub type r#ParamsOptions = Vec<ParamsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// course to duplicate id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// duplicated course full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// duplicated course short name
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// duplicated course category parent
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// duplicated course visible, default to yes
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// Course duplication options
    #[serde(rename = "options")]
    pub r#options: Option<r#ParamsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// course id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// short name
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_course_duplicate_course", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_course_duplicate_course", params).await
}
