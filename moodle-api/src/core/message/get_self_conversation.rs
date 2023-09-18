use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The id of the user who we are viewing self-conversations for
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Limit for number of messages
    #[serde(rename = "messagelimit")]
    pub r#messagelimit: Option<i64>,
    /// Offset for messages list
    #[serde(rename = "messageoffset")]
    pub r#messageoffset: Option<i64>,
    /// Order messages by newest first
    #[serde(rename = "newestmessagesfirst")]
    pub r#newestmessagesfirst: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMembersItemContactrequestsItem {
    /// The id of the contact request
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The id of the user who created the contact request
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The id of the user confirming the request
    #[serde(rename = "requesteduserid")]
    pub r#requesteduserid: Option<i64>,
    /// The timecreated timestamp for the contact request
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
}

/// The contact requests
pub type r#ReturnsMembersItemContactrequests = Vec<ReturnsMembersItemContactrequestsItem>;

/// information about conversation
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMembersItemConversationsItem {
    /// Conversations id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Conversation type: private or public
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// Multilang compatible conversation name2
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The timecreated timestamp for the conversation
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
}

/// Conversations between users
pub type r#ReturnsMembersItemConversations = Vec<ReturnsMembersItemConversationsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMembersItem {
    /// The user id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The user's name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// The link to the user's profile page
    #[serde(rename = "profileurl")]
    pub r#profileurl: Option<String>,
    /// User picture URL
    #[serde(rename = "profileimageurl")]
    pub r#profileimageurl: Option<String>,
    /// Small user picture URL
    #[serde(rename = "profileimageurlsmall")]
    pub r#profileimageurlsmall: Option<String>,
    /// The user's online status
    #[serde(rename = "isonline")]
    pub r#isonline: Option<bool>,
    /// Show the user's online status?
    #[serde(rename = "showonlinestatus")]
    pub r#showonlinestatus: Option<bool>,
    /// If the user has been blocked
    #[serde(rename = "isblocked")]
    pub r#isblocked: Option<bool>,
    /// Is the user a contact?
    #[serde(rename = "iscontact")]
    pub r#iscontact: Option<bool>,
    /// Is the user deleted?
    #[serde(rename = "isdeleted")]
    pub r#isdeleted: Option<bool>,
    /// If the user can still message even if they get blocked
    #[serde(rename = "canmessageevenifblocked")]
    pub r#canmessageevenifblocked: Option<bool>,
    /// If the user can be messaged
    #[serde(rename = "canmessage")]
    pub r#canmessage: Option<bool>,
    /// If the user requires to be contacts
    #[serde(rename = "requirescontact")]
    pub r#requirescontact: Option<bool>,
    /// The contact requests
    #[serde(rename = "contactrequests")]
    pub r#contactrequests: Option<r#ReturnsMembersItemContactrequests>,
    /// Conversations between users
    #[serde(rename = "conversations")]
    pub r#conversations: Option<r#ReturnsMembersItemConversations>,
}

pub type r#ReturnsMembers = Vec<ReturnsMembersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMessagesItem {
    /// The id of the message
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The id of the user who sent the message
    #[serde(rename = "useridfrom")]
    pub r#useridfrom: Option<i64>,
    /// The text of the message
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// The timecreated timestamp for the message
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
}

pub type r#ReturnsMessages = Vec<ReturnsMessagesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// The conversation id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The conversation name, if set
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// A subtitle for the conversation name, if set
    #[serde(rename = "subname")]
    pub r#subname: Option<String>,
    /// A link to the conversation picture, if set
    #[serde(rename = "imageurl")]
    pub r#imageurl: Option<String>,
    /// The type of the conversation (1=individual,2=group,3=self)
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// Total number of conversation members
    #[serde(rename = "membercount")]
    pub r#membercount: Option<i64>,
    /// If the user muted this conversation
    #[serde(rename = "ismuted")]
    pub r#ismuted: Option<bool>,
    /// If the user marked this conversation as a favourite
    #[serde(rename = "isfavourite")]
    pub r#isfavourite: Option<bool>,
    /// If the user has read all messages in the conversation
    #[serde(rename = "isread")]
    pub r#isread: Option<bool>,
    /// The number of unread messages in this conversation
    #[serde(rename = "unreadcount")]
    pub r#unreadcount: Option<i64>,
    #[serde(rename = "members")]
    pub r#members: Option<r#ReturnsMembers>,
    #[serde(rename = "messages")]
    pub r#messages: Option<r#ReturnsMessages>,
    /// If the user can delete messages in the conversation for all users
    #[serde(rename = "candeletemessagesforallusers")]
    pub r#candeletemessagesforallusers: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_get_self_conversation", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_get_self_conversation", params)
        .await
}
