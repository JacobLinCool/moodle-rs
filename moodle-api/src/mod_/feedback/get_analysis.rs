use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Feedback instance id
    #[serde(rename = "feedbackid")]
    pub r#feedbackid: Option<i64>,
    /// Group id, 0 means that the function will determine the user group
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Course where user completes the feedback (for site feedbacks only).
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemsdataItemItemItemfilesItem {
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

/// itemfiles
pub type r#ReturnsItemsdataItemItemItemfiles = Vec<ReturnsItemsdataItemItemItemfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemsdataItemItem {
    /// The record id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The feedback instance id this records belongs to.
    #[serde(rename = "feedback")]
    pub r#feedback: Option<i64>,
    /// If it belogns to a template, the template id.
    #[serde(rename = "template")]
    pub r#template: Option<i64>,
    /// The item name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The item label.
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// The text describing the item or the available possible answers.
    #[serde(rename = "presentation")]
    pub r#presentation: Option<String>,
    /// The type of the item.
    #[serde(rename = "typ")]
    pub r#typ: Option<String>,
    /// Whether it has a value or not.
    #[serde(rename = "hasvalue")]
    pub r#hasvalue: Option<i64>,
    /// The position in the list of questions.
    #[serde(rename = "position")]
    pub r#position: Option<i64>,
    /// Whether is a item (question) required or not.
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// The item id this item depend on.
    #[serde(rename = "dependitem")]
    pub r#dependitem: Option<i64>,
    /// The depend value.
    #[serde(rename = "dependvalue")]
    pub r#dependvalue: Option<String>,
    /// Different additional settings for the item (question).
    #[serde(rename = "options")]
    pub r#options: Option<String>,
    /// itemfiles
    #[serde(rename = "itemfiles")]
    pub r#itemfiles: Option<r#ReturnsItemsdataItemItemItemfiles>,
    /// The item position number
    #[serde(rename = "itemnumber")]
    pub r#itemnumber: Option<i64>,
    /// Additional data that may be required by external functions
    #[serde(rename = "otherdata")]
    pub r#otherdata: Option<String>,
}

pub type r#ReturnsItemsdataItemData = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemsdataItem {
    #[serde(rename = "item")]
    pub r#item: Option<ReturnsItemsdataItemItem>,
    #[serde(rename = "data")]
    pub r#data: Option<r#ReturnsItemsdataItemData>,
}

pub type r#ReturnsItemsdata = Vec<ReturnsItemsdataItem>;

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
    /// Number of completed submissions.
    #[serde(rename = "completedcount")]
    pub r#completedcount: Option<i64>,
    /// Number of items (questions).
    #[serde(rename = "itemscount")]
    pub r#itemscount: Option<i64>,
    #[serde(rename = "itemsdata")]
    pub r#itemsdata: Option<r#ReturnsItemsdata>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_feedback_get_analysis", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_feedback_get_analysis", params).await
}
