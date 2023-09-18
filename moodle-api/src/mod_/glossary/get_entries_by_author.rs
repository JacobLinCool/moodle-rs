use serde::{self, Deserialize, Serialize};

/// An array of options
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptions {
    /// When false, includes the non-approved entries created by the user. When true, also includes the ones that the user has the permission to approve.
    #[serde(rename = "includenotapproved")]
    pub r#includenotapproved: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Glossary entry ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// First letter of firstname or lastname, or either keywords: 'ALL' or 'SPECIAL'.
    #[serde(rename = "letter")]
    pub r#letter: Option<String>,
    /// Search and order using: 'FIRSTNAME' or 'LASTNAME'
    #[serde(rename = "field")]
    pub r#field: Option<String>,
    /// The direction of the order: 'ASC' or 'DESC'
    #[serde(rename = "sort")]
    pub r#sort: Option<String>,
    /// Start returning records from here
    #[serde(rename = "from")]
    pub r#from: Option<i64>,
    /// Number of records to return
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
    /// An array of options
    #[serde(rename = "options")]
    pub r#options: Option<ParamsOptions>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItemDefinitioninlinefilesItem {
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

/// entry definition inline files
pub type r#ReturnsEntriesItemDefinitioninlinefiles =
    Vec<ReturnsEntriesItemDefinitioninlinefilesItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItemAttachmentsItem {
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

/// attachments
pub type r#ReturnsEntriesItemAttachments = Vec<ReturnsEntriesItemAttachmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItemTagsItem {
    /// Tag id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Tag name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The raw, unnormalised name for the tag as entered by users.
    #[serde(rename = "rawname")]
    pub r#rawname: Option<String>,
    /// Whether this tag is standard.
    #[serde(rename = "isstandard")]
    pub r#isstandard: Option<bool>,
    /// Tag collection id.
    #[serde(rename = "tagcollid")]
    pub r#tagcollid: Option<i64>,
    /// Tag instance id.
    #[serde(rename = "taginstanceid")]
    pub r#taginstanceid: Option<i64>,
    /// Context the tag instance belongs to.
    #[serde(rename = "taginstancecontextid")]
    pub r#taginstancecontextid: Option<i64>,
    /// Id of the record tagged.
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// Tag ordering.
    #[serde(rename = "ordering")]
    pub r#ordering: Option<i64>,
    /// Whether the tag is flagged as inappropriate.
    #[serde(rename = "flag")]
    pub r#flag: Option<i64>,
}

/// Tags
pub type r#ReturnsEntriesItemTags = Vec<ReturnsEntriesItemTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItem {
    /// The entry ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The glossary ID
    #[serde(rename = "glossaryid")]
    pub r#glossaryid: Option<i64>,
    /// Author ID
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Author full name
    #[serde(rename = "userfullname")]
    pub r#userfullname: Option<String>,
    /// Author picture
    #[serde(rename = "userpictureurl")]
    pub r#userpictureurl: Option<String>,
    /// The concept
    #[serde(rename = "concept")]
    pub r#concept: Option<String>,
    /// The definition
    #[serde(rename = "definition")]
    pub r#definition: Option<String>,
    /// definition format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "definitionformat")]
    pub r#definitionformat: Option<i64>,
    /// The definition trust flag
    #[serde(rename = "definitiontrust")]
    pub r#definitiontrust: Option<bool>,
    /// entry definition inline files
    #[serde(rename = "definitioninlinefiles")]
    pub r#definitioninlinefiles: Option<r#ReturnsEntriesItemDefinitioninlinefiles>,
    /// Whether or not the entry has attachments
    #[serde(rename = "attachment")]
    pub r#attachment: Option<bool>,
    /// attachments
    #[serde(rename = "attachments")]
    pub r#attachments: Option<r#ReturnsEntriesItemAttachments>,
    /// Time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// The entry was created by a teacher, or equivalent.
    #[serde(rename = "teacherentry")]
    pub r#teacherentry: Option<bool>,
    /// The source glossary ID
    #[serde(rename = "sourceglossaryid")]
    pub r#sourceglossaryid: Option<i64>,
    /// Whether the concept should be automatically linked
    #[serde(rename = "usedynalink")]
    pub r#usedynalink: Option<bool>,
    /// When true, the matching is case sensitive
    #[serde(rename = "casesensitive")]
    pub r#casesensitive: Option<bool>,
    /// When true, the matching is done on full words only
    #[serde(rename = "fullmatch")]
    pub r#fullmatch: Option<bool>,
    /// Whether the entry was approved
    #[serde(rename = "approved")]
    pub r#approved: Option<bool>,
    /// Tags
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsEntriesItemTags>,
}

pub type r#ReturnsEntries = Vec<ReturnsEntriesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsRatinginfoScalesItemItemsItem {
    /// Scale value/option id.
    #[serde(rename = "value")]
    pub r#value: Option<i64>,
    /// Scale name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

/// Scale items. Only returned for not numerical scales.
pub type r#ReturnsRatinginfoScalesItemItems = Vec<ReturnsRatinginfoScalesItemItemsItem>;

/// Scale information
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsRatinginfoScalesItem {
    /// Scale id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course id.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Scale name (when a real scale is used).
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Max value for the scale.
    #[serde(rename = "max")]
    pub r#max: Option<i64>,
    /// Whether is a numeric scale.
    #[serde(rename = "isnumeric")]
    pub r#isnumeric: Option<bool>,
    /// Scale items. Only returned for not numerical scales.
    #[serde(rename = "items")]
    pub r#items: Option<r#ReturnsRatinginfoScalesItemItems>,
}

/// Different scales used information
pub type r#ReturnsRatinginfoScales = Vec<ReturnsRatinginfoScalesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsRatinginfoRatingsItem {
    /// Item id.
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// Scale id.
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// User who rated id.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Aggregated ratings grade.
    #[serde(rename = "aggregate")]
    pub r#aggregate: Option<f64>,
    /// Aggregated ratings as string.
    #[serde(rename = "aggregatestr")]
    pub r#aggregatestr: Option<String>,
    /// The aggregation label.
    #[serde(rename = "aggregatelabel")]
    pub r#aggregatelabel: Option<String>,
    /// Ratings count (used when aggregating).
    #[serde(rename = "count")]
    pub r#count: Option<i64>,
    /// The rating the user gave.
    #[serde(rename = "rating")]
    pub r#rating: Option<i64>,
    /// Whether the user can rate the item.
    #[serde(rename = "canrate")]
    pub r#canrate: Option<bool>,
    /// Whether the user can view the aggregated grade.
    #[serde(rename = "canviewaggregate")]
    pub r#canviewaggregate: Option<bool>,
}

/// The ratings
pub type r#ReturnsRatinginfoRatings = Vec<ReturnsRatinginfoRatingsItem>;

/// Rating information
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsRatinginfo {
    /// Context id.
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// Context name.
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Rating area name.
    #[serde(rename = "ratingarea")]
    pub r#ratingarea: Option<String>,
    /// Whether the user can view all the individual ratings.
    #[serde(rename = "canviewall")]
    pub r#canviewall: Option<bool>,
    /// Whether the user can view aggregate of ratings of others.
    #[serde(rename = "canviewany")]
    pub r#canviewany: Option<bool>,
    /// Different scales used information
    #[serde(rename = "scales")]
    pub r#scales: Option<r#ReturnsRatinginfoScales>,
    /// The ratings
    #[serde(rename = "ratings")]
    pub r#ratings: Option<r#ReturnsRatinginfoRatings>,
}

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
    /// The total number of records matching the request.
    #[serde(rename = "count")]
    pub r#count: Option<i64>,
    #[serde(rename = "entries")]
    pub r#entries: Option<r#ReturnsEntries>,
    /// Rating information
    #[serde(rename = "ratinginfo")]
    pub r#ratinginfo: Option<ReturnsRatinginfo>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_glossary_get_entries_by_author", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_glossary_get_entries_by_author", params)
        .await
}
