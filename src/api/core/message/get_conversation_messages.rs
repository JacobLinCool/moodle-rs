use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The current user's id
    #[serde(rename = "currentuserid")]
    pub r#currentuserid: Option<i64>,
    /// The conversation id
    #[serde(rename = "convid")]
    pub r#convid: Option<i64>,
    /// Limit from
    #[serde(rename = "limitfrom")]
    pub r#limitfrom: Option<i64>,
    /// Limit number
    #[serde(rename = "limitnum")]
    pub r#limitnum: Option<i64>,
    /// Newest first?
    #[serde(rename = "newest")]
    pub r#newest: Option<bool>,
    /// The timestamp from which the messages were created
    #[serde(rename = "timefrom")]
    pub r#timefrom: Option<i64>,
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
    pub r#contactrequests: ReturnsMembersItemContactrequests,
    /// Conversations between users
    #[serde(rename = "conversations")]
    pub r#conversations: ReturnsMembersItemConversations,
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
    #[serde(rename = "members")]
    pub r#members: ReturnsMembers,
    #[serde(rename = "messages")]
    pub r#messages: ReturnsMessages,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_get_conversation_messages", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
