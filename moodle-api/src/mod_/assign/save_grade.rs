use serde::{self, Deserialize, Serialize};

/// Editor structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPlugindataAssignfeedbackcommentsEditor {
    /// The text for this feedback.
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// The format for this feedback
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
}

/// plugin data
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPlugindata {
    /// Editor structure
    #[serde(rename = "assignfeedbackcomments_editor")]
    pub r#assignfeedbackcomments_editor: Option<ParamsPlugindataAssignfeedbackcommentsEditor>,
    /// The id of a draft area containing files for this feedback.
    #[serde(rename = "files_filemanager")]
    pub r#files_filemanager: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAdvancedgradingdataGuideCriteriaItemFillingsItem {
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
pub type r#ParamsAdvancedgradingdataGuideCriteriaItemFillings =
    Vec<ParamsAdvancedgradingdataGuideCriteriaItemFillingsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAdvancedgradingdataGuideCriteriaItem {
    /// criterion id
    #[serde(rename = "criterionid")]
    pub r#criterionid: Option<i64>,
    /// filling
    #[serde(rename = "fillings")]
    pub r#fillings: Option<r#ParamsAdvancedgradingdataGuideCriteriaItemFillings>,
}

pub type r#ParamsAdvancedgradingdataGuideCriteria = Vec<ParamsAdvancedgradingdataGuideCriteriaItem>;

/// items
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAdvancedgradingdataGuide {
    #[serde(rename = "criteria")]
    pub r#criteria: Option<r#ParamsAdvancedgradingdataGuideCriteria>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAdvancedgradingdataRubricCriteriaItemFillingsItem {
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
pub type r#ParamsAdvancedgradingdataRubricCriteriaItemFillings =
    Vec<ParamsAdvancedgradingdataRubricCriteriaItemFillingsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAdvancedgradingdataRubricCriteriaItem {
    /// criterion id
    #[serde(rename = "criterionid")]
    pub r#criterionid: Option<i64>,
    /// filling
    #[serde(rename = "fillings")]
    pub r#fillings: Option<r#ParamsAdvancedgradingdataRubricCriteriaItemFillings>,
}

pub type r#ParamsAdvancedgradingdataRubricCriteria =
    Vec<ParamsAdvancedgradingdataRubricCriteriaItem>;

/// items
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAdvancedgradingdataRubric {
    #[serde(rename = "criteria")]
    pub r#criteria: Option<r#ParamsAdvancedgradingdataRubricCriteria>,
}

/// advanced grading data
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAdvancedgradingdata {
    /// items
    #[serde(rename = "guide")]
    pub r#guide: Option<ParamsAdvancedgradingdataGuide>,
    /// items
    #[serde(rename = "rubric")]
    pub r#rubric: Option<ParamsAdvancedgradingdataRubric>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The assignment id to operate on
    #[serde(rename = "assignmentid")]
    pub r#assignmentid: Option<i64>,
    /// The student id to operate on
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The new grade for this user. Ignored if advanced grading used
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// The attempt number (-1 means latest attempt)
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// Allow another attempt if the attempt reopen method is manual
    #[serde(rename = "addattempt")]
    pub r#addattempt: Option<bool>,
    /// The next marking workflow state
    #[serde(rename = "workflowstate")]
    pub r#workflowstate: Option<String>,
    /// If true, this grade will be applied to all members of the group (for group assignments).
    #[serde(rename = "applytoall")]
    pub r#applytoall: Option<bool>,
    /// plugin data
    #[serde(rename = "plugindata")]
    pub r#plugindata: Option<ParamsPlugindata>,
    /// advanced grading data
    #[serde(rename = "advancedgradingdata")]
    pub r#advancedgradingdata: Option<ParamsAdvancedgradingdata>,
}
