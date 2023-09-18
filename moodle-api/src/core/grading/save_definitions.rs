use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAreasItemDefinitionsItemGuideGuideCriteriaItem {
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

pub type r#ParamsAreasItemDefinitionsItemGuideGuideCriteria =
    Vec<ParamsAreasItemDefinitionsItemGuideGuideCriteriaItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAreasItemDefinitionsItemGuideGuideCommentsItem {
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
pub type r#ParamsAreasItemDefinitionsItemGuideGuideComments =
    Vec<ParamsAreasItemDefinitionsItemGuideGuideCommentsItem>;

/// items
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAreasItemDefinitionsItemGuide {
    #[serde(rename = "guide_criteria")]
    pub r#guide_criteria: Option<r#ParamsAreasItemDefinitionsItemGuideGuideCriteria>,
    /// comments
    #[serde(rename = "guide_comments")]
    pub r#guide_comments: Option<r#ParamsAreasItemDefinitionsItemGuideGuideComments>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAreasItemDefinitionsItemRubricRubricCriteriaItemLevelsItem {
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
pub type r#ParamsAreasItemDefinitionsItemRubricRubricCriteriaItemLevels =
    Vec<ParamsAreasItemDefinitionsItemRubricRubricCriteriaItemLevelsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAreasItemDefinitionsItemRubricRubricCriteriaItem {
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
    pub r#levels: Option<r#ParamsAreasItemDefinitionsItemRubricRubricCriteriaItemLevels>,
}

/// definition details
pub type r#ParamsAreasItemDefinitionsItemRubricRubricCriteria =
    Vec<ParamsAreasItemDefinitionsItemRubricRubricCriteriaItem>;

/// items
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAreasItemDefinitionsItemRubric {
    /// definition details
    #[serde(rename = "rubric_criteria")]
    pub r#rubric_criteria: Option<r#ParamsAreasItemDefinitionsItemRubricRubricCriteria>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAreasItemDefinitionsItem {
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
    pub r#guide: Option<ParamsAreasItemDefinitionsItemGuide>,
    /// items
    #[serde(rename = "rubric")]
    pub r#rubric: Option<ParamsAreasItemDefinitionsItemRubric>,
}

/// definitions
pub type r#ParamsAreasItemDefinitions = Vec<ParamsAreasItemDefinitionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAreasItem {
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
    pub r#definitions: Option<r#ParamsAreasItemDefinitions>,
}

/// areas with definitions to save
pub type r#ParamsAreas = Vec<ParamsAreasItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// areas with definitions to save
    #[serde(rename = "areas")]
    pub r#areas: Option<r#ParamsAreas>,
}
