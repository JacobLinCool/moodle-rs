use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCmAdvancedgradingItem {
    /// Gradable area name
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// Grading method
    #[serde(rename = "method")]
    pub r#method: Option<String>,
}

/// Advanced grading settings
pub type r#ReturnsCmAdvancedgrading = Vec<ReturnsCmAdvancedgradingItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCmOutcomesItem {
    /// Outcome id
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Outcome full name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Scale items
    #[serde(rename = "scale")]
    pub r#scale: Option<String>,
}

/// Outcomes information
pub type r#ReturnsCmOutcomes = Vec<ReturnsCmOutcomesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCm {
    /// The course module id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The course id
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// The module type id
    #[serde(rename = "module")]
    pub r#module: Option<i64>,
    /// The activity name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The module component name (forum, assign, etc..)
    #[serde(rename = "modname")]
    pub r#modname: Option<String>,
    /// The activity instance id
    #[serde(rename = "instance")]
    pub r#instance: Option<i64>,
    /// The module section id
    #[serde(rename = "section")]
    pub r#section: Option<i64>,
    /// The module section number
    #[serde(rename = "sectionnum")]
    pub r#sectionnum: Option<i64>,
    /// Group mode
    #[serde(rename = "groupmode")]
    pub r#groupmode: Option<i64>,
    /// Grouping id
    #[serde(rename = "groupingid")]
    pub r#groupingid: Option<i64>,
    /// If completion is enabled
    #[serde(rename = "completion")]
    pub r#completion: Option<i64>,
    /// Module id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// Time added
    #[serde(rename = "added")]
    pub r#added: Option<i64>,
    /// Score
    #[serde(rename = "score")]
    pub r#score: Option<i64>,
    /// Indentation
    #[serde(rename = "indent")]
    pub r#indent: Option<i64>,
    /// If visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// If visible on course page
    #[serde(rename = "visibleoncoursepage")]
    pub r#visibleoncoursepage: Option<i64>,
    /// Visible old
    #[serde(rename = "visibleold")]
    pub r#visibleold: Option<i64>,
    /// Completion grade item
    #[serde(rename = "completiongradeitemnumber")]
    pub r#completiongradeitemnumber: Option<i64>,
    /// Completion pass grade setting
    #[serde(rename = "completionpassgrade")]
    pub r#completionpassgrade: Option<i64>,
    /// Completion view setting
    #[serde(rename = "completionview")]
    pub r#completionview: Option<i64>,
    /// Completion time expected
    #[serde(rename = "completionexpected")]
    pub r#completionexpected: Option<i64>,
    /// If the description is showed
    #[serde(rename = "showdescription")]
    pub r#showdescription: Option<i64>,
    /// The download content value
    #[serde(rename = "downloadcontent")]
    pub r#downloadcontent: Option<i64>,
    /// Availability settings
    #[serde(rename = "availability")]
    pub r#availability: Option<String>,
    /// Grade (max value or scale id)
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// Scale items (if used)
    #[serde(rename = "scale")]
    pub r#scale: Option<String>,
    /// Grade to pass (float)
    #[serde(rename = "gradepass")]
    pub r#gradepass: Option<String>,
    /// Grade category
    #[serde(rename = "gradecat")]
    pub r#gradecat: Option<i64>,
    /// Advanced grading settings
    #[serde(rename = "advancedgrading")]
    pub r#advancedgrading: Option<r#ReturnsCmAdvancedgrading>,
    /// Outcomes information
    #[serde(rename = "outcomes")]
    pub r#outcomes: Option<r#ReturnsCmOutcomes>,
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
    #[serde(rename = "cm")]
    pub r#cm: Option<ReturnsCm>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_course_get_course_module", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_course_get_course_module", params).await
}
