use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course Id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Return grades only for this user (optional)
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Get users from this group only
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

/// Grade items
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsergradesItemGradeitemsItem {
    /// Grade item id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Grade item name
    #[serde(rename = "itemname")]
    pub r#itemname: Option<String>,
    /// Grade item type
    #[serde(rename = "itemtype")]
    pub r#itemtype: Option<String>,
    /// Grade item module
    #[serde(rename = "itemmodule")]
    pub r#itemmodule: Option<String>,
    /// Grade item instance
    #[serde(rename = "iteminstance")]
    pub r#iteminstance: Option<i64>,
    /// Grade item item number
    #[serde(rename = "itemnumber")]
    pub r#itemnumber: Option<i64>,
    /// Grade item idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// Grade item category id
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// Outcome id
    #[serde(rename = "outcomeid")]
    pub r#outcomeid: Option<i64>,
    /// Scale id
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// Grade item for user locked?
    #[serde(rename = "locked")]
    pub r#locked: Option<bool>,
    /// Course module id (if type mod)
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// Weight raw
    #[serde(rename = "weightraw")]
    pub r#weightraw: Option<f64>,
    /// Weight
    #[serde(rename = "weightformatted")]
    pub r#weightformatted: Option<String>,
    /// Status
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Grade raw
    #[serde(rename = "graderaw")]
    pub r#graderaw: Option<f64>,
    /// Grade submit date
    #[serde(rename = "gradedatesubmitted")]
    pub r#gradedatesubmitted: Option<i64>,
    /// Grade graded date
    #[serde(rename = "gradedategraded")]
    pub r#gradedategraded: Option<i64>,
    /// Grade hidden by date?
    #[serde(rename = "gradehiddenbydate")]
    pub r#gradehiddenbydate: Option<bool>,
    /// Grade needs update?
    #[serde(rename = "gradeneedsupdate")]
    pub r#gradeneedsupdate: Option<bool>,
    /// Grade is hidden?
    #[serde(rename = "gradeishidden")]
    pub r#gradeishidden: Option<bool>,
    /// Grade is locked?
    #[serde(rename = "gradeislocked")]
    pub r#gradeislocked: Option<bool>,
    /// Grade overridden?
    #[serde(rename = "gradeisoverridden")]
    pub r#gradeisoverridden: Option<bool>,
    /// The grade formatted
    #[serde(rename = "gradeformatted")]
    pub r#gradeformatted: Option<String>,
    /// Grade min
    #[serde(rename = "grademin")]
    pub r#grademin: Option<f64>,
    /// Grade max
    #[serde(rename = "grademax")]
    pub r#grademax: Option<f64>,
    /// Range formatted
    #[serde(rename = "rangeformatted")]
    pub r#rangeformatted: Option<String>,
    /// Percentage
    #[serde(rename = "percentageformatted")]
    pub r#percentageformatted: Option<String>,
    /// Letter grade
    #[serde(rename = "lettergradeformatted")]
    pub r#lettergradeformatted: Option<String>,
    /// Rank in the course
    #[serde(rename = "rank")]
    pub r#rank: Option<i64>,
    /// Num users in course
    #[serde(rename = "numusers")]
    pub r#numusers: Option<i64>,
    /// Grade average
    #[serde(rename = "averageformatted")]
    pub r#averageformatted: Option<String>,
    /// Grade feedback
    #[serde(rename = "feedback")]
    pub r#feedback: Option<String>,
    /// feedback format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "feedbackformat")]
    pub r#feedbackformat: Option<i64>,
}

pub type r#ReturnsUsergradesItemGradeitems = Vec<ReturnsUsergradesItemGradeitemsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsergradesItem {
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// course idnumber
    #[serde(rename = "courseidnumber")]
    pub r#courseidnumber: Option<String>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// user fullname
    #[serde(rename = "userfullname")]
    pub r#userfullname: Option<String>,
    /// user idnumber
    #[serde(rename = "useridnumber")]
    pub r#useridnumber: Option<String>,
    /// table max depth (needed for printing it)
    #[serde(rename = "maxdepth")]
    pub r#maxdepth: Option<i64>,
    #[serde(rename = "gradeitems")]
    pub r#gradeitems: Option<r#ReturnsUsergradesItemGradeitems>,
}

pub type r#ReturnsUsergrades = Vec<ReturnsUsergradesItem>;

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
    #[serde(rename = "usergrades")]
    pub r#usergrades: Option<r#ReturnsUsergrades>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("gradereport_user_get_grade_items", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("gradereport_user_get_grade_items", params)
        .await
}
