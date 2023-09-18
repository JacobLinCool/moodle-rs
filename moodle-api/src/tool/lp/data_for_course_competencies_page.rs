use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// The module id
    #[serde(rename = "moduleid")]
    pub r#moduleid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSettings {
    /// courseid
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// pushratingstouserplans
    #[serde(rename = "pushratingstouserplans")]
    pub r#pushratingstouserplans: Option<bool>,
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
pub struct ReturnsStatisticsLeastproficientItem {
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

/// leastproficient
pub type r#ReturnsStatisticsLeastproficient = Vec<ReturnsStatisticsLeastproficientItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsStatistics {
    /// competencycount
    #[serde(rename = "competencycount")]
    pub r#competencycount: Option<i64>,
    /// proficientcompetencycount
    #[serde(rename = "proficientcompetencycount")]
    pub r#proficientcompetencycount: Option<i64>,
    /// proficientcompetencypercentage
    #[serde(rename = "proficientcompetencypercentage")]
    pub r#proficientcompetencypercentage: Option<f64>,
    /// proficientcompetencypercentageformatted
    #[serde(rename = "proficientcompetencypercentageformatted")]
    pub r#proficientcompetencypercentageformatted: Option<String>,
    /// leastproficient
    #[serde(rename = "leastproficient")]
    pub r#leastproficient: Option<r#ReturnsStatisticsLeastproficient>,
    /// leastproficientcount
    #[serde(rename = "leastproficientcount")]
    pub r#leastproficientcount: Option<i64>,
    /// canbegradedincourse
    #[serde(rename = "canbegradedincourse")]
    pub r#canbegradedincourse: Option<bool>,
    /// canmanagecoursecompetencies
    #[serde(rename = "canmanagecoursecompetencies")]
    pub r#canmanagecoursecompetencies: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetenciesItemCompetency {
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
pub struct ReturnsCompetenciesItemCoursecompetency {
    /// courseid
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// competencyid
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// ruleoutcome
    #[serde(rename = "ruleoutcome")]
    pub r#ruleoutcome: Option<i64>,
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
pub struct ReturnsCompetenciesItemCoursemodulesItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// iconurl
    #[serde(rename = "iconurl")]
    pub r#iconurl: Option<String>,
}

pub type r#ReturnsCompetenciesItemCoursemodules = Vec<ReturnsCompetenciesItemCoursemodulesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetenciesItemUsercompetencycourse {
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
pub struct ReturnsCompetenciesItemRuleoutcomeoptionsItem {
    /// The option value
    #[serde(rename = "value")]
    pub r#value: Option<i64>,
    /// The name of the option
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// If this is the currently selected option
    #[serde(rename = "selected")]
    pub r#selected: Option<bool>,
}

pub type r#ReturnsCompetenciesItemRuleoutcomeoptions =
    Vec<ReturnsCompetenciesItemRuleoutcomeoptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetenciesItemComppathAncestorsItem {
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
pub type r#ReturnsCompetenciesItemComppathAncestors =
    Vec<ReturnsCompetenciesItemComppathAncestorsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetenciesItemComppathFramework {
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
pub struct ReturnsCompetenciesItemComppath {
    /// ancestors
    #[serde(rename = "ancestors")]
    pub r#ancestors: Option<r#ReturnsCompetenciesItemComppathAncestors>,
    #[serde(rename = "framework")]
    pub r#framework: Option<ReturnsCompetenciesItemComppathFramework>,
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
pub struct ReturnsCompetenciesItemPlansItemCommentarea {
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
pub struct ReturnsCompetenciesItemPlansItemReviewer {
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
pub struct ReturnsCompetenciesItemPlansItemTemplate {
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
pub struct ReturnsCompetenciesItemPlansItem {
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
    pub r#commentarea: Option<ReturnsCompetenciesItemPlansItemCommentarea>,
    #[serde(rename = "reviewer")]
    pub r#reviewer: Option<ReturnsCompetenciesItemPlansItemReviewer>,
    #[serde(rename = "template")]
    pub r#template: Option<ReturnsCompetenciesItemPlansItemTemplate>,
    /// url
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

pub type r#ReturnsCompetenciesItemPlans = Vec<ReturnsCompetenciesItemPlansItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetenciesItem {
    #[serde(rename = "competency")]
    pub r#competency: Option<ReturnsCompetenciesItemCompetency>,
    #[serde(rename = "coursecompetency")]
    pub r#coursecompetency: Option<ReturnsCompetenciesItemCoursecompetency>,
    #[serde(rename = "coursemodules")]
    pub r#coursemodules: Option<r#ReturnsCompetenciesItemCoursemodules>,
    #[serde(rename = "usercompetencycourse")]
    pub r#usercompetencycourse: Option<ReturnsCompetenciesItemUsercompetencycourse>,
    #[serde(rename = "ruleoutcomeoptions")]
    pub r#ruleoutcomeoptions: Option<r#ReturnsCompetenciesItemRuleoutcomeoptions>,
    #[serde(rename = "comppath")]
    pub r#comppath: Option<ReturnsCompetenciesItemComppath>,
    #[serde(rename = "plans")]
    pub r#plans: Option<r#ReturnsCompetenciesItemPlans>,
}

pub type r#ReturnsCompetencies = Vec<ReturnsCompetenciesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// The current course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// The current page context ID.
    #[serde(rename = "pagecontextid")]
    pub r#pagecontextid: Option<i64>,
    /// Current user id, if the user is a gradable user.
    #[serde(rename = "gradableuserid")]
    pub r#gradableuserid: Option<i64>,
    /// User can manage competency frameworks
    #[serde(rename = "canmanagecompetencyframeworks")]
    pub r#canmanagecompetencyframeworks: Option<bool>,
    /// User can manage linked course competencies
    #[serde(rename = "canmanagecoursecompetencies")]
    pub r#canmanagecoursecompetencies: Option<bool>,
    /// User can configure course competency settings
    #[serde(rename = "canconfigurecoursecompetencies")]
    pub r#canconfigurecoursecompetencies: Option<bool>,
    /// User can grade competencies.
    #[serde(rename = "cangradecompetencies")]
    pub r#cangradecompetencies: Option<bool>,
    #[serde(rename = "settings")]
    pub r#settings: Option<ReturnsSettings>,
    #[serde(rename = "statistics")]
    pub r#statistics: Option<ReturnsStatistics>,
    #[serde(rename = "competencies")]
    pub r#competencies: Option<r#ReturnsCompetencies>,
    /// Url to the manage competencies page.
    #[serde(rename = "manageurl")]
    pub r#manageurl: Option<String>,
    /// Url to the course competencies page.
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_data_for_course_competencies_page", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_data_for_course_competencies_page", params)
        .await
}
