use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPagecontext {
    /// Context ID. Either use this value, or level and instanceid.
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// Context level. To be used with instanceid.
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// Context instance ID. To be used with level
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The template id
    #[serde(rename = "templateid")]
    pub r#templateid: Option<i64>,
    #[serde(rename = "pagecontext")]
    pub r#pagecontext: Option<ParamsPagecontext>,
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
pub struct ReturnsCompetenciesItemLinkedcoursesItem {
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
pub type r#ReturnsCompetenciesItemLinkedcourses = Vec<ReturnsCompetenciesItemLinkedcoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetenciesItemRelatedcompetenciesItem {
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
pub type r#ReturnsCompetenciesItemRelatedcompetencies =
    Vec<ReturnsCompetenciesItemRelatedcompetenciesItem>;

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
pub struct ReturnsCompetenciesItemFramework {
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
pub struct ReturnsCompetenciesItem {
    /// linkedcourses
    #[serde(rename = "linkedcourses")]
    pub r#linkedcourses: Option<r#ReturnsCompetenciesItemLinkedcourses>,
    /// relatedcompetencies
    #[serde(rename = "relatedcompetencies")]
    pub r#relatedcompetencies: Option<r#ReturnsCompetenciesItemRelatedcompetencies>,
    #[serde(rename = "competency")]
    pub r#competency: Option<ReturnsCompetenciesItemCompetency>,
    #[serde(rename = "framework")]
    pub r#framework: Option<ReturnsCompetenciesItemFramework>,
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
    pub r#comppath: Option<ReturnsCompetenciesItemComppath>,
    /// pluginbaseurl
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
}

pub type r#ReturnsCompetencies = Vec<ReturnsCompetenciesItem>;

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
    /// unlinkedcompetencycount
    #[serde(rename = "unlinkedcompetencycount")]
    pub r#unlinkedcompetencycount: Option<i64>,
    /// plancount
    #[serde(rename = "plancount")]
    pub r#plancount: Option<i64>,
    /// completedplancount
    #[serde(rename = "completedplancount")]
    pub r#completedplancount: Option<i64>,
    /// usercompetencyplancount
    #[serde(rename = "usercompetencyplancount")]
    pub r#usercompetencyplancount: Option<i64>,
    /// proficientusercompetencyplancount
    #[serde(rename = "proficientusercompetencyplancount")]
    pub r#proficientusercompetencyplancount: Option<i64>,
    /// linkedcompetencypercentage
    #[serde(rename = "linkedcompetencypercentage")]
    pub r#linkedcompetencypercentage: Option<f64>,
    /// linkedcompetencypercentageformatted
    #[serde(rename = "linkedcompetencypercentageformatted")]
    pub r#linkedcompetencypercentageformatted: Option<String>,
    /// linkedcompetencycount
    #[serde(rename = "linkedcompetencycount")]
    pub r#linkedcompetencycount: Option<i64>,
    /// completedplanpercentage
    #[serde(rename = "completedplanpercentage")]
    pub r#completedplanpercentage: Option<f64>,
    /// completedplanpercentageformatted
    #[serde(rename = "completedplanpercentageformatted")]
    pub r#completedplanpercentageformatted: Option<String>,
    /// proficientusercompetencyplanpercentage
    #[serde(rename = "proficientusercompetencyplanpercentage")]
    pub r#proficientusercompetencyplanpercentage: Option<f64>,
    /// proficientusercompetencyplanpercentageformatted
    #[serde(rename = "proficientusercompetencyplanpercentageformatted")]
    pub r#proficientusercompetencyplanpercentageformatted: Option<String>,
    /// leastproficient
    #[serde(rename = "leastproficient")]
    pub r#leastproficient: Option<r#ReturnsStatisticsLeastproficient>,
    /// leastproficientcount
    #[serde(rename = "leastproficientcount")]
    pub r#leastproficientcount: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "template")]
    pub r#template: Option<ReturnsTemplate>,
    /// Context ID
    #[serde(rename = "pagecontextid")]
    pub r#pagecontextid: Option<i64>,
    /// User can manage competency frameworks
    #[serde(rename = "canmanagecompetencyframeworks")]
    pub r#canmanagecompetencyframeworks: Option<bool>,
    /// User can manage learning plan templates
    #[serde(rename = "canmanagetemplatecompetencies")]
    pub r#canmanagetemplatecompetencies: Option<bool>,
    #[serde(rename = "competencies")]
    pub r#competencies: Option<r#ReturnsCompetencies>,
    /// Url to the manage competencies page.
    #[serde(rename = "manageurl")]
    pub r#manageurl: Option<String>,
    /// Base URL of the plugin.
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
    #[serde(rename = "statistics")]
    pub r#statistics: Option<ReturnsStatistics>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_data_for_template_competencies_page", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_data_for_template_competencies_page", params)
        .await
}
