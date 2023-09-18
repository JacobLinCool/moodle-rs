use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// User ID
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Competency ID
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// New grade
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
    /// A note to attach to the evidence
    #[serde(rename = "note")]
    pub r#note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsActionuser {
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
pub struct Returns {
    /// usercompetencyid
    #[serde(rename = "usercompetencyid")]
    pub r#usercompetencyid: Option<i64>,
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// action
    #[serde(rename = "action")]
    pub r#action: Option<i64>,
    /// actionuserid
    #[serde(rename = "actionuserid")]
    pub r#actionuserid: Option<i64>,
    /// descidentifier
    #[serde(rename = "descidentifier")]
    pub r#descidentifier: Option<String>,
    /// desccomponent
    #[serde(rename = "desccomponent")]
    pub r#desccomponent: Option<String>,
    /// desca
    #[serde(rename = "desca")]
    pub r#desca: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// grade
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
    /// note
    #[serde(rename = "note")]
    pub r#note: Option<String>,
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
    #[serde(rename = "actionuser")]
    pub r#actionuser: Option<ReturnsActionuser>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// gradename
    #[serde(rename = "gradename")]
    pub r#gradename: Option<String>,
    /// userdate
    #[serde(rename = "userdate")]
    pub r#userdate: Option<String>,
    /// candelete
    #[serde(rename = "candelete")]
    pub r#candelete: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_competency_grade_competency", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_competency_grade_competency", params)
        .await
}
