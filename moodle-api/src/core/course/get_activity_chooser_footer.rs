use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// ID of the course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// ID of the section
    #[serde(rename = "sectionid")]
    pub r#sectionid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Is a footer being return by this request?
    #[serde(rename = "footer")]
    pub r#footer: Option<bool>,
    /// The path to the plugin JS file
    #[serde(rename = "customfooterjs")]
    pub r#customfooterjs: Option<String>,
    /// The prerendered footer
    #[serde(rename = "customfootertemplate")]
    pub r#customfootertemplate: Option<String>,
    /// Either "" or the prerendered carousel page
    #[serde(rename = "customcarouseltemplate")]
    pub r#customcarouseltemplate: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_course_get_activity_chooser_footer", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_course_get_activity_chooser_footer", params)
        .await
}
