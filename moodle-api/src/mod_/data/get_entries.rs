use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// data instance id
    #[serde(rename = "databaseid")]
    pub r#databaseid: Option<i64>,
    /// Group id, 0 means that the function will determine the user group
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Whether to return contents or not. This will return each entry raw contents and the complete list view (using the template).
    #[serde(rename = "returncontents")]
    pub r#returncontents: Option<bool>,
    /// Sort the records by this field id, reserved ids are: 0: timeadded -1: firstname -2: lastname -3: approved -4: timemodified. Empty for using the default database setting.
    #[serde(rename = "sort")]
    pub r#sort: Option<i64>,
    /// The direction of the sorting: 'ASC' or 'DESC'. Empty for using the default database setting.
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    /// The page of records to return.
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The number of records to return per page
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItemContentsItemFilesItem {
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
pub type r#ReturnsEntriesItemContentsItemFiles = Vec<ReturnsEntriesItemContentsItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItemContentsItem {
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
    pub r#files: Option<r#ReturnsEntriesItemContentsItemFiles>,
}

/// The record contents.
pub type r#ReturnsEntriesItemContents = Vec<ReturnsEntriesItemContentsItem>;

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

/// Tags.
pub type r#ReturnsEntriesItemTags = Vec<ReturnsEntriesItemTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItem {
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
    pub r#contents: Option<r#ReturnsEntriesItemContents>,
    /// Tags.
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsEntriesItemTags>,
}

pub type r#ReturnsEntries = Vec<ReturnsEntriesItem>;

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
    #[serde(rename = "entries")]
    pub r#entries: Option<r#ReturnsEntries>,
    /// Total count of records.
    #[serde(rename = "totalcount")]
    pub r#totalcount: Option<i64>,
    /// Total size (bytes) of the files included in the records.
    #[serde(rename = "totalfilesize")]
    pub r#totalfilesize: Option<i64>,
    /// The list view contents as is rendered in the site.
    #[serde(rename = "listviewcontents")]
    pub r#listviewcontents: Option<String>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_data_get_entries", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_data_get_entries", params).await
}
