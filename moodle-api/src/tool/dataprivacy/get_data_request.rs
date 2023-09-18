use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The request ID
    #[serde(rename = "requestid")]
    pub r#requestid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsResultForuser {
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
pub struct ReturnsResultRequestedbyuser {
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
pub struct ReturnsResultDpouser {
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
pub struct ReturnsResult {
    /// type
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// comments
    #[serde(rename = "comments")]
    pub r#comments: Option<String>,
    /// commentsformat
    #[serde(rename = "commentsformat")]
    pub r#commentsformat: Option<i64>,
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// requestedby
    #[serde(rename = "requestedby")]
    pub r#requestedby: Option<i64>,
    /// status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// dpo
    #[serde(rename = "dpo")]
    pub r#dpo: Option<i64>,
    /// dpocomment
    #[serde(rename = "dpocomment")]
    pub r#dpocomment: Option<String>,
    /// dpocommentformat
    #[serde(rename = "dpocommentformat")]
    pub r#dpocommentformat: Option<i64>,
    /// systemapproved
    #[serde(rename = "systemapproved")]
    pub r#systemapproved: Option<bool>,
    /// creationmethod
    #[serde(rename = "creationmethod")]
    pub r#creationmethod: Option<i64>,
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
    #[serde(rename = "foruser")]
    pub r#foruser: Option<ReturnsResultForuser>,
    #[serde(rename = "requestedbyuser")]
    pub r#requestedbyuser: Option<ReturnsResultRequestedbyuser>,
    #[serde(rename = "dpouser")]
    pub r#dpouser: Option<ReturnsResultDpouser>,
    /// messagehtml
    #[serde(rename = "messagehtml")]
    pub r#messagehtml: Option<String>,
    /// typename
    #[serde(rename = "typename")]
    pub r#typename: Option<String>,
    /// typenameshort
    #[serde(rename = "typenameshort")]
    pub r#typenameshort: Option<String>,
    /// statuslabel
    #[serde(rename = "statuslabel")]
    pub r#statuslabel: Option<String>,
    /// statuslabelclass
    #[serde(rename = "statuslabelclass")]
    pub r#statuslabelclass: Option<String>,
    /// canreview
    #[serde(rename = "canreview")]
    pub r#canreview: Option<bool>,
    /// approvedeny
    #[serde(rename = "approvedeny")]
    pub r#approvedeny: Option<bool>,
    /// canmarkcomplete
    #[serde(rename = "canmarkcomplete")]
    pub r#canmarkcomplete: Option<bool>,
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
    #[serde(rename = "result")]
    pub r#result: Option<ReturnsResult>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_dataprivacy_get_data_request", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_dataprivacy_get_data_request", params)
        .await
}
