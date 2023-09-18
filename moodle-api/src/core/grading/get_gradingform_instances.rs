use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// definition id
    #[serde(rename = "definitionid")]
    pub r#definitionid: Option<i64>,
    /// submitted since
    #[serde(rename = "since")]
    pub r#since: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsInstancesItemGuideCriteriaItem {
    /// filling id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// criterion id
    #[serde(rename = "criterionid")]
    pub r#criterionid: Option<i64>,
    /// level id
    #[serde(rename = "levelid")]
    pub r#levelid: Option<i64>,
    /// remark
    #[serde(rename = "remark")]
    pub r#remark: Option<String>,
    /// remark format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "remarkformat")]
    pub r#remarkformat: Option<i64>,
    /// maximum score
    #[serde(rename = "score")]
    pub r#score: Option<f64>,
}

/// filling
pub type r#ReturnsInstancesItemGuideCriteria = Vec<ReturnsInstancesItemGuideCriteriaItem>;

/// items
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsInstancesItemGuide {
    /// filling
    #[serde(rename = "criteria")]
    pub r#criteria: Option<r#ReturnsInstancesItemGuideCriteria>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsInstancesItemRubricCriteriaItem {
    /// filling id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// criterion id
    #[serde(rename = "criterionid")]
    pub r#criterionid: Option<i64>,
    /// level id
    #[serde(rename = "levelid")]
    pub r#levelid: Option<i64>,
    /// remark
    #[serde(rename = "remark")]
    pub r#remark: Option<String>,
    /// remark format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "remarkformat")]
    pub r#remarkformat: Option<i64>,
}

/// filling
pub type r#ReturnsInstancesItemRubricCriteria = Vec<ReturnsInstancesItemRubricCriteriaItem>;

/// items
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsInstancesItemRubric {
    /// filling
    #[serde(rename = "criteria")]
    pub r#criteria: Option<r#ReturnsInstancesItemRubricCriteria>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsInstancesItem {
    /// instance id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// rater id
    #[serde(rename = "raterid")]
    pub r#raterid: Option<i64>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// raw grade
    #[serde(rename = "rawgrade")]
    pub r#rawgrade: Option<String>,
    /// status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// feedback
    #[serde(rename = "feedback")]
    pub r#feedback: Option<String>,
    /// feedback format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "feedbackformat")]
    pub r#feedbackformat: Option<i64>,
    /// modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// items
    #[serde(rename = "guide")]
    pub r#guide: Option<ReturnsInstancesItemGuide>,
    /// items
    #[serde(rename = "rubric")]
    pub r#rubric: Option<ReturnsInstancesItemRubric>,
}

/// list of grading instances
pub type r#ReturnsInstances = Vec<ReturnsInstancesItem>;

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
    /// list of grading instances
    #[serde(rename = "instances")]
    pub r#instances: Option<r#ReturnsInstances>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_grading_get_gradingform_instances", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_grading_get_gradingform_instances", params)
        .await
}
