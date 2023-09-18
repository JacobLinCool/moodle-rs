use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The ID of the discussion from which to fetch posts.
    #[serde(rename = "discussionid")]
    pub r#discussionid: Option<i64>,
    /// Sort by this element: id, created or modified
    #[serde(rename = "sortby")]
    pub r#sortby: Option<String>,
    /// Sort direction: ASC or DESC
    #[serde(rename = "sortdirection")]
    pub r#sortdirection: Option<String>,
    /// Whether inline attachments should be included or not
    #[serde(rename = "includeinlineattachments")]
    pub r#includeinlineattachments: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemAuthorGroupsItemUrls {
    /// image
    #[serde(rename = "image")]
    pub r#image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemAuthorGroupsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsPostsItemAuthorGroupsItemUrls>,
}

/// groups
pub type r#ReturnsPostsItemAuthorGroups = Vec<ReturnsPostsItemAuthorGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemAuthorUrls {
    /// The URL for the use profile page
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// The URL for the use profile image
    #[serde(rename = "profileimage")]
    pub r#profileimage: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemAuthor {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// fullname
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// isdeleted
    #[serde(rename = "isdeleted")]
    pub r#isdeleted: Option<bool>,
    /// groups
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ReturnsPostsItemAuthorGroups>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsPostsItemAuthorUrls>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemCapabilities {
    /// Whether the user can view the post
    #[serde(rename = "view")]
    pub r#view: Option<bool>,
    /// Whether the user can edit the post
    #[serde(rename = "edit")]
    pub r#edit: Option<bool>,
    /// Whether the user can delete the post
    #[serde(rename = "delete")]
    pub r#delete: Option<bool>,
    /// Whether the user can split the post
    #[serde(rename = "split")]
    pub r#split: Option<bool>,
    /// Whether the user can reply to the post
    #[serde(rename = "reply")]
    pub r#reply: Option<bool>,
    /// Whether the user can self enrol into the course
    #[serde(rename = "selfenrol")]
    pub r#selfenrol: Option<bool>,
    /// Whether the user can export the post
    #[serde(rename = "export")]
    pub r#export: Option<bool>,
    /// Whether the user can control the read status of the post
    #[serde(rename = "controlreadstatus")]
    pub r#controlreadstatus: Option<bool>,
    /// Whether the user can post a private reply
    #[serde(rename = "canreplyprivately")]
    pub r#canreplyprivately: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemUrls {
    /// The URL used to view the post
    #[serde(rename = "view")]
    pub r#view: Option<String>,
    /// The URL used to view the post in isolation
    #[serde(rename = "viewisolated")]
    pub r#viewisolated: Option<String>,
    /// The URL used to view the parent of the post
    #[serde(rename = "viewparent")]
    pub r#viewparent: Option<String>,
    /// The URL used to edit the post
    #[serde(rename = "edit")]
    pub r#edit: Option<String>,
    /// The URL used to delete the post
    #[serde(rename = "delete")]
    pub r#delete: Option<String>,
    /// The URL used to split the discussion with the selected post being the first post in the new discussion
    #[serde(rename = "split")]
    pub r#split: Option<String>,
    /// The URL used to reply to the post
    #[serde(rename = "reply")]
    pub r#reply: Option<String>,
    /// The URL used to export the post
    #[serde(rename = "export")]
    pub r#export: Option<String>,
    /// The URL used to mark the post as read
    #[serde(rename = "markasread")]
    pub r#markasread: Option<String>,
    /// The URL used to mark the post as unread
    #[serde(rename = "markasunread")]
    pub r#markasunread: Option<String>,
    /// discuss
    #[serde(rename = "discuss")]
    pub r#discuss: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemAttachmentsItemUrls {
    /// The URL used to export the attachment
    #[serde(rename = "export")]
    pub r#export: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemAttachmentsItemHtml {
    /// The HTML source for the Plagiarism Response
    #[serde(rename = "plagiarism")]
    pub r#plagiarism: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemAttachmentsItem {
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
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsPostsItemAttachmentsItemUrls>,
    #[serde(rename = "html")]
    pub r#html: Option<ReturnsPostsItemAttachmentsItemHtml>,
}

/// attachments
pub type r#ReturnsPostsItemAttachments = Vec<ReturnsPostsItemAttachmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemMessageinlinefilesItem {
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

/// messageinlinefiles
pub type r#ReturnsPostsItemMessageinlinefiles = Vec<ReturnsPostsItemMessageinlinefilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemTagsItemUrls {
    /// The URL to view the tag
    #[serde(rename = "view")]
    pub r#view: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemTagsItem {
    /// The ID of the Tag
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The tagid
    #[serde(rename = "tagid")]
    pub r#tagid: Option<i64>,
    /// Whether this is a standard tag
    #[serde(rename = "isstandard")]
    pub r#isstandard: Option<bool>,
    /// The display name of the tag
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// Wehther this tag is flagged
    #[serde(rename = "flag")]
    pub r#flag: Option<bool>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsPostsItemTagsItemUrls>,
}

/// tags
pub type r#ReturnsPostsItemTags = Vec<ReturnsPostsItemTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItemHtml {
    /// The HTML source to rate the post
    #[serde(rename = "rating")]
    pub r#rating: Option<String>,
    /// The HTML source to view the list of tags
    #[serde(rename = "taglist")]
    pub r#taglist: Option<String>,
    /// The HTML source to view the author details
    #[serde(rename = "authorsubheading")]
    pub r#authorsubheading: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// subject
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// replysubject
    #[serde(rename = "replysubject")]
    pub r#replysubject: Option<String>,
    /// message
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// message format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "messageformat")]
    pub r#messageformat: Option<i64>,
    #[serde(rename = "author")]
    pub r#author: Option<ReturnsPostsItemAuthor>,
    /// discussionid
    #[serde(rename = "discussionid")]
    pub r#discussionid: Option<i64>,
    /// hasparent
    #[serde(rename = "hasparent")]
    pub r#hasparent: Option<bool>,
    /// parentid
    #[serde(rename = "parentid")]
    pub r#parentid: Option<i64>,
    /// timecreated
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// unread
    #[serde(rename = "unread")]
    pub r#unread: Option<bool>,
    /// isdeleted
    #[serde(rename = "isdeleted")]
    pub r#isdeleted: Option<bool>,
    /// isprivatereply
    #[serde(rename = "isprivatereply")]
    pub r#isprivatereply: Option<bool>,
    /// haswordcount
    #[serde(rename = "haswordcount")]
    pub r#haswordcount: Option<bool>,
    /// wordcount
    #[serde(rename = "wordcount")]
    pub r#wordcount: Option<i64>,
    /// charcount
    #[serde(rename = "charcount")]
    pub r#charcount: Option<i64>,
    #[serde(rename = "capabilities")]
    pub r#capabilities: Option<ReturnsPostsItemCapabilities>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsPostsItemUrls>,
    /// attachments
    #[serde(rename = "attachments")]
    pub r#attachments: Option<r#ReturnsPostsItemAttachments>,
    /// messageinlinefiles
    #[serde(rename = "messageinlinefiles")]
    pub r#messageinlinefiles: Option<r#ReturnsPostsItemMessageinlinefiles>,
    /// tags
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsPostsItemTags>,
    #[serde(rename = "html")]
    pub r#html: Option<ReturnsPostsItemHtml>,
}

pub type r#ReturnsPosts = Vec<ReturnsPostsItem>;

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
    #[serde(rename = "posts")]
    pub r#posts: Option<r#ReturnsPosts>,
    /// The forum id
    #[serde(rename = "forumid")]
    pub r#forumid: Option<i64>,
    /// The forum course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
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
        .post("mod_forum_get_discussion_posts", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_forum_get_discussion_posts", params).await
}
