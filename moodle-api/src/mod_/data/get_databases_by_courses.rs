use serde::{self, Deserialize, Serialize};

/// Array of course ids
pub type r#ParamsCourseids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of course ids
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsCourseids>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDatabasesItemIntrofilesItem {
    /// File name.
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// File path.
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// File size.
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// Downloadable file url.
    #[serde(rename = "fileurl")]
    pub r#fileurl: Option<String>,
    /// Time modified.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// File mime type.
    #[serde(rename = "mimetype")]
    pub r#mimetype: Option<String>,
    /// Whether is an external file.
    #[serde(rename = "isexternalfile")]
    pub r#isexternalfile: Option<bool>,
    /// The repository type for the external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// introfiles
pub type r#ReturnsDatabasesItemIntrofiles = Vec<ReturnsDatabasesItemIntrofilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDatabasesItem {
    /// Database id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course id
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// Database name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The Database intro
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// Forced activity language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// comments enabled
    #[serde(rename = "comments")]
    pub r#comments: Option<bool>,
    /// timeavailablefrom field
    #[serde(rename = "timeavailablefrom")]
    pub r#timeavailablefrom: Option<i64>,
    /// timeavailableto field
    #[serde(rename = "timeavailableto")]
    pub r#timeavailableto: Option<i64>,
    /// timeviewfrom field
    #[serde(rename = "timeviewfrom")]
    pub r#timeviewfrom: Option<i64>,
    /// timeviewto field
    #[serde(rename = "timeviewto")]
    pub r#timeviewto: Option<i64>,
    /// requiredentries field
    #[serde(rename = "requiredentries")]
    pub r#requiredentries: Option<i64>,
    /// requiredentriestoview field
    #[serde(rename = "requiredentriestoview")]
    pub r#requiredentriestoview: Option<i64>,
    /// maxentries field
    #[serde(rename = "maxentries")]
    pub r#maxentries: Option<i64>,
    /// rssarticles field
    #[serde(rename = "rssarticles")]
    pub r#rssarticles: Option<i64>,
    /// singletemplate field
    #[serde(rename = "singletemplate")]
    pub r#singletemplate: Option<String>,
    /// listtemplate field
    #[serde(rename = "listtemplate")]
    pub r#listtemplate: Option<String>,
    /// listtemplateheader field
    #[serde(rename = "listtemplateheader")]
    pub r#listtemplateheader: Option<String>,
    /// listtemplatefooter field
    #[serde(rename = "listtemplatefooter")]
    pub r#listtemplatefooter: Option<String>,
    /// addtemplate field
    #[serde(rename = "addtemplate")]
    pub r#addtemplate: Option<String>,
    /// rsstemplate field
    #[serde(rename = "rsstemplate")]
    pub r#rsstemplate: Option<String>,
    /// rsstitletemplate field
    #[serde(rename = "rsstitletemplate")]
    pub r#rsstitletemplate: Option<String>,
    /// csstemplate field
    #[serde(rename = "csstemplate")]
    pub r#csstemplate: Option<String>,
    /// jstemplate field
    #[serde(rename = "jstemplate")]
    pub r#jstemplate: Option<String>,
    /// asearchtemplate field
    #[serde(rename = "asearchtemplate")]
    pub r#asearchtemplate: Option<String>,
    /// approval field
    #[serde(rename = "approval")]
    pub r#approval: Option<bool>,
    /// manageapproved field
    #[serde(rename = "manageapproved")]
    pub r#manageapproved: Option<bool>,
    /// scale field
    #[serde(rename = "scale")]
    pub r#scale: Option<i64>,
    /// assessed field
    #[serde(rename = "assessed")]
    pub r#assessed: Option<i64>,
    /// assesstimestart field
    #[serde(rename = "assesstimestart")]
    pub r#assesstimestart: Option<i64>,
    /// assesstimefinish field
    #[serde(rename = "assesstimefinish")]
    pub r#assesstimefinish: Option<i64>,
    /// defaultsort field
    #[serde(rename = "defaultsort")]
    pub r#defaultsort: Option<i64>,
    /// defaultsortdir field
    #[serde(rename = "defaultsortdir")]
    pub r#defaultsortdir: Option<i64>,
    /// editany field (not used any more)
    #[serde(rename = "editany")]
    pub r#editany: Option<bool>,
    /// notification field (not used any more)
    #[serde(rename = "notification")]
    pub r#notification: Option<i64>,
    /// Time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// coursemodule
    #[serde(rename = "coursemodule")]
    pub r#coursemodule: Option<i64>,
    /// introfiles
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsDatabasesItemIntrofiles>,
}

pub type r#ReturnsDatabases = Vec<ReturnsDatabasesItem>;

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
    #[serde(rename = "databases")]
    pub r#databases: Option<r#ReturnsDatabases>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_data_get_databases_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_data_get_databases_by_courses", params)
        .await
}
