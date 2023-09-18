use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Post to fetch.
    #[serde(rename = "postid")]
    pub r#postid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostAuthorGroupsItemUrls {
    /// image
    #[serde(rename = "image")]
    pub r#image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostAuthorGroupsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsPostAuthorGroupsItemUrls>,
}

/// groups
pub type r#ReturnsPostAuthorGroups = Vec<ReturnsPostAuthorGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostAuthorUrls {
    /// The URL for the use profile page
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// The URL for the use profile image
    #[serde(rename = "profileimage")]
    pub r#profileimage: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostAuthor {
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
    pub r#groups: Option<r#ReturnsPostAuthorGroups>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsPostAuthorUrls>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostCapabilities {
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
pub struct ReturnsPostUrls {
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
pub struct ReturnsPostAttachmentsItemUrls {
    /// The URL used to export the attachment
    #[serde(rename = "export")]
    pub r#export: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostAttachmentsItemHtml {
    /// The HTML source for the Plagiarism Response
    #[serde(rename = "plagiarism")]
    pub r#plagiarism: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostAttachmentsItem {
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
    pub r#urls: Option<ReturnsPostAttachmentsItemUrls>,
    #[serde(rename = "html")]
    pub r#html: Option<ReturnsPostAttachmentsItemHtml>,
}

/// attachments
pub type r#ReturnsPostAttachments = Vec<ReturnsPostAttachmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostMessageinlinefilesItem {
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
pub type r#ReturnsPostMessageinlinefiles = Vec<ReturnsPostMessageinlinefilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostTagsItemUrls {
    /// The URL to view the tag
    #[serde(rename = "view")]
    pub r#view: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostTagsItem {
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
    pub r#urls: Option<ReturnsPostTagsItemUrls>,
}

/// tags
pub type r#ReturnsPostTags = Vec<ReturnsPostTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPostHtml {
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
pub struct ReturnsPost {
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
    pub r#author: Option<ReturnsPostAuthor>,
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
    pub r#capabilities: Option<ReturnsPostCapabilities>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsPostUrls>,
    /// attachments
    #[serde(rename = "attachments")]
    pub r#attachments: Option<r#ReturnsPostAttachments>,
    /// messageinlinefiles
    #[serde(rename = "messageinlinefiles")]
    pub r#messageinlinefiles: Option<r#ReturnsPostMessageinlinefiles>,
    /// tags
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsPostTags>,
    #[serde(rename = "html")]
    pub r#html: Option<ReturnsPostHtml>,
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
    #[serde(rename = "post")]
    pub r#post: Option<ReturnsPost>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_forum_get_discussion_post", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_forum_get_discussion_post", params).await
}
