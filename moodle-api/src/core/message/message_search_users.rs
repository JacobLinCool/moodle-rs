use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The id of the user who is performing the search
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The string being searched
    #[serde(rename = "search")]
    pub r#search: Option<String>,
    /// Limit from
    #[serde(rename = "limitfrom")]
    pub r#limitfrom: Option<i64>,
    /// Limit number
    #[serde(rename = "limitnum")]
    pub r#limitnum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsContactsItemContactrequestsItem {
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
pub type r#ReturnsContactsItemContactrequests = Vec<ReturnsContactsItemContactrequestsItem>;

/// information about conversation
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsContactsItemConversationsItem {
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
pub type r#ReturnsContactsItemConversations = Vec<ReturnsContactsItemConversationsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsContactsItem {
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
    pub r#contactrequests: Option<r#ReturnsContactsItemContactrequests>,
    /// Conversations between users
    #[serde(rename = "conversations")]
    pub r#conversations: Option<r#ReturnsContactsItemConversations>,
}

pub type r#ReturnsContacts = Vec<ReturnsContactsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsNoncontactsItemContactrequestsItem {
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
pub type r#ReturnsNoncontactsItemContactrequests = Vec<ReturnsNoncontactsItemContactrequestsItem>;

/// information about conversation
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsNoncontactsItemConversationsItem {
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
pub type r#ReturnsNoncontactsItemConversations = Vec<ReturnsNoncontactsItemConversationsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsNoncontactsItem {
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
    pub r#contactrequests: Option<r#ReturnsNoncontactsItemContactrequests>,
    /// Conversations between users
    #[serde(rename = "conversations")]
    pub r#conversations: Option<r#ReturnsNoncontactsItemConversations>,
}

pub type r#ReturnsNoncontacts = Vec<ReturnsNoncontactsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "contacts")]
    pub r#contacts: Option<r#ReturnsContacts>,
    #[serde(rename = "noncontacts")]
    pub r#noncontacts: Option<r#ReturnsNoncontacts>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_message_search_users", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_message_search_users", params)
        .await
}
