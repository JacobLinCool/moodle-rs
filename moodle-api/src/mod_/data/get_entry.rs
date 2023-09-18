use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// record entry id
    #[serde(rename = "entryid")]
    pub r#entryid: Option<i64>,
    /// Whether to return contents or not.
    #[serde(rename = "returncontents")]
    pub r#returncontents: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntryContentsItemFilesItem {
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

/// files
pub type r#ReturnsEntryContentsItemFiles = Vec<ReturnsEntryContentsItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntryContentsItem {
    /// Content id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The field type of the content.
    #[serde(rename = "fieldid")]
    pub r#fieldid: Option<i64>,
    /// The record this content belongs to.
    #[serde(rename = "recordid")]
    pub r#recordid: Option<i64>,
    /// Contents.
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// Contents.
    #[serde(rename = "content1")]
    pub r#content1: Option<String>,
    /// Contents.
    #[serde(rename = "content2")]
    pub r#content2: Option<String>,
    /// Contents.
    #[serde(rename = "content3")]
    pub r#content3: Option<String>,
    /// Contents.
    #[serde(rename = "content4")]
    pub r#content4: Option<String>,
    /// files
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsEntryContentsItemFiles>,
}

/// The record contents.
pub type r#ReturnsEntryContents = Vec<ReturnsEntryContentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntryTagsItem {
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

/// Tags.
pub type r#ReturnsEntryTags = Vec<ReturnsEntryTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntry {
    /// Record id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The id of the user who created the record.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The group id this record belongs to (0 for no groups).
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// The database id this record belongs to.
    #[serde(rename = "dataid")]
    pub r#dataid: Option<i64>,
    /// Time the record was created.
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Last time the record was modified.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Whether the entry has been approved (if the database is configured in that way).
    #[serde(rename = "approved")]
    pub r#approved: Option<bool>,
    /// Whether the current user can manage this entry
    #[serde(rename = "canmanageentry")]
    pub r#canmanageentry: Option<bool>,
    /// The user who created the entry fullname.
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// The record contents.
    #[serde(rename = "contents")]
    pub r#contents: Option<r#ReturnsEntryContents>,
    /// Tags.
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsEntryTags>,
}

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
    #[serde(rename = "entry")]
    pub r#entry: Option<ReturnsEntry>,
    /// The entry as is rendered in the site.
    #[serde(rename = "entryviewcontents")]
    pub r#entryviewcontents: Option<String>,
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
    let json = client.post("mod_data_get_entry", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_data_get_entry", params).await
}
