use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The competency id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
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

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_list_courses_using_competency", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_list_courses_using_competency", params)
        .await
}
