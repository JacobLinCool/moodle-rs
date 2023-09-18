use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Data base record id for the user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Data base record id for the competency
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetencyLinkedcoursesItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// summary
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// startdate
    #[serde(rename = "startdate")]
    pub r#startdate: Option<i64>,
    /// enddate
    #[serde(rename = "enddate")]
    pub r#enddate: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// showactivitydates
    #[serde(rename = "showactivitydates")]
    pub r#showactivitydates: Option<bool>,
    /// showcompletionconditions
    #[serde(rename = "showcompletionconditions")]
    pub r#showcompletionconditions: Option<bool>,
    /// pdfexportfont
    #[serde(rename = "pdfexportfont")]
    pub r#pdfexportfont: Option<String>,
    /// fullnamedisplay
    #[serde(rename = "fullnamedisplay")]
    pub r#fullnamedisplay: Option<String>,
    /// viewurl
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// courseimage
    #[serde(rename = "courseimage")]
    pub r#courseimage: Option<String>,
    /// progress
    #[serde(rename = "progress")]
    pub r#progress: Option<i64>,
    /// hasprogress
    #[serde(rename = "hasprogress")]
    pub r#hasprogress: Option<bool>,
    /// isfavourite
    #[serde(rename = "isfavourite")]
    pub r#isfavourite: Option<bool>,
    /// hidden
    #[serde(rename = "hidden")]
    pub r#hidden: Option<bool>,
    /// timeaccess
    #[serde(rename = "timeaccess")]
    pub r#timeaccess: Option<i64>,
    /// showshortname
    #[serde(rename = "showshortname")]
    pub r#showshortname: Option<bool>,
    /// coursecategory
    #[serde(rename = "coursecategory")]
    pub r#coursecategory: Option<String>,
}

/// linkedcourses
pub type r#ReturnsCompetencyLinkedcourses = Vec<ReturnsCompetencyLinkedcoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetencyRelatedcompetenciesItem {
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

/// relatedcompetencies
pub type r#ReturnsCompetencyRelatedcompetencies = Vec<ReturnsCompetencyRelatedcompetenciesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetencyCompetency {
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
pub struct ReturnsCompetencyFramework {
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
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// scaleid
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// scaleconfiguration
    #[serde(rename = "scaleconfiguration")]
    pub r#scaleconfiguration: Option<String>,
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// taxonomies
    #[serde(rename = "taxonomies")]
    pub r#taxonomies: Option<String>,
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
    /// competenciescount
    #[serde(rename = "competenciescount")]
    pub r#competenciescount: Option<i64>,
    /// contextname
    #[serde(rename = "contextname")]
    pub r#contextname: Option<String>,
    /// contextnamenoprefix
    #[serde(rename = "contextnamenoprefix")]
    pub r#contextnamenoprefix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetencyComppathAncestorsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// first
    #[serde(rename = "first")]
    pub r#first: Option<bool>,
    /// last
    #[serde(rename = "last")]
    pub r#last: Option<bool>,
    /// position
    #[serde(rename = "position")]
    pub r#position: Option<i64>,
}

/// ancestors
pub type r#ReturnsCompetencyComppathAncestors = Vec<ReturnsCompetencyComppathAncestorsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetencyComppathFramework {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// first
    #[serde(rename = "first")]
    pub r#first: Option<bool>,
    /// last
    #[serde(rename = "last")]
    pub r#last: Option<bool>,
    /// position
    #[serde(rename = "position")]
    pub r#position: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetencyComppath {
    /// ancestors
    #[serde(rename = "ancestors")]
    pub r#ancestors: Option<r#ReturnsCompetencyComppathAncestors>,
    #[serde(rename = "framework")]
    pub r#framework: Option<ReturnsCompetencyComppathFramework>,
    /// pluginbaseurl
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
    /// pagecontextid
    #[serde(rename = "pagecontextid")]
    pub r#pagecontextid: Option<i64>,
    /// showlinks
    #[serde(rename = "showlinks")]
    pub r#showlinks: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetency {
    /// linkedcourses
    #[serde(rename = "linkedcourses")]
    pub r#linkedcourses: Option<r#ReturnsCompetencyLinkedcourses>,
    /// relatedcompetencies
    #[serde(rename = "relatedcompetencies")]
    pub r#relatedcompetencies: Option<r#ReturnsCompetencyRelatedcompetencies>,
    #[serde(rename = "competency")]
    pub r#competency: Option<ReturnsCompetencyCompetency>,
    #[serde(rename = "framework")]
    pub r#framework: Option<ReturnsCompetencyFramework>,
    /// hascourses
    #[serde(rename = "hascourses")]
    pub r#hascourses: Option<bool>,
    /// hasrelatedcompetencies
    #[serde(rename = "hasrelatedcompetencies")]
    pub r#hasrelatedcompetencies: Option<bool>,
    /// scaleid
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// scaleconfiguration
    #[serde(rename = "scaleconfiguration")]
    pub r#scaleconfiguration: Option<String>,
    /// taxonomyterm
    #[serde(rename = "taxonomyterm")]
    pub r#taxonomyterm: Option<String>,
    #[serde(rename = "comppath")]
    pub r#comppath: Option<ReturnsCompetencyComppath>,
    /// pluginbaseurl
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUser {
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
pub struct ReturnsUsercompetencyReviewer {
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
pub struct ReturnsUsercompetency {
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
    pub r#reviewer: Option<ReturnsUsercompetencyReviewer>,
    /// statusname
    #[serde(rename = "statusname")]
    pub r#statusname: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsercompetencyplan {
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// competencyid
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// proficiency
    #[serde(rename = "proficiency")]
    pub r#proficiency: Option<bool>,
    /// grade
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
    /// planid
    #[serde(rename = "planid")]
    pub r#planid: Option<i64>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
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
    /// gradename
    #[serde(rename = "gradename")]
    pub r#gradename: Option<String>,
    /// proficiencyname
    #[serde(rename = "proficiencyname")]
    pub r#proficiencyname: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUsercompetencycourse {
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// courseid
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// competencyid
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
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
    /// gradename
    #[serde(rename = "gradename")]
    pub r#gradename: Option<String>,
    /// proficiencyname
    #[serde(rename = "proficiencyname")]
    pub r#proficiencyname: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEvidenceItemActionuser {
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
pub struct ReturnsEvidenceItem {
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
    pub r#actionuser: Option<ReturnsEvidenceItemActionuser>,
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

/// evidence
pub type r#ReturnsEvidence = Vec<ReturnsEvidenceItem>;

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
pub struct Returns {
    /// showrelatedcompetencies
    #[serde(rename = "showrelatedcompetencies")]
    pub r#showrelatedcompetencies: Option<bool>,
    /// cangrade
    #[serde(rename = "cangrade")]
    pub r#cangrade: Option<bool>,
    #[serde(rename = "competency")]
    pub r#competency: Option<ReturnsCompetency>,
    #[serde(rename = "user")]
    pub r#user: Option<ReturnsUser>,
    #[serde(rename = "usercompetency")]
    pub r#usercompetency: Option<ReturnsUsercompetency>,
    #[serde(rename = "usercompetencyplan")]
    pub r#usercompetencyplan: Option<ReturnsUsercompetencyplan>,
    #[serde(rename = "usercompetencycourse")]
    pub r#usercompetencycourse: Option<ReturnsUsercompetencycourse>,
    /// evidence
    #[serde(rename = "evidence")]
    pub r#evidence: Option<r#ReturnsEvidence>,
    #[serde(rename = "commentarea")]
    pub r#commentarea: Option<ReturnsCommentarea>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_data_for_user_competency_summary", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_data_for_user_competency_summary", params)
        .await
}
