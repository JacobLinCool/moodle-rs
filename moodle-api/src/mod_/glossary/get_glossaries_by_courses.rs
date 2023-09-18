use serde::{self, Deserialize, Serialize};

/// Array of course IDs
pub type r#ParamsCourseids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of course IDs
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsCourseids>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGlossariesItemIntrofilesItem {
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
    /// The repository type for external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// Files in the introduction
pub type r#ReturnsGlossariesItemIntrofiles = Vec<ReturnsGlossariesItemIntrofilesItem>;

pub type r#ReturnsGlossariesItemBrowsemodes = Vec<String>;

/// Glossaries
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGlossariesItem {
    /// Activity instance id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course module id
    #[serde(rename = "coursemodule")]
    pub r#coursemodule: Option<i64>,
    /// Course id
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// Activity name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Activity introduction
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// Files in the introduction
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsGlossariesItemIntrofiles>,
    /// Course section id
    #[serde(rename = "section")]
    pub r#section: Option<i64>,
    /// Visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// Group mode
    #[serde(rename = "groupmode")]
    pub r#groupmode: Option<i64>,
    /// Group id
    #[serde(rename = "groupingid")]
    pub r#groupingid: Option<i64>,
    /// Forced activity language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// If enabled, multiple entries can have the same concept name
    #[serde(rename = "allowduplicatedentries")]
    pub r#allowduplicatedentries: Option<i64>,
    /// Display format type
    #[serde(rename = "displayformat")]
    pub r#displayformat: Option<String>,
    /// If enabled this glossary is a main glossary.
    #[serde(rename = "mainglossary")]
    pub r#mainglossary: Option<i64>,
    /// If enabled, participants can browse the glossary by special characters, such as @ and #
    #[serde(rename = "showspecial")]
    pub r#showspecial: Option<i64>,
    /// If enabled, participants can browse the glossary by letters of the alphabet
    #[serde(rename = "showalphabet")]
    pub r#showalphabet: Option<i64>,
    /// If enabled, participants can browse all entries at once
    #[serde(rename = "showall")]
    pub r#showall: Option<i64>,
    /// If enabled, all participants with permission to create comments will be able to add comments to glossary entries
    #[serde(rename = "allowcomments")]
    pub r#allowcomments: Option<i64>,
    /// If enabled, students are provided with a link to a printer-friendly version of the glossary. The link is always available to teachers
    #[serde(rename = "allowprintview")]
    pub r#allowprintview: Option<i64>,
    /// If site-wide glossary auto-linking has been enabled by an administrator and this checkbox is ticked, the entry will be automatically linked wherever the concept words and phrases appear throughout the rest of the course.
    #[serde(rename = "usedynalink")]
    pub r#usedynalink: Option<i64>,
    /// If set to no, entries require approving by a teacher before they are viewable by everyone.
    #[serde(rename = "defaultapproval")]
    pub r#defaultapproval: Option<i64>,
    /// When approving glossary items you may wish to use a different display format
    #[serde(rename = "approvaldisplayformat")]
    pub r#approvaldisplayformat: Option<String>,
    #[serde(rename = "globalglossary")]
    pub r#globalglossary: Option<i64>,
    /// Entries shown per page
    #[serde(rename = "entbypage")]
    pub r#entbypage: Option<i64>,
    /// Always allow editing
    #[serde(rename = "editalways")]
    pub r#editalways: Option<i64>,
    /// To enable the RSS feed for this activity, select either concepts with author or concepts without author to be included in the feed
    #[serde(rename = "rsstype")]
    pub r#rsstype: Option<i64>,
    /// This setting specifies the number of glossary entry concepts to include in the RSS feed. Between 5 and 20 generally acceptable
    #[serde(rename = "rssarticles")]
    pub r#rssarticles: Option<i64>,
    /// Aggregate type
    #[serde(rename = "assessed")]
    pub r#assessed: Option<i64>,
    /// Restrict rating to items created after this
    #[serde(rename = "assesstimestart")]
    pub r#assesstimestart: Option<i64>,
    /// Restrict rating to items created before this
    #[serde(rename = "assesstimefinish")]
    pub r#assesstimefinish: Option<i64>,
    /// Scale ID
    #[serde(rename = "scale")]
    pub r#scale: Option<i64>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Number of entries to complete
    #[serde(rename = "completionentries")]
    pub r#completionentries: Option<i64>,
    #[serde(rename = "browsemodes")]
    pub r#browsemodes: Option<r#ReturnsGlossariesItemBrowsemodes>,
    /// Whether the user can add a new entry
    #[serde(rename = "canaddentry")]
    pub r#canaddentry: Option<i64>,
}

pub type r#ReturnsGlossaries = Vec<ReturnsGlossariesItem>;

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
    #[serde(rename = "glossaries")]
    pub r#glossaries: Option<r#ReturnsGlossaries>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_glossary_get_glossaries_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_glossary_get_glossaries_by_courses", params)
        .await
}
