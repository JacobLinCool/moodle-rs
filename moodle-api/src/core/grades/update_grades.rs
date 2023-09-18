use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsGradesItem {
    /// Student ID
    #[serde(rename = "studentid")]
    pub r#studentid: Option<i64>,
    /// Student grade
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// A string representation of the feedback from the grader
    #[serde(rename = "str_feedback")]
    pub r#str_feedback: Option<String>,
}

/// Any student grades to alter
pub type r#ParamsGrades = Vec<ParamsGradesItem>;

/// Any grade item settings to alter
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsItemdetails {
    /// The grade item name
    #[serde(rename = "itemname")]
    pub r#itemname: Option<String>,
    /// Arbitrary ID provided by the module responsible for the grade item
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<i64>,
    /// The type of grade (0 = none, 1 = value, 2 = scale, 3 = text)
    #[serde(rename = "gradetype")]
    pub r#gradetype: Option<i64>,
    /// Maximum grade allowed
    #[serde(rename = "grademax")]
    pub r#grademax: Option<f64>,
    /// Minimum grade allowed
    #[serde(rename = "grademin")]
    pub r#grademin: Option<f64>,
    /// The ID of the custom scale being is used
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// Multiply all grades by this number
    #[serde(rename = "multfactor")]
    pub r#multfactor: Option<f64>,
    /// Add this to all grades
    #[serde(rename = "plusfactor")]
    pub r#plusfactor: Option<f64>,
    /// True if the grade item should be deleted
    #[serde(rename = "deleted")]
    pub r#deleted: Option<bool>,
    /// True if the grade item is hidden
    #[serde(rename = "hidden")]
    pub r#hidden: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The source of the grade update
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    /// id of course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// A component, for example mod_forum or mod_quiz
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// The activity ID
    #[serde(rename = "activityid")]
    pub r#activityid: Option<i64>,
    /// grade item ID number for modules that have multiple grades. Typically this is 0.
    #[serde(rename = "itemnumber")]
    pub r#itemnumber: Option<i64>,
    /// Any student grades to alter
    #[serde(rename = "grades")]
    pub r#grades: Option<r#ParamsGrades>,
    /// Any grade item settings to alter
    #[serde(rename = "itemdetails")]
    pub r#itemdetails: Option<ParamsItemdetails>,
}
