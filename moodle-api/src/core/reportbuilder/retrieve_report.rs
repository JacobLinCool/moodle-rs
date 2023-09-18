use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report ID
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
    /// Page number
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// Reports per page
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDetailsModifiedby {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// email
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// phone1
    #[serde(rename = "phone1")]
    pub r#phone1: Option<String>,
    /// phone2
    #[serde(rename = "phone2")]
    pub r#phone2: Option<String>,
    /// department
    #[serde(rename = "department")]
    pub r#department: Option<String>,
    /// institution
    #[serde(rename = "institution")]
    pub r#institution: Option<String>,
    /// fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// identity
    #[serde(rename = "identity")]
    pub r#identity: Option<String>,
    /// profileurl
    #[serde(rename = "profileurl")]
    pub r#profileurl: Option<String>,
    /// profileimageurl
    #[serde(rename = "profileimageurl")]
    pub r#profileimageurl: Option<String>,
    /// profileimageurlsmall
    #[serde(rename = "profileimageurlsmall")]
    pub r#profileimageurlsmall: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDetails {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// source
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    /// type
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// uniquerows
    #[serde(rename = "uniquerows")]
    pub r#uniquerows: Option<bool>,
    /// conditiondata
    #[serde(rename = "conditiondata")]
    pub r#conditiondata: Option<String>,
    /// settingsdata
    #[serde(rename = "settingsdata")]
    pub r#settingsdata: Option<String>,
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// itemid
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// usercreated
    #[serde(rename = "usercreated")]
    pub r#usercreated: Option<i64>,
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// timecreated
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// usermodified
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// sourcename
    #[serde(rename = "sourcename")]
    pub r#sourcename: Option<String>,
    #[serde(rename = "modifiedby")]
    pub r#modifiedby: Option<ReturnsDetailsModifiedby>,
}

/// headers
pub type r#ReturnsDataHeaders = Vec<String>;

/// columns
pub type r#ReturnsDataRowsItemColumns = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDataRowsItem {
    /// columns
    #[serde(rename = "columns")]
    pub r#columns: Option<r#ReturnsDataRowsItemColumns>,
}

/// rows
pub type r#ReturnsDataRows = Vec<ReturnsDataRowsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsData {
    /// headers
    #[serde(rename = "headers")]
    pub r#headers: Option<r#ReturnsDataHeaders>,
    /// rows
    #[serde(rename = "rows")]
    pub r#rows: Option<r#ReturnsDataRows>,
    /// totalrowcount
    #[serde(rename = "totalrowcount")]
    pub r#totalrowcount: Option<i64>,
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
    #[serde(rename = "details")]
    pub r#details: Option<ReturnsDetails>,
    #[serde(rename = "data")]
    pub r#data: Option<ReturnsData>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_reportbuilder_retrieve_report", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_reportbuilder_retrieve_report", params)
        .await
}
