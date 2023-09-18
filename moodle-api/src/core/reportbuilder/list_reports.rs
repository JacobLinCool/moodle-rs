use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Page number
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// Reports per page
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsReportsItemModifiedby {
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
pub struct ReturnsReportsItem {
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
    pub r#modifiedby: Option<ReturnsReportsItemModifiedby>,
}

pub type r#ReturnsReports = Vec<ReturnsReportsItem>;

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
    #[serde(rename = "reports")]
    pub r#reports: Option<r#ReturnsReports>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_reportbuilder_list_reports", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_reportbuilder_list_reports", params).await
}
