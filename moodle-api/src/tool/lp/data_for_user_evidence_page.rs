use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The user evidence ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserevidenceFilesItem {
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// filearea
    #[serde(rename = "filearea")]
    pub r#filearea: Option<String>,
    /// itemid
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// filepath
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// filename
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// isdir
    #[serde(rename = "isdir")]
    pub r#isdir: Option<bool>,
    /// isimage
    #[serde(rename = "isimage")]
    pub r#isimage: Option<bool>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// timecreated
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// filesize
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// author
    #[serde(rename = "author")]
    pub r#author: Option<String>,
    /// license
    #[serde(rename = "license")]
    pub r#license: Option<String>,
    /// filenameshort
    #[serde(rename = "filenameshort")]
    pub r#filenameshort: Option<String>,
    /// filesizeformatted
    #[serde(rename = "filesizeformatted")]
    pub r#filesizeformatted: Option<String>,
    /// icon
    #[serde(rename = "icon")]
    pub r#icon: Option<String>,
    /// timecreatedformatted
    #[serde(rename = "timecreatedformatted")]
    pub r#timecreatedformatted: Option<String>,
    /// timemodifiedformatted
    #[serde(rename = "timemodifiedformatted")]
    pub r#timemodifiedformatted: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

/// files
pub type r#ReturnsUserevidenceFiles = Vec<ReturnsUserevidenceFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserevidenceUsercompetenciesItemCompetency {
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// parentid
    #[serde(rename = "parentid")]
    pub r#parentid: Option<i64>,
    /// path
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// ruleoutcome
    #[serde(rename = "ruleoutcome")]
    pub r#ruleoutcome: Option<i64>,
    /// ruletype
    #[serde(rename = "ruletype")]
    pub r#ruletype: Option<String>,
    /// ruleconfig
    #[serde(rename = "ruleconfig")]
    pub r#ruleconfig: Option<String>,
    /// scaleid
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// scaleconfiguration
    #[serde(rename = "scaleconfiguration")]
    pub r#scaleconfiguration: Option<String>,
    /// competencyframeworkid
    #[serde(rename = "competencyframeworkid")]
    pub r#competencyframeworkid: Option<i64>,
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserevidenceUsercompetenciesItemUsercompetencyReviewer {
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
pub struct ReturnsUserevidenceUsercompetenciesItemUsercompetency {
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// competencyid
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// reviewerid
    #[serde(rename = "reviewerid")]
    pub r#reviewerid: Option<i64>,
    /// proficiency
    #[serde(rename = "proficiency")]
    pub r#proficiency: Option<bool>,
    /// grade
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
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
    /// canrequestreview
    #[serde(rename = "canrequestreview")]
    pub r#canrequestreview: Option<bool>,
    /// canreview
    #[serde(rename = "canreview")]
    pub r#canreview: Option<bool>,
    /// gradename
    #[serde(rename = "gradename")]
    pub r#gradename: Option<String>,
    /// isrequestreviewallowed
    #[serde(rename = "isrequestreviewallowed")]
    pub r#isrequestreviewallowed: Option<bool>,
    /// iscancelreviewrequestallowed
    #[serde(rename = "iscancelreviewrequestallowed")]
    pub r#iscancelreviewrequestallowed: Option<bool>,
    /// isstartreviewallowed
    #[serde(rename = "isstartreviewallowed")]
    pub r#isstartreviewallowed: Option<bool>,
    /// isstopreviewallowed
    #[serde(rename = "isstopreviewallowed")]
    pub r#isstopreviewallowed: Option<bool>,
    /// isstatusidle
    #[serde(rename = "isstatusidle")]
    pub r#isstatusidle: Option<bool>,
    /// isstatusinreview
    #[serde(rename = "isstatusinreview")]
    pub r#isstatusinreview: Option<bool>,
    /// isstatuswaitingforreview
    #[serde(rename = "isstatuswaitingforreview")]
    pub r#isstatuswaitingforreview: Option<bool>,
    /// proficiencyname
    #[serde(rename = "proficiencyname")]
    pub r#proficiencyname: Option<String>,
    #[serde(rename = "reviewer")]
    pub r#reviewer: Option<ReturnsUserevidenceUsercompetenciesItemUsercompetencyReviewer>,
    /// statusname
    #[serde(rename = "statusname")]
    pub r#statusname: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserevidenceUsercompetenciesItem {
    #[serde(rename = "competency")]
    pub r#competency: Option<ReturnsUserevidenceUsercompetenciesItemCompetency>,
    #[serde(rename = "usercompetency")]
    pub r#usercompetency: Option<ReturnsUserevidenceUsercompetenciesItemUsercompetency>,
}

/// usercompetencies
pub type r#ReturnsUserevidenceUsercompetencies = Vec<ReturnsUserevidenceUsercompetenciesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserevidence {
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
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
    /// canmanage
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// filecount
    #[serde(rename = "filecount")]
    pub r#filecount: Option<i64>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsUserevidenceFiles>,
    /// hasurlorfiles
    #[serde(rename = "hasurlorfiles")]
    pub r#hasurlorfiles: Option<bool>,
    /// urlshort
    #[serde(rename = "urlshort")]
    pub r#urlshort: Option<String>,
    /// competencycount
    #[serde(rename = "competencycount")]
    pub r#competencycount: Option<i64>,
    /// usercompetencies
    #[serde(rename = "usercompetencies")]
    pub r#usercompetencies: Option<r#ReturnsUserevidenceUsercompetencies>,
    /// userhasplan
    #[serde(rename = "userhasplan")]
    pub r#userhasplan: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "userevidence")]
    pub r#userevidence: Option<ReturnsUserevidence>,
    /// Url to the tool_lp plugin folder on this Moodle site
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_data_for_user_evidence_page", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_data_for_user_evidence_page", params)
        .await
}
