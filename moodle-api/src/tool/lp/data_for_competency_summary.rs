use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The competency id
    #[serde(rename = "competencyid")]
    pub r#competencyid: Option<i64>,
    /// Include or not related competencies
    #[serde(rename = "includerelated")]
    pub r#includerelated: Option<bool>,
    /// Include or not competency courses
    #[serde(rename = "includecourses")]
    pub r#includecourses: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsLinkedcoursesItem {
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
pub type r#ReturnsLinkedcourses = Vec<ReturnsLinkedcoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsRelatedcompetenciesItem {
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
pub type r#ReturnsRelatedcompetencies = Vec<ReturnsRelatedcompetenciesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetency {
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
pub struct ReturnsFramework {
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
pub struct ReturnsComppathAncestorsItem {
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
pub type r#ReturnsComppathAncestors = Vec<ReturnsComppathAncestorsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsComppathFramework {
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
pub struct ReturnsComppath {
    /// ancestors
    #[serde(rename = "ancestors")]
    pub r#ancestors: Option<r#ReturnsComppathAncestors>,
    #[serde(rename = "framework")]
    pub r#framework: Option<ReturnsComppathFramework>,
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
pub struct Returns {
    /// linkedcourses
    #[serde(rename = "linkedcourses")]
    pub r#linkedcourses: Option<r#ReturnsLinkedcourses>,
    /// relatedcompetencies
    #[serde(rename = "relatedcompetencies")]
    pub r#relatedcompetencies: Option<r#ReturnsRelatedcompetencies>,
    #[serde(rename = "competency")]
    pub r#competency: Option<ReturnsCompetency>,
    #[serde(rename = "framework")]
    pub r#framework: Option<ReturnsFramework>,
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
    pub r#comppath: Option<ReturnsComppath>,
    /// pluginbaseurl
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_data_for_competency_summary", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_data_for_competency_summary", params)
        .await
}
