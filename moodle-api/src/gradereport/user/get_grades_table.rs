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

/// The item returned data
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemItemname {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// col span
    #[serde(rename = "colspan")]
    pub r#colspan: Option<i64>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<String>,
}

/// The item returned data
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemLeader {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// row span
    #[serde(rename = "rowspan")]
    pub r#rowspan: Option<i64>,
}

/// weight column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemWeight {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

/// grade column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemGrade {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

/// range column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemRange {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

/// percentage column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemPercentage {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

/// lettergrade column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemLettergrade {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

/// rank column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemRank {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

/// average column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemAverage {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

/// feedback column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemFeedback {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

/// contributiontocoursetotal column
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItemContributiontocoursetotal {
    /// class
    #[serde(rename = "class")]
    pub r#class: Option<String>,
    /// cell content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<String>,
}

pub type r#ReturnsTablesItemTabledataItemParentcategories = Vec<i64>;

/// table
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItemTabledataItem {
    /// The item returned data
    #[serde(rename = "itemname")]
    pub r#itemname: Option<ReturnsTablesItemTabledataItemItemname>,
    /// The item returned data
    #[serde(rename = "leader")]
    pub r#leader: Option<ReturnsTablesItemTabledataItemLeader>,
    /// weight column
    #[serde(rename = "weight")]
    pub r#weight: Option<ReturnsTablesItemTabledataItemWeight>,
    /// grade column
    #[serde(rename = "grade")]
    pub r#grade: Option<ReturnsTablesItemTabledataItemGrade>,
    /// range column
    #[serde(rename = "range")]
    pub r#range: Option<ReturnsTablesItemTabledataItemRange>,
    /// percentage column
    #[serde(rename = "percentage")]
    pub r#percentage: Option<ReturnsTablesItemTabledataItemPercentage>,
    /// lettergrade column
    #[serde(rename = "lettergrade")]
    pub r#lettergrade: Option<ReturnsTablesItemTabledataItemLettergrade>,
    /// rank column
    #[serde(rename = "rank")]
    pub r#rank: Option<ReturnsTablesItemTabledataItemRank>,
    /// average column
    #[serde(rename = "average")]
    pub r#average: Option<ReturnsTablesItemTabledataItemAverage>,
    /// feedback column
    #[serde(rename = "feedback")]
    pub r#feedback: Option<ReturnsTablesItemTabledataItemFeedback>,
    /// contributiontocoursetotal column
    #[serde(rename = "contributiontocoursetotal")]
    pub r#contributiontocoursetotal:
        Option<ReturnsTablesItemTabledataItemContributiontocoursetotal>,
    #[serde(rename = "parentcategories")]
    pub r#parentcategories: Option<r#ReturnsTablesItemTabledataItemParentcategories>,
}

pub type r#ReturnsTablesItemTabledata = Vec<ReturnsTablesItemTabledataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTablesItem {
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// user fullname
    #[serde(rename = "userfullname")]
    pub r#userfullname: Option<String>,
    /// table max depth (needed for printing it)
    #[serde(rename = "maxdepth")]
    pub r#maxdepth: Option<i64>,
    #[serde(rename = "tabledata")]
    pub r#tabledata: Option<r#ReturnsTablesItemTabledata>,
}

pub type r#ReturnsTables = Vec<ReturnsTablesItem>;

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
    #[serde(rename = "tables")]
    pub r#tables: Option<r#ReturnsTables>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("gradereport_user_get_grades_table", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("gradereport_user_get_grades_table", params)
        .await
}
