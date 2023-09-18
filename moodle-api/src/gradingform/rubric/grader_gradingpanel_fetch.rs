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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGradeCriteriaItemLevelsItem {
    /// ID of level
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// ID of the criterion this matches to
    #[serde(rename = "criterionid")]
    pub r#criterionid: Option<i64>,
    /// What this level is worth
    #[serde(rename = "score")]
    pub r#score: Option<String>,
    /// Definition of the level
    #[serde(rename = "definition")]
    pub r#definition: Option<String>,
    /// Selected flag
    #[serde(rename = "checked")]
    pub r#checked: Option<bool>,
}

pub type r#ReturnsGradeCriteriaItemLevels = Vec<ReturnsGradeCriteriaItemLevelsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGradeCriteriaItem {
    /// ID of the Criteria
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Description of the Criteria
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Any remarks for this criterion for the user being assessed
    #[serde(rename = "remark")]
    pub r#remark: Option<String>,
    #[serde(rename = "levels")]
    pub r#levels: Option<r#ReturnsGradeCriteriaItemLevels>,
}

pub type r#ReturnsGradeCriteria = Vec<ReturnsGradeCriteriaItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGrade {
    /// The id of the current grading instance
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
    /// The mode i.e. evaluate editable
    #[serde(rename = "rubricmode")]
    pub r#rubricmode: Option<String>,
    /// Can the user edit this
    #[serde(rename = "canedit")]
    pub r#canedit: Option<bool>,
    #[serde(rename = "criteria")]
    pub r#criteria: Option<r#ReturnsGradeCriteria>,
    /// The time that the grade was created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Current user grade
    #[serde(rename = "usergrade")]
    pub r#usergrade: Option<String>,
    /// Max possible grade
    #[serde(rename = "maxgrade")]
    pub r#maxgrade: Option<String>,
    /// The assumed grader of this grading instance
    #[serde(rename = "gradedby")]
    pub r#gradedby: Option<String>,
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
        .post("gradingform_rubric_grader_gradingpanel_fetch", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("gradingform_rubric_grader_gradingpanel_fetch", params)
        .await
}
