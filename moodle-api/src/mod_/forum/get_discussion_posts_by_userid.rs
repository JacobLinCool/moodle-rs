use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The ID of the user of whom to fetch posts.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The ID of the module of which to fetch items.
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// Sort by this element: id, created or modified
    #[serde(rename = "sortby")]
    pub r#sortby: Option<String>,
    /// Sort direction: ASC or DESC
    #[serde(rename = "sortdirection")]
    pub r#sortdirection: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemAuthorGroupsItemUrls {
    /// image
    #[serde(rename = "image")]
    pub r#image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemAuthorGroupsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsDiscussionsItemPostsUserpostsItemAuthorGroupsItemUrls>,
}

/// groups
pub type r#ReturnsDiscussionsItemPostsUserpostsItemAuthorGroups =
    Vec<ReturnsDiscussionsItemPostsUserpostsItemAuthorGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemAuthorUrls {
    /// The URL for the use profile page
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// The URL for the use profile image
    #[serde(rename = "profileimage")]
    pub r#profileimage: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemAuthor {
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
    pub r#groups: Option<r#ReturnsDiscussionsItemPostsUserpostsItemAuthorGroups>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsDiscussionsItemPostsUserpostsItemAuthorUrls>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemCapabilities {
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
pub struct ReturnsDiscussionsItemPostsUserpostsItemUrls {
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
pub struct ReturnsDiscussionsItemPostsUserpostsItemAttachmentsItemUrls {
    /// The URL used to export the attachment
    #[serde(rename = "export")]
    pub r#export: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemAttachmentsItemHtml {
    /// The HTML source for the Plagiarism Response
    #[serde(rename = "plagiarism")]
    pub r#plagiarism: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemAttachmentsItem {
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
    pub r#urls: Option<ReturnsDiscussionsItemPostsUserpostsItemAttachmentsItemUrls>,
    #[serde(rename = "html")]
    pub r#html: Option<ReturnsDiscussionsItemPostsUserpostsItemAttachmentsItemHtml>,
}

/// attachments
pub type r#ReturnsDiscussionsItemPostsUserpostsItemAttachments =
    Vec<ReturnsDiscussionsItemPostsUserpostsItemAttachmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemMessageinlinefilesItem {
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
pub type r#ReturnsDiscussionsItemPostsUserpostsItemMessageinlinefiles =
    Vec<ReturnsDiscussionsItemPostsUserpostsItemMessageinlinefilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemTagsItemUrls {
    /// The URL to view the tag
    #[serde(rename = "view")]
    pub r#view: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemTagsItem {
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
    pub r#urls: Option<ReturnsDiscussionsItemPostsUserpostsItemTagsItemUrls>,
}

/// tags
pub type r#ReturnsDiscussionsItemPostsUserpostsItemTags =
    Vec<ReturnsDiscussionsItemPostsUserpostsItemTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsUserpostsItemHtml {
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
pub struct ReturnsDiscussionsItemPostsUserpostsItem {
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
    pub r#author: Option<ReturnsDiscussionsItemPostsUserpostsItemAuthor>,
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
    pub r#capabilities: Option<ReturnsDiscussionsItemPostsUserpostsItemCapabilities>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsDiscussionsItemPostsUserpostsItemUrls>,
    /// attachments
    #[serde(rename = "attachments")]
    pub r#attachments: Option<r#ReturnsDiscussionsItemPostsUserpostsItemAttachments>,
    /// messageinlinefiles
    #[serde(rename = "messageinlinefiles")]
    pub r#messageinlinefiles: Option<r#ReturnsDiscussionsItemPostsUserpostsItemMessageinlinefiles>,
    /// tags
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsDiscussionsItemPostsUserpostsItemTags>,
    #[serde(rename = "html")]
    pub r#html: Option<ReturnsDiscussionsItemPostsUserpostsItemHtml>,
}

pub type r#ReturnsDiscussionsItemPostsUserposts = Vec<ReturnsDiscussionsItemPostsUserpostsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemAuthorGroupsItemUrls {
    /// image
    #[serde(rename = "image")]
    pub r#image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemAuthorGroupsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsDiscussionsItemPostsParentpostsItemAuthorGroupsItemUrls>,
}

/// groups
pub type r#ReturnsDiscussionsItemPostsParentpostsItemAuthorGroups =
    Vec<ReturnsDiscussionsItemPostsParentpostsItemAuthorGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemAuthorUrls {
    /// The URL for the use profile page
    #[serde(rename = "profile")]
    pub r#profile: Option<String>,
    /// The URL for the use profile image
    #[serde(rename = "profileimage")]
    pub r#profileimage: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemAuthor {
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
    pub r#groups: Option<r#ReturnsDiscussionsItemPostsParentpostsItemAuthorGroups>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsDiscussionsItemPostsParentpostsItemAuthorUrls>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemCapabilities {
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
pub struct ReturnsDiscussionsItemPostsParentpostsItemUrls {
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
pub struct ReturnsDiscussionsItemPostsParentpostsItemAttachmentsItemUrls {
    /// The URL used to export the attachment
    #[serde(rename = "export")]
    pub r#export: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemAttachmentsItemHtml {
    /// The HTML source for the Plagiarism Response
    #[serde(rename = "plagiarism")]
    pub r#plagiarism: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemAttachmentsItem {
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
    pub r#urls: Option<ReturnsDiscussionsItemPostsParentpostsItemAttachmentsItemUrls>,
    #[serde(rename = "html")]
    pub r#html: Option<ReturnsDiscussionsItemPostsParentpostsItemAttachmentsItemHtml>,
}

/// attachments
pub type r#ReturnsDiscussionsItemPostsParentpostsItemAttachments =
    Vec<ReturnsDiscussionsItemPostsParentpostsItemAttachmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemMessageinlinefilesItem {
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
pub type r#ReturnsDiscussionsItemPostsParentpostsItemMessageinlinefiles =
    Vec<ReturnsDiscussionsItemPostsParentpostsItemMessageinlinefilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemTagsItemUrls {
    /// The URL to view the tag
    #[serde(rename = "view")]
    pub r#view: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemTagsItem {
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
    pub r#urls: Option<ReturnsDiscussionsItemPostsParentpostsItemTagsItemUrls>,
}

/// tags
pub type r#ReturnsDiscussionsItemPostsParentpostsItemTags =
    Vec<ReturnsDiscussionsItemPostsParentpostsItemTagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPostsParentpostsItemHtml {
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
pub struct ReturnsDiscussionsItemPostsParentpostsItem {
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
    pub r#author: Option<ReturnsDiscussionsItemPostsParentpostsItemAuthor>,
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
    pub r#capabilities: Option<ReturnsDiscussionsItemPostsParentpostsItemCapabilities>,
    #[serde(rename = "urls")]
    pub r#urls: Option<ReturnsDiscussionsItemPostsParentpostsItemUrls>,
    /// attachments
    #[serde(rename = "attachments")]
    pub r#attachments: Option<r#ReturnsDiscussionsItemPostsParentpostsItemAttachments>,
    /// messageinlinefiles
    #[serde(rename = "messageinlinefiles")]
    pub r#messageinlinefiles:
        Option<r#ReturnsDiscussionsItemPostsParentpostsItemMessageinlinefiles>,
    /// tags
    #[serde(rename = "tags")]
    pub r#tags: Option<r#ReturnsDiscussionsItemPostsParentpostsItemTags>,
    #[serde(rename = "html")]
    pub r#html: Option<ReturnsDiscussionsItemPostsParentpostsItemHtml>,
}

pub type r#ReturnsDiscussionsItemPostsParentposts = Vec<ReturnsDiscussionsItemPostsParentpostsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemPosts {
    #[serde(rename = "userposts")]
    pub r#userposts: Option<r#ReturnsDiscussionsItemPostsUserposts>,
    #[serde(rename = "parentposts")]
    pub r#parentposts: Option<r#ReturnsDiscussionsItemPostsParentposts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItem {
    /// Name of the discussion
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// ID of the discussion
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Timestamp of the discussion start
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Full name of the user that started the discussion
    #[serde(rename = "authorfullname")]
    pub r#authorfullname: Option<String>,
    #[serde(rename = "posts")]
    pub r#posts: Option<ReturnsDiscussionsItemPosts>,
}

pub type r#ReturnsDiscussions = Vec<ReturnsDiscussionsItem>;

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
    #[serde(rename = "discussions")]
    pub r#discussions: Option<r#ReturnsDiscussions>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_forum_get_discussion_posts_by_userid", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_forum_get_discussion_posts_by_userid", params)
        .await
}
