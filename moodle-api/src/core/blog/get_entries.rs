use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFiltersItem {
    /// The expected keys (value format) are: tag      PARAM_NOTAGS blog tag tagid    PARAM_INT    blog tag id userid   PARAM_INT    blog author (userid) cmid    PARAM_INT    course module id entryid  PARAM_INT    entry id groupid  PARAM_INT    group id courseid PARAM_INT    course id search   PARAM_RAW    search term
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the filter.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Parameters to filter blog listings.
pub type r#ParamsFilters = Vec<ParamsFiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Parameters to filter blog listings.
    #[serde(rename = "filters")]
    pub r#filters: Option<r#ParamsFilters>,
    /// The blog page to return.
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The number of posts to return per page.
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItemSummaryfilesItem {
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

/// summaryfiles
pub type r#ReturnsEntriesItemSummaryfiles = Vec<ReturnsEntriesItemSummaryfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEntriesItemAttachmentfilesItem {
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

/// attachmentfiles
pub type r#ReturnsEntriesItemAttachmentfiles = Vec<ReturnsEntriesItemAttachmentfilesItem>;

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
    /// Post/entry id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Where it was published the post (blog, blog_external...).
    #[serde(rename = "module")]
    pub r#module: Option<String>,
    /// Post author.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Course where the post was created.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Group post was created for.
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Module id where the post was created (not used anymore).
    #[serde(rename = "moduleid")]
    pub r#moduleid: Option<i64>,
    /// Course module id where the post was created.
    #[serde(rename = "coursemoduleid")]
    pub r#coursemoduleid: Option<i64>,
    /// Post subject.
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// Post summary.
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// Post content.
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// Post unique hash.
    #[serde(rename = "uniquehash")]
    pub r#uniquehash: Option<String>,
    /// Post rating.
    #[serde(rename = "rating")]
    pub r#rating: Option<i64>,
    /// Post content format.
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// Post atachment.
    #[serde(rename = "attachment")]
    pub r#attachment: Option<String>,
    /// Post publish state.
    #[serde(rename = "publishstate")]
    pub r#publishstate: Option<String>,
    /// When it was last modified.
    #[serde(rename = "lastmodified")]
    pub r#lastmodified: Option<i64>,
    /// When it was created.
    #[serde(rename = "created")]
    pub r#created: Option<i64>,
    /// User that updated the post.
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// summaryfiles
    #[serde(rename = "summaryfiles")]
    pub r#summaryfiles: Option<r#ReturnsEntriesItemSummaryfiles>,
    /// attachmentfiles
    #[serde(rename = "attachmentfiles")]
    pub r#attachmentfiles: Option<r#ReturnsEntriesItemAttachmentfiles>,
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
    /// The total number of entries found.
    #[serde(rename = "totalentries")]
    pub r#totalentries: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_blog_get_entries", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_blog_get_entries", params).await
}
