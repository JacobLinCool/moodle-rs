use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Data base record id for the plan
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCommentarea {
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// commentarea
    #[serde(rename = "commentarea")]
    pub r#commentarea: Option<String>,
    /// itemid
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// courseid
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// cid
    #[serde(rename = "cid")]
    pub r#cid: Option<String>,
    /// autostart
    #[serde(rename = "autostart")]
    pub r#autostart: Option<bool>,
    /// canpost
    #[serde(rename = "canpost")]
    pub r#canpost: Option<bool>,
    /// canview
    #[serde(rename = "canview")]
    pub r#canview: Option<bool>,
    /// count
    #[serde(rename = "count")]
    pub r#count: Option<i64>,
    /// collapsediconkey
    #[serde(rename = "collapsediconkey")]
    pub r#collapsediconkey: Option<String>,
    /// displaytotalcount
    #[serde(rename = "displaytotalcount")]
    pub r#displaytotalcount: Option<bool>,
    /// displaycancel
    #[serde(rename = "displaycancel")]
    pub r#displaycancel: Option<bool>,
    /// fullwidth
    #[serde(rename = "fullwidth")]
    pub r#fullwidth: Option<bool>,
    /// linktext
    #[serde(rename = "linktext")]
    pub r#linktext: Option<String>,
    /// notoggle
    #[serde(rename = "notoggle")]
    pub r#notoggle: Option<bool>,
    /// template
    #[serde(rename = "template")]
    pub r#template: Option<String>,
    /// canpostorhascomments
    #[serde(rename = "canpostorhascomments")]
    pub r#canpostorhascomments: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsReviewer {
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
pub struct ReturnsTemplate {
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// duedate
    #[serde(rename = "duedate")]
    pub r#duedate: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
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
    /// duedateformatted
    #[serde(rename = "duedateformatted")]
    pub r#duedateformatted: Option<String>,
    /// cohortscount
    #[serde(rename = "cohortscount")]
    pub r#cohortscount: Option<i64>,
    /// planscount
    #[serde(rename = "planscount")]
    pub r#planscount: Option<i64>,
    /// canmanage
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// canread
    #[serde(rename = "canread")]
    pub r#canread: Option<bool>,
    /// contextname
    #[serde(rename = "contextname")]
    pub r#contextname: Option<String>,
    /// contextnamenoprefix
    #[serde(rename = "contextnamenoprefix")]
    pub r#contextnamenoprefix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// templateid
    #[serde(rename = "templateid")]
    pub r#templateid: Option<i64>,
    /// origtemplateid
    #[serde(rename = "origtemplateid")]
    pub r#origtemplateid: Option<i64>,
    /// status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// duedate
    #[serde(rename = "duedate")]
    pub r#duedate: Option<i64>,
    /// reviewerid
    #[serde(rename = "reviewerid")]
    pub r#reviewerid: Option<i64>,
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
    /// statusname
    #[serde(rename = "statusname")]
    pub r#statusname: Option<String>,
    /// isbasedontemplate
    #[serde(rename = "isbasedontemplate")]
    pub r#isbasedontemplate: Option<bool>,
    /// canmanage
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// canrequestreview
    #[serde(rename = "canrequestreview")]
    pub r#canrequestreview: Option<bool>,
    /// canreview
    #[serde(rename = "canreview")]
    pub r#canreview: Option<bool>,
    /// canbeedited
    #[serde(rename = "canbeedited")]
    pub r#canbeedited: Option<bool>,
    /// isactive
    #[serde(rename = "isactive")]
    pub r#isactive: Option<bool>,
    /// isdraft
    #[serde(rename = "isdraft")]
    pub r#isdraft: Option<bool>,
    /// iscompleted
    #[serde(rename = "iscompleted")]
    pub r#iscompleted: Option<bool>,
    /// isinreview
    #[serde(rename = "isinreview")]
    pub r#isinreview: Option<bool>,
    /// iswaitingforreview
    #[serde(rename = "iswaitingforreview")]
    pub r#iswaitingforreview: Option<bool>,
    /// isreopenallowed
    #[serde(rename = "isreopenallowed")]
    pub r#isreopenallowed: Option<bool>,
    /// iscompleteallowed
    #[serde(rename = "iscompleteallowed")]
    pub r#iscompleteallowed: Option<bool>,
    /// isunlinkallowed
    #[serde(rename = "isunlinkallowed")]
    pub r#isunlinkallowed: Option<bool>,
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
    /// isapproveallowed
    #[serde(rename = "isapproveallowed")]
    pub r#isapproveallowed: Option<bool>,
    /// isunapproveallowed
    #[serde(rename = "isunapproveallowed")]
    pub r#isunapproveallowed: Option<bool>,
    /// duedateformatted
    #[serde(rename = "duedateformatted")]
    pub r#duedateformatted: Option<String>,
    #[serde(rename = "commentarea")]
    pub r#commentarea: Option<ReturnsCommentarea>,
    #[serde(rename = "reviewer")]
    pub r#reviewer: Option<ReturnsReviewer>,
    #[serde(rename = "template")]
    pub r#template: Option<ReturnsTemplate>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_competency_read_plan", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_competency_read_plan", params).await
}
