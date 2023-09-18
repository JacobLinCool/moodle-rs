use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// forum instance id
    #[serde(rename = "forumid")]
    pub r#forumid: Option<i64>,
    /// sort by this element: numreplies, , created or timemodified
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// current page
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// items per page
    #[serde(rename = "perpage")]
    pub r#perpage: Option<i64>,
    /// group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemMessageinlinefilesItem {
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

/// post message inline files
pub type r#ReturnsDiscussionsItemMessageinlinefiles =
    Vec<ReturnsDiscussionsItemMessageinlinefilesItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItemAttachmentsItem {
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
pub type r#ReturnsDiscussionsItemAttachments = Vec<ReturnsDiscussionsItemAttachmentsItem>;

/// post
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDiscussionsItem {
    /// Post id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Discussion name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// The id of the user who last modified
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// Time discussion can start
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// Time discussion ends
    #[serde(rename = "timeend")]
    pub r#timeend: Option<i64>,
    /// Discussion id
    #[serde(rename = "discussion")]
    pub r#discussion: Option<i64>,
    /// Parent id
    #[serde(rename = "parent")]
    pub r#parent: Option<i64>,
    /// User who started the discussion id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Creation time
    #[serde(rename = "created")]
    pub r#created: Option<i64>,
    /// Time modified
    #[serde(rename = "modified")]
    pub r#modified: Option<i64>,
    /// Mailed?
    #[serde(rename = "mailed")]
    pub r#mailed: Option<i64>,
    /// The post subject
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// The post message
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// message format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "messageformat")]
    pub r#messageformat: Option<i64>,
    /// Can we trust?
    #[serde(rename = "messagetrust")]
    pub r#messagetrust: Option<i64>,
    /// post message inline files
    #[serde(rename = "messageinlinefiles")]
    pub r#messageinlinefiles: Option<r#ReturnsDiscussionsItemMessageinlinefiles>,
    /// Has attachments?
    #[serde(rename = "attachment")]
    pub r#attachment: Option<String>,
    /// attachments
    #[serde(rename = "attachments")]
    pub r#attachments: Option<r#ReturnsDiscussionsItemAttachments>,
    /// The post message total score
    #[serde(rename = "totalscore")]
    pub r#totalscore: Option<i64>,
    /// Mail now?
    #[serde(rename = "mailnow")]
    pub r#mailnow: Option<i64>,
    /// Post author full name
    #[serde(rename = "userfullname")]
    pub r#userfullname: Option<String>,
    /// Post modifier full name
    #[serde(rename = "usermodifiedfullname")]
    pub r#usermodifiedfullname: Option<String>,
    /// Post author picture.
    #[serde(rename = "userpictureurl")]
    pub r#userpictureurl: Option<String>,
    /// Post modifier picture.
    #[serde(rename = "usermodifiedpictureurl")]
    pub r#usermodifiedpictureurl: Option<String>,
    /// The number of replies in the discussion
    #[serde(rename = "numreplies")]
    pub r#numreplies: Option<i64>,
    /// The number of unread discussions.
    #[serde(rename = "numunread")]
    pub r#numunread: Option<i64>,
    /// Is the discussion pinned
    #[serde(rename = "pinned")]
    pub r#pinned: Option<bool>,
    /// Is the discussion locked
    #[serde(rename = "locked")]
    pub r#locked: Option<bool>,
    /// Is the discussion starred
    #[serde(rename = "starred")]
    pub r#starred: Option<bool>,
    /// Can the user reply to the discussion
    #[serde(rename = "canreply")]
    pub r#canreply: Option<bool>,
    /// Can the user lock the discussion
    #[serde(rename = "canlock")]
    pub r#canlock: Option<bool>,
    /// Can the user star the discussion
    #[serde(rename = "canfavourite")]
    pub r#canfavourite: Option<bool>,
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
        .post("mod_forum_get_forum_discussions", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_forum_get_forum_discussions", params).await
}
