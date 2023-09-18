use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The name of the component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// The ID of the context being graded
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The grade item itemname being graded
    #[serde(rename = "itemname")]
    pub r#itemname: Option<String>,
    /// The ID of the user show
    #[serde(rename = "gradeduserid")]
    pub r#gradeduserid: Option<i64>,
    /// Wheteher to notify the user or not
    #[serde(rename = "notifyuser")]
    pub r#notifyuser: Option<bool>,
    /// The serialised form data representing the grade
    #[serde(rename = "formdata")]
    pub r#formdata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGradeOptionsItem {
    /// The grade value
    #[serde(rename = "value")]
    pub r#value: Option<f64>,
    /// The description fo the option
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Whether this item is currently selected
    #[serde(rename = "selected")]
    pub r#selected: Option<bool>,
}

/// The description of the grade option
pub type r#ReturnsGradeOptions = Vec<ReturnsGradeOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGrade {
    /// The description of the grade option
    #[serde(rename = "options")]
    pub r#options: Option<r#ReturnsGradeOptions>,
    /// Current user grade
    #[serde(rename = "usergrade")]
    pub r#usergrade: Option<String>,
    /// Max possible grade
    #[serde(rename = "maxgrade")]
    pub r#maxgrade: Option<String>,
    /// The assumed grader of this grading instance
    #[serde(rename = "gradedby")]
    pub r#gradedby: Option<String>,
    /// The time that the grade was created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// The time that the grade was last updated
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
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
    /// The template to use when rendering this data
    #[serde(rename = "templatename")]
    pub r#templatename: Option<String>,
    /// Does the user have a grade?
    #[serde(rename = "hasgrade")]
    pub r#hasgrade: Option<bool>,
    #[serde(rename = "grade")]
    pub r#grade: Option<ReturnsGrade>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_grades_grader_gradingpanel_scale_store", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_grades_grader_gradingpanel_scale_store", params)
        .await
}
