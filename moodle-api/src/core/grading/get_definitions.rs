use serde::{self, Deserialize, Serialize};

/// 1 or more course module ids
pub type r#ParamsCmids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// 1 or more course module ids
    #[serde(rename = "cmids")]
    pub r#cmids: Option<r#ParamsCmids>,
    /// area name
    #[serde(rename = "areaname")]
    pub r#areaname: Option<String>,
    /// Only the active method
    #[serde(rename = "activeonly")]
    pub r#activeonly: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItemDefinitionsItemGuideGuideCriteriaItem {
    /// criterion id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// description
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// markers description
    #[serde(rename = "descriptionmarkers")]
    pub r#descriptionmarkers: Option<String>,
    /// descriptionmarkers format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionmarkersformat")]
    pub r#descriptionmarkersformat: Option<i64>,
    /// maximum score
    #[serde(rename = "maxscore")]
    pub r#maxscore: Option<f64>,
}

pub type r#ReturnsAreasItemDefinitionsItemGuideGuideCriteria =
    Vec<ReturnsAreasItemDefinitionsItemGuideGuideCriteriaItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItemDefinitionsItemGuideGuideCommentsItem {
    /// criterion id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
}

/// comments
pub type r#ReturnsAreasItemDefinitionsItemGuideGuideComments =
    Vec<ReturnsAreasItemDefinitionsItemGuideGuideCommentsItem>;

/// items
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItemDefinitionsItemGuide {
    #[serde(rename = "guide_criteria")]
    pub r#guide_criteria: Option<r#ReturnsAreasItemDefinitionsItemGuideGuideCriteria>,
    /// comments
    #[serde(rename = "guide_comments")]
    pub r#guide_comments: Option<r#ReturnsAreasItemDefinitionsItemGuideGuideComments>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItemDefinitionsItemRubricRubricCriteriaItemLevelsItem {
    /// level id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// score
    #[serde(rename = "score")]
    pub r#score: Option<f64>,
    /// definition
    #[serde(rename = "definition")]
    pub r#definition: Option<String>,
    /// definition format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "definitionformat")]
    pub r#definitionformat: Option<i64>,
}

/// levels
pub type r#ReturnsAreasItemDefinitionsItemRubricRubricCriteriaItemLevels =
    Vec<ReturnsAreasItemDefinitionsItemRubricRubricCriteriaItemLevelsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItemDefinitionsItemRubricRubricCriteriaItem {
    /// criterion id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// levels
    #[serde(rename = "levels")]
    pub r#levels: Option<r#ReturnsAreasItemDefinitionsItemRubricRubricCriteriaItemLevels>,
}

/// definition details
pub type r#ReturnsAreasItemDefinitionsItemRubricRubricCriteria =
    Vec<ReturnsAreasItemDefinitionsItemRubricRubricCriteriaItem>;

/// items
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItemDefinitionsItemRubric {
    /// definition details
    #[serde(rename = "rubric_criteria")]
    pub r#rubric_criteria: Option<r#ReturnsAreasItemDefinitionsItemRubricRubricCriteria>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItemDefinitionsItem {
    /// definition id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// method
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// copied from id
    #[serde(rename = "copiedfromid")]
    pub r#copiedfromid: Option<i64>,
    /// creation time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// user who created definition
    #[serde(rename = "usercreated")]
    pub r#usercreated: Option<i64>,
    /// last modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// user who modified definition
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// time copied
    #[serde(rename = "timecopied")]
    pub r#timecopied: Option<i64>,
    /// items
    #[serde(rename = "guide")]
    pub r#guide: Option<ReturnsAreasItemDefinitionsItemGuide>,
    /// items
    #[serde(rename = "rubric")]
    pub r#rubric: Option<ReturnsAreasItemDefinitionsItemRubric>,
}

/// definitions
pub type r#ReturnsAreasItemDefinitions = Vec<ReturnsAreasItemDefinitionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAreasItem {
    /// course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// context id
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// component name
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// area name
    #[serde(rename = "areaname")]
    pub r#areaname: Option<String>,
    /// active method
    #[serde(rename = "activemethod")]
    pub r#activemethod: Option<String>,
    /// definitions
    #[serde(rename = "definitions")]
    pub r#definitions: Option<r#ReturnsAreasItemDefinitions>,
}

/// list of grading areas
pub type r#ReturnsAreas = Vec<ReturnsAreasItem>;

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
    /// list of grading areas
    #[serde(rename = "areas")]
    pub r#areas: Option<r#ReturnsAreas>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_grading_get_definitions", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_grading_get_definitions", params).await
}
